use super::*;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use parking_lot::Mutex;
use serde_json::json;
use serial_test::serial;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

#[derive(Clone, Default)]
struct TestState {
    responses: Arc<Mutex<HashMap<String, VecDeque<MockResponse>>>>,
    requests: Arc<Mutex<Vec<RecordedRequest>>>,
}

#[derive(Debug, Clone)]
struct MockResponse {
    status: StatusCode,
    body: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RecordedRequest {
    model: String,
    authorization: String,
}

#[tokio::test]
async fn rotates_keys_and_falls_back_across_models() {
    let state = TestState::default();
    state.responses.lock().insert(
        "m1".to_owned(),
        VecDeque::from([MockResponse {
            status: StatusCode::TOO_MANY_REQUESTS,
            body: json!({ "error": { "message": "Please retry in 12.5s." } }),
        }]),
    );
    state.responses.lock().insert(
        "m2".to_owned(),
        VecDeque::from([MockResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: json!({ "error": { "message": "backend down" } }),
        }]),
    );
    state.responses.lock().insert(
        "m3".to_owned(),
        VecDeque::from([MockResponse {
            status: StatusCode::OK,
            body: json!({ "choices": [{ "message": { "content": "{\"tags\":[\"warm\"]}" } }] }),
        }]),
    );

    let server = spawn_test_server(state.clone()).await;
    let client = GroqClient::with_model_chain(
        vec![
            "gsk_key_1".to_owned(),
            "gsk_key_2".to_owned(),
            "gsk_key_3".to_owned(),
        ],
        vec!["m1".to_owned(), "m2".to_owned(), "m3".to_owned()],
    )
    .expect("client should build")
    .with_endpoint(server)
    .with_max_attempts_per_model(1);

    let response = client
        .chat_completion(&GroqChatRequest::new("clasifica este sample"))
        .await
        .expect("third model should succeed");

    let requests = state.requests.lock().clone();
    assert_eq!(response.model, "m3");
    assert_eq!(response.key_index, 2);
    assert_eq!(response.attempt_count, 3);
    assert_eq!(response.content, "{\"tags\":[\"warm\"]}");
    assert_eq!(
        requests,
        vec![
            RecordedRequest {
                model: "m1".to_owned(),
                authorization: "Bearer gsk_key_1".to_owned(),
            },
            RecordedRequest {
                model: "m2".to_owned(),
                authorization: "Bearer gsk_key_2".to_owned(),
            },
            RecordedRequest {
                model: "m3".to_owned(),
                authorization: "Bearer gsk_key_3".to_owned(),
            },
        ]
    );
}

#[tokio::test]
async fn retries_same_model_before_switching() {
    let state = TestState::default();
    state.responses.lock().insert(
        "m1".to_owned(),
        VecDeque::from([
            MockResponse {
                status: StatusCode::TOO_MANY_REQUESTS,
                body: json!({ "error": { "message": "Please retry in 1.0s." } }),
            },
            MockResponse {
                status: StatusCode::TOO_MANY_REQUESTS,
                body: json!({ "error": { "message": "Please retry in 1.0s." } }),
            },
            MockResponse {
                status: StatusCode::OK,
                body: json!({ "choices": [{ "message": { "content": "{\"genre\":\"house\"}" } }] }),
            },
        ]),
    );

    let server = spawn_test_server(state.clone()).await;
    let client = GroqClient::with_model_chain(
        vec![
            "gsk_key_1".to_owned(),
            "gsk_key_2".to_owned(),
            "gsk_key_3".to_owned(),
        ],
        vec!["m1".to_owned(), "m2".to_owned()],
    )
    .expect("client should build")
    .with_endpoint(server)
    .with_max_attempts_per_model(3);

    let response = client
        .chat_completion(&GroqChatRequest::new("clasifica este sample"))
        .await
        .expect("third retry on same model should succeed");

    let requests = state.requests.lock().clone();
    assert_eq!(response.model, "m1");
    assert_eq!(response.key_index, 2);
    assert_eq!(response.attempt_count, 3);
    assert_eq!(requests.len(), 3);
    assert!(requests.iter().all(|request| request.model == "m1"));
}

#[test]
#[serial]
fn loads_numbered_keys_before_legacy_key() {
    let previous = [
        ("GROQ_API", std::env::var("GROQ_API").ok()),
        ("GROQ_API_1", std::env::var("GROQ_API_1").ok()),
        ("GROQ_API_2", std::env::var("GROQ_API_2").ok()),
        ("GROQ_API_3", std::env::var("GROQ_API_3").ok()),
    ];

    std::env::set_var("GROQ_API", "gsk_legacy");
    std::env::set_var("GROQ_API_1", "gsk_one");
    std::env::set_var("GROQ_API_2", "gsk_two");
    std::env::set_var("GROQ_API_3", "gsk_three");

    let keys = load_groq_api_keys();

    restore_env_vars(&previous);

    assert_eq!(keys, vec!["gsk_one", "gsk_two", "gsk_three"]);
}

#[test]
fn extracts_retry_after_from_groq_message() {
    let body = r#"{"error":{"message":"Rate limit reached. Please retry in 23.71s."}}"#;
    assert_eq!(extract_retry_after_seconds(body), Some(23.71));
}

async fn spawn_test_server(state: TestState) -> String {
    let app = Router::new()
        .route("/", post(chat_handler))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("should bind test listener");
    let address = listener
        .local_addr()
        .expect("listener should expose local addr");
    tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("test server should serve");
    });

    format!("http://{address}/")
}

async fn chat_handler(
    State(state): State<TestState>,
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let model = payload
        .get("model")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default()
        .to_owned();
    let authorization = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_owned();
    state.requests.lock().push(RecordedRequest {
        model: model.clone(),
        authorization,
    });

    let response = state
        .responses
        .lock()
        .get_mut(&model)
        .and_then(VecDeque::pop_front)
        .unwrap_or(MockResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: json!({ "error": { "message": "missing mock response" } }),
        });

    (response.status, Json(response.body))
}

fn restore_env_vars(previous: &[(&str, Option<String>)]) {
    for (name, value) in previous {
        if let Some(value) = value {
            std::env::set_var(name, value);
        } else {
            std::env::remove_var(name);
        }
    }
}

use super::*;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use parking_lot::Mutex;
use serde_json::json;
use serial_test::serial;
use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Clone, Default)]
struct TestState {
    responses: Arc<Mutex<VecDeque<MockResponse>>>,
    requests: Arc<Mutex<Vec<RecordedRequest>>>,
}

#[derive(Debug, Clone)]
struct MockResponse {
    status: reqwest::StatusCode,
    retry_after_header: Option<&'static str>,
    body: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RecordedRequest {
    authorization: String,
    used_json_response_format: bool,
}

#[tokio::test]
async fn returns_content_on_success() {
    let state = TestState::default();
    state.responses.lock().push_back(MockResponse {
        status: reqwest::StatusCode::OK,
        retry_after_header: None,
        body: json!({ "choices": [{ "message": { "content": "{\"mood\":\"uplifting\"}" } }] }),
    });

    let server = spawn_test_server(state.clone()).await;
    let client = OpenAiClient::new("sk-openai-key".to_owned())
        .expect("client should build")
        .with_endpoint(server);

    let response = client
        .chat_completion(&OpenAiChatRequest::new("clasifica este sample"))
        .await
        .expect("request should succeed");

    assert_eq!(response.model, "gpt-4o-mini");
    assert_eq!(response.attempt_count, 1);
    assert_eq!(response.content, "{\"mood\":\"uplifting\"}");
    assert_eq!(
        state.requests.lock().clone(),
        vec![RecordedRequest {
            authorization: "Bearer sk-openai-key".to_owned(),
            used_json_response_format: true,
        }]
    );
}

#[tokio::test]
async fn retries_without_response_format_after_json_schema_rejection() {
    let state = TestState::default();
    state.responses.lock().extend([
        MockResponse {
            status: reqwest::StatusCode::BAD_REQUEST,
            retry_after_header: None,
            body: json!({ "error": { "message": "json_validate_failed: schema mismatch" } }),
        },
        MockResponse {
            status: reqwest::StatusCode::OK,
            retry_after_header: None,
            body: json!({ "choices": [{ "message": { "content": "{\"genre\":\"house\"}" } }] }),
        },
    ]);

    let server = spawn_test_server(state.clone()).await;
    let client = OpenAiClient::new("sk-openai-key".to_owned())
        .expect("client should build")
        .with_endpoint(server);

    let response = client
        .chat_completion(&OpenAiChatRequest::new("clasifica este sample"))
        .await
        .expect("second attempt without response_format should succeed");

    let requests = state.requests.lock().clone();
    assert_eq!(response.attempt_count, 2);
    assert_eq!(requests.len(), 2);
    assert!(requests[0].used_json_response_format);
    assert!(!requests[1].used_json_response_format);
}

#[test]
#[serial]
fn loads_api_key_from_env() {
    let previous = std::env::var("OPENAI_API_KEY").ok();
    std::env::set_var("OPENAI_API_KEY", "sk-test");

    let key = load_openai_api_key();

    if let Some(value) = previous {
        std::env::set_var("OPENAI_API_KEY", value);
    } else {
        std::env::remove_var("OPENAI_API_KEY");
    }

    assert_eq!(key.as_deref(), Some("sk-test"));
}

#[tokio::test]
async fn preserves_retry_after_on_http_errors() {
    let state = TestState::default();
    state.responses.lock().push_back(MockResponse {
        status: reqwest::StatusCode::TOO_MANY_REQUESTS,
        retry_after_header: Some("7"),
        body: json!({ "error": { "message": "rate limited" } }),
    });

    let server = spawn_test_server(state).await;
    let client = OpenAiClient::new("sk-openai-key".to_owned())
        .expect("client should build")
        .with_endpoint(server);

    let error = client
        .chat_completion(&OpenAiChatRequest::new("clasifica este sample"))
        .await
        .expect_err("429 should fail");

    let mut observed = None;
    if let OpenAiClientError::Request {
        status_code,
        retry_after_seconds,
        retryable,
        ..
    } = error
    {
        observed = Some((status_code, retry_after_seconds, retryable));
    }

    assert_eq!(observed, Some((Some(429), Some(7.0), true)));
}

async fn spawn_test_server(state: TestState) -> String {
    let app = Router::new().route("/", post(chat_handler)).with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("should bind test listener");
    let address = listener
        .local_addr()
        .expect("listener should expose local addr");
    tokio::spawn(async move {
        axum::serve(listener, app).await.expect("test server should serve");
    });

    format!("http://{address}/")
}

async fn chat_handler(
    State(state): State<TestState>,
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let used_json_response_format = payload.get("response_format").is_some();
    let authorization = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_owned();
    state.requests.lock().push(RecordedRequest {
        authorization,
        used_json_response_format,
    });

    let response = state.responses.lock().pop_front().unwrap_or(MockResponse {
        status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        retry_after_header: None,
        body: json!({ "error": { "message": "missing mock response" } }),
    });

    let mut response_headers = HeaderMap::new();
    if let Some(retry_after) = response.retry_after_header {
        response_headers.insert(
            reqwest::header::RETRY_AFTER,
            retry_after.parse().expect("retry-after header should parse"),
        );
    }

    (response.status, response_headers, Json(response.body)).into_response()
}
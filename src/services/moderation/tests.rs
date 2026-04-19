use super::*;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use parking_lot::Mutex;
use serde_json::json;
use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Clone, Default)]
struct MockState {
    responses: Arc<Mutex<VecDeque<MockResponse>>>,
}

#[derive(Debug, Clone)]
struct MockResponse {
    status: StatusCode,
    retry_after_header: Option<&'static str>,
    body: serde_json::Value,
}

#[tokio::test]
async fn local_prefilter_rejects_promotional_spam_without_ai() {
    let service = ModerationService::new(None, None);
    let request = ModerationRequest {
        body: "Buy now at https://spam.test and https://promo.test for free money".to_owned(),
        ..build_request()
    };

    let result = service.moderate(&request).await;

    assert_eq!(result.decision.verdict, ModerationVerdict::Rejected);
    assert_eq!(result.decision.category, ModerationCategory::Spam);
    assert!(result.ai_assessment.is_none());
    assert_eq!(result.recommended_sample_state(), "en_supervision");
}

#[tokio::test]
async fn groq_success_produces_ai_assessment() {
    let groq_state = MockState::default();
    groq_state.responses.lock().push_back(MockResponse {
        status: StatusCode::OK,
        retry_after_header: None,
        body: json!({
            "choices": [{
                "message": {
                    "content": "{\"safe\":true,\"category\":\"safe\",\"confidence\":0.98,\"recommended_level\":\"aprobado\",\"reason_code\":\"sin_hallazgos\",\"summary\":\"Metadata segura\"}"
                }
            }]
        }),
    });

    let groq =
        GroqClient::with_model_chain(vec!["gsk_test".to_owned()], vec!["groq-mod".to_owned()])
            .expect("groq client should build")
            .with_endpoint(spawn_test_server(groq_state).await)
            .with_max_attempts_per_model(1);
    let service = ModerationService::new(Some(groq), None);

    let result = service.moderate(&build_request()).await;

    assert_eq!(result.decision.verdict, ModerationVerdict::Approved);
    assert_eq!(
        result
            .ai_assessment
            .as_ref()
            .map(|assessment| assessment.provider),
        Some(ModerationProvider::Groq)
    );
    assert!(result
        .admin_panel
        .badges
        .iter()
        .any(|badge| badge == "Groq"));
}

#[tokio::test]
async fn falls_back_to_openai_after_groq_parse_failure() {
    let groq_state = MockState::default();
    groq_state.responses.lock().push_back(MockResponse {
        status: StatusCode::OK,
        retry_after_header: None,
        body: json!({
            "choices": [{ "message": { "content": "sin json util" } }]
        }),
    });

    let openai_state = MockState::default();
    openai_state.responses.lock().push_back(MockResponse {
        status: StatusCode::OK,
        retry_after_header: None,
        body: json!({
            "choices": [{
                "message": {
                    "content": "{\"safe\":false,\"category\":\"copyright\",\"confidence\":0.62,\"recommended_level\":\"revision\",\"reason_code\":\"copyright_risk\",\"summary\":\"Possible copyright claim\"}"
                }
            }]
        }),
    });

    let groq =
        GroqClient::with_model_chain(vec!["gsk_test".to_owned()], vec!["groq-mod".to_owned()])
            .expect("groq client should build")
            .with_endpoint(spawn_test_server(groq_state).await)
            .with_max_attempts_per_model(1);
    let openai = OpenAiClient::new("sk-openai".to_owned())
        .expect("openai client should build")
        .with_endpoint(spawn_test_server(openai_state).await);
    let service = ModerationService::new(Some(groq), Some(openai));

    let result = service.moderate(&build_request()).await;

    assert_eq!(result.decision.verdict, ModerationVerdict::Review);
    assert_eq!(
        result
            .ai_assessment
            .as_ref()
            .map(|assessment| assessment.provider),
        Some(ModerationProvider::OpenAi)
    );
    assert!(result
        .provider_failures
        .iter()
        .any(|failure| matches!(failure, ModerationProviderFailure::Parse(_))));
}

#[tokio::test]
async fn provider_failures_force_review_and_preserve_retry_after() {
    let groq_state = MockState::default();
    groq_state.responses.lock().push_back(MockResponse {
        status: StatusCode::TOO_MANY_REQUESTS,
        retry_after_header: None,
        body: json!({ "error": { "message": "Please retry in 12.0s." } }),
    });

    let openai_state = MockState::default();
    openai_state.responses.lock().push_back(MockResponse {
        status: StatusCode::SERVICE_UNAVAILABLE,
        retry_after_header: Some("7"),
        body: json!({ "error": { "message": "upstream unavailable" } }),
    });

    let groq =
        GroqClient::with_model_chain(vec!["gsk_test".to_owned()], vec!["groq-mod".to_owned()])
            .expect("groq client should build")
            .with_endpoint(spawn_test_server(groq_state).await)
            .with_max_attempts_per_model(1);
    let openai = OpenAiClient::new("sk-openai".to_owned())
        .expect("openai client should build")
        .with_endpoint(spawn_test_server(openai_state).await);
    let service = ModerationService::new(Some(groq), Some(openai));

    let result = service.moderate(&build_request()).await;

    assert_eq!(result.decision.verdict, ModerationVerdict::Review);
    assert!(result.is_retryable());
    assert_eq!(result.retry_after_seconds(), Some(12.0));
}

fn build_request() -> ModerationRequest {
    ModerationRequest {
        entity_kind: ModerationEntityKind::Sample,
        entity_id: Some(42),
        title: "Dusty House Loop".to_owned(),
        body: "Warm analog house drums with synth texture".to_owned(),
        tags: vec!["house".to_owned(), "analog".to_owned(), "drums".to_owned()],
        primary_folder: Some("Loops".to_owned()),
        secondary_folder: Some("House".to_owned()),
        image_urls: Vec::new(),
        extra_context: Some("uploaded from curated pack".to_owned()),
    }
}

async fn spawn_test_server(state: MockState) -> String {
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
    State(state): State<MockState>,
    _headers: HeaderMap,
    Json(_payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let response = state.responses.lock().pop_front().unwrap_or(MockResponse {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        retry_after_header: None,
        body: json!({ "error": { "message": "missing mock response" } }),
    });

    let mut response_headers = HeaderMap::new();
    if let Some(retry_after) = response.retry_after_header {
        response_headers.insert(
            reqwest::header::RETRY_AFTER,
            retry_after
                .parse()
                .expect("retry-after header should parse"),
        );
    }

    (response.status, response_headers, Json(response.body)).into_response()
}

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::audio::ia::prompts::AUDIO_CLASSIFICATION_SYSTEM_PROMPT;

const OPENAI_CHAT_COMPLETIONS_URL: &str = "https://api.openai.com/v1/chat/completions";
const DEFAULT_OPENAI_MODEL: &str = "gpt-4o-mini";

#[derive(Clone)]
pub struct OpenAiClient {
    http: reqwest::Client,
    endpoint: String,
    api_key: String,
    default_model: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenAiChatRequest {
    pub user_prompt: String,
    pub system_prompt: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub require_json_object: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenAiChatSuccess {
    pub model: String,
    pub content: String,
    pub attempt_count: usize,
}

#[derive(Debug, Error)]
pub enum OpenAiClientError {
    #[error("No hay API key configurada para OpenAI")]
    MissingApiKey,
    #[error("OpenAI rechazo response_format JSON: {message}")]
    JsonSchemaRejected { status_code: u16, message: String },
    #[error("OpenAI fallo: {message}")]
    Request {
        status_code: Option<u16>,
        retry_after_seconds: Option<f32>,
        retryable: bool,
        message: String,
    },
}

impl OpenAiChatRequest {
    #[must_use]
    pub fn new(user_prompt: impl Into<String>) -> Self {
        Self {
            user_prompt: user_prompt.into(),
            system_prompt: AUDIO_CLASSIFICATION_SYSTEM_PROMPT.to_owned(),
            temperature: 0.2,
            max_tokens: 1_500,
            require_json_object: true,
        }
    }
}

impl OpenAiClient {
    pub fn from_env() -> Result<Self, OpenAiClientError> {
        Self::new(load_openai_api_key().ok_or(OpenAiClientError::MissingApiKey)?)
    }

    pub fn new(api_key: String) -> Result<Self, OpenAiClientError> {
        if api_key.trim().is_empty() {
            return Err(OpenAiClientError::MissingApiKey);
        }

        Ok(Self {
            http: reqwest::Client::builder()
                .connect_timeout(std::time::Duration::from_secs(8))
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .map_err(|error| OpenAiClientError::Request {
                    status_code: None,
                    retry_after_seconds: None,
                    retryable: false,
                    message: error.to_string(),
                })?,
            endpoint: OPENAI_CHAT_COMPLETIONS_URL.to_owned(),
            api_key,
            default_model: DEFAULT_OPENAI_MODEL.to_owned(),
        })
    }

    #[must_use]
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }

    #[must_use]
    pub fn with_default_model(mut self, model: impl Into<String>) -> Self {
        self.default_model = model.into();
        self
    }

    pub async fn chat_completion(
        &self,
        request: &OpenAiChatRequest,
    ) -> Result<OpenAiChatSuccess, OpenAiClientError> {
        match self.execute_chat_completion(request).await {
            Ok(content) => Ok(OpenAiChatSuccess {
                model: self.default_model.clone(),
                content,
                attempt_count: 1,
            }),
            Err(OpenAiClientError::JsonSchemaRejected { .. }) if request.require_json_object => {
                let mut fallback_request = request.clone();
                fallback_request.require_json_object = false;
                let content = self.execute_chat_completion(&fallback_request).await?;
                Ok(OpenAiChatSuccess {
                    model: self.default_model.clone(),
                    content,
                    attempt_count: 2,
                })
            }
            Err(error) => Err(error),
        }
    }

    async fn execute_chat_completion(
        &self,
        request: &OpenAiChatRequest,
    ) -> Result<String, OpenAiClientError> {
        let payload = OpenAiChatPayload {
            model: &self.default_model,
            messages: [
                OpenAiMessagePayload {
                    role: "system",
                    content: &request.system_prompt,
                },
                OpenAiMessagePayload {
                    role: "user",
                    content: &request.user_prompt,
                },
            ],
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            response_format: request.require_json_object.then_some(OpenAiResponseFormat {
                kind: "json_object",
            }),
        };

        let response = self
            .http
            .post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|error| OpenAiClientError::Request {
                status_code: None,
                retry_after_seconds: None,
                retryable: error.is_connect() || error.is_timeout(),
                message: error.to_string(),
            })?;

        let status = response.status();
        let retry_after = response
            .headers()
            .get(reqwest::header::RETRY_AFTER)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<f32>().ok());
        let body = response
            .text()
            .await
            .map_err(|error| OpenAiClientError::Request {
                status_code: Some(status.as_u16()),
                retry_after_seconds: retry_after,
                retryable: status.is_server_error(),
                message: error.to_string(),
            })?;

        if !status.is_success() {
            return Err(build_openai_error(status, retry_after, &body));
        }

        let parsed: OpenAiChatApiResponse =
            serde_json::from_str(&body).map_err(|error| OpenAiClientError::Request {
                status_code: Some(status.as_u16()),
                retry_after_seconds: retry_after,
                retryable: false,
                message: error.to_string(),
            })?;
        parsed
            .choices
            .into_iter()
            .find_map(|choice| choice.message.content)
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
            .ok_or_else(|| OpenAiClientError::Request {
                status_code: Some(status.as_u16()),
                retry_after_seconds: retry_after,
                retryable: false,
                message: "respuesta sin contenido util".to_owned(),
            })
    }
}

#[must_use]
pub fn load_openai_api_key() -> Option<String> {
    std::env::var("OPENAI_API_KEY")
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn build_openai_error(
    status: StatusCode,
    retry_after_seconds: Option<f32>,
    body: &str,
) -> OpenAiClientError {
    let message = extract_error_message(body).unwrap_or_else(|| truncate(body, 500));
    if status == StatusCode::BAD_REQUEST && message.contains("json_validate_failed") {
        return OpenAiClientError::JsonSchemaRejected {
            status_code: status.as_u16(),
            message,
        };
    }

    OpenAiClientError::Request {
        status_code: Some(status.as_u16()),
        retry_after_seconds,
        retryable: status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error(),
        message,
    }
}

fn extract_error_message(body: &str) -> Option<String> {
    let parsed = serde_json::from_str::<serde_json::Value>(body).ok()?;
    parsed
        .get("error")
        .and_then(|value| value.get("message"))
        .and_then(serde_json::Value::as_str)
        .map(ToOwned::to_owned)
}

fn truncate(value: &str, max_chars: usize) -> String {
    value.chars().take(max_chars).collect()
}

#[derive(Debug, Serialize)]
struct OpenAiChatPayload<'a> {
    model: &'a str,
    messages: [OpenAiMessagePayload<'a>; 2],
    temperature: f32,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<OpenAiResponseFormat<'a>>,
}

#[derive(Debug, Serialize)]
struct OpenAiMessagePayload<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Debug, Serialize)]
struct OpenAiResponseFormat<'a> {
    #[serde(rename = "type")]
    kind: &'a str,
}

#[derive(Debug, Deserialize)]
struct OpenAiChatApiResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: OpenAiResponseMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAiResponseMessage {
    content: Option<String>,
}

#[cfg(test)]
#[path = "openai/tests.rs"]
mod tests;

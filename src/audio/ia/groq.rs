use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use thiserror::Error;

use crate::audio::ia::prompts::AUDIO_CLASSIFICATION_SYSTEM_PROMPT;

const GROQ_CHAT_COMPLETIONS_URL: &str = "https://api.groq.com/openai/v1/chat/completions";
const DEFAULT_MAX_ATTEMPTS_PER_MODEL: usize = 3;

pub const DEFAULT_GROQ_MODEL_CHAIN: [&str; 8] = [
    "openai/gpt-oss-120b",
    "moonshotai/kimi-k2-instruct-0905",
    "moonshotai/kimi-k2-instruct",
    "llama-3.3-70b-versatile",
    "qwen/qwen3-32b",
    "meta-llama/llama-4-scout-17b-16e-instruct",
    "openai/gpt-oss-20b",
    "groq/compound",
];

#[derive(Clone)]
pub struct GroqClient {
    http: reqwest::Client,
    endpoint: String,
    api_keys: Arc<[String]>,
    model_chain: Arc<[String]>,
    max_attempts_per_model: usize,
    next_key_index: Arc<AtomicUsize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroqChatRequest {
    pub user_prompt: String,
    pub system_prompt: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub require_json_object: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroqChatSuccess {
    pub model: String,
    pub content: String,
    pub key_index: usize,
    pub attempt_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroqAttemptFailure {
    pub model: String,
    pub key_index: usize,
    pub status_code: Option<u16>,
    pub retry_after_seconds: Option<f32>,
    pub retryable: bool,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum GroqClientError {
    #[error("No hay API keys configuradas para Groq")]
    MissingApiKeys,
    #[error("La cadena de modelos Groq esta vacia")]
    MissingModels,
    #[error("Groq agoto todos los intentos disponibles")]
    Exhausted { failures: Vec<GroqAttemptFailure> },
}

impl GroqClientError {
    #[must_use]
    pub fn failures(&self) -> &[GroqAttemptFailure] {
        match self {
            Self::Exhausted { failures } => failures,
            Self::MissingApiKeys | Self::MissingModels => &[],
        }
    }
}

impl GroqChatRequest {
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

impl GroqClient {
    pub fn from_env() -> Result<Self, GroqClientError> {
        Self::new(load_groq_api_keys())
    }

    pub fn from_env_with_model_chain(
        model_chain: Vec<String>,
    ) -> Result<Self, GroqClientError> {
        Self::with_model_chain(load_groq_api_keys(), model_chain)
    }

    pub fn new(api_keys: Vec<String>) -> Result<Self, GroqClientError> {
        if api_keys.is_empty() {
            return Err(GroqClientError::MissingApiKeys);
        }

        Self::with_model_chain(api_keys, DEFAULT_GROQ_MODEL_CHAIN.iter().map(ToString::to_string).collect())
    }

    pub fn with_model_chain(
        api_keys: Vec<String>,
        model_chain: Vec<String>,
    ) -> Result<Self, GroqClientError> {
        if api_keys.is_empty() {
            return Err(GroqClientError::MissingApiKeys);
        }
        if model_chain.is_empty() {
            return Err(GroqClientError::MissingModels);
        }

        Ok(Self {
            http: reqwest::Client::builder()
                .connect_timeout(std::time::Duration::from_secs(8))
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .map_err(|error| GroqClientError::Exhausted {
                    failures: vec![GroqAttemptFailure {
                        model: "client_init".to_owned(),
                        key_index: 0,
                        status_code: None,
                        retry_after_seconds: None,
                        retryable: false,
                        message: error.to_string(),
                    }],
                })?,
            endpoint: GROQ_CHAT_COMPLETIONS_URL.to_owned(),
            api_keys: api_keys.into(),
            model_chain: model_chain.into(),
            max_attempts_per_model: DEFAULT_MAX_ATTEMPTS_PER_MODEL,
            next_key_index: Arc::new(AtomicUsize::new(0)),
        })
    }

    #[must_use]
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }

    #[must_use]
    pub fn with_max_attempts_per_model(mut self, attempts: usize) -> Self {
        self.max_attempts_per_model = attempts.max(1);
        self
    }

    pub async fn chat_completion(
        &self,
        request: &GroqChatRequest,
    ) -> Result<GroqChatSuccess, GroqClientError> {
        if self.api_keys.is_empty() {
            return Err(GroqClientError::MissingApiKeys);
        }
        if self.model_chain.is_empty() {
            return Err(GroqClientError::MissingModels);
        }

        let mut failures = Vec::new();
        let mut attempt_count = 0_usize;

        for model in self.model_chain.iter() {
            for attempt_in_model in 0..self.max_attempts_per_model {
                attempt_count += 1;
                let (key_index, api_key) = self.next_api_key();
                match self
                    .execute_chat_completion(model, key_index, &api_key, request)
                    .await
                {
                    Ok(content) => {
                        return Ok(GroqChatSuccess {
                            model: model.clone(),
                            content,
                            key_index,
                            attempt_count,
                        });
                    }
                    Err(failure) => {
                        let retryable = failure.retryable;
                        failures.push(failure);
                        if !retryable || attempt_in_model + 1 >= self.max_attempts_per_model {
                            break;
                        }
                    }
                }
            }
        }

        Err(GroqClientError::Exhausted { failures })
    }

    fn next_api_key(&self) -> (usize, String) {
        let key_index = self.next_key_index.fetch_add(1, Ordering::Relaxed) % self.api_keys.len();
        (key_index, self.api_keys[key_index].clone())
    }

    async fn execute_chat_completion(
        &self,
        model: &str,
        key_index: usize,
        api_key: &str,
        request: &GroqChatRequest,
    ) -> Result<String, GroqAttemptFailure> {
        let payload = GroqChatPayload {
            model,
            messages: [
                GroqMessagePayload {
                    role: "system",
                    content: &request.system_prompt,
                },
                GroqMessagePayload {
                    role: "user",
                    content: &request.user_prompt,
                },
            ],
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            response_format: request.require_json_object.then_some(GroqResponseFormat {
                kind: "json_object",
            }),
        };

        let response = self
            .http
            .post(&self.endpoint)
            .bearer_auth(api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|error| GroqAttemptFailure {
                model: model.to_owned(),
                key_index,
                status_code: None,
                retry_after_seconds: None,
                retryable: error.is_connect() || error.is_timeout(),
                message: error.to_string(),
            })?;

        let status = response.status();
        let body = response.text().await.map_err(|error| GroqAttemptFailure {
            model: model.to_owned(),
            key_index,
            status_code: Some(status.as_u16()),
            retry_after_seconds: None,
            retryable: status.is_server_error(),
            message: error.to_string(),
        })?;

        if !status.is_success() {
            return Err(build_attempt_failure(model, key_index, status, &body));
        }

        let parsed: GroqChatApiResponse = serde_json::from_str(&body).map_err(|error| GroqAttemptFailure {
            model: model.to_owned(),
            key_index,
            status_code: Some(status.as_u16()),
            retry_after_seconds: None,
            retryable: false,
            message: error.to_string(),
        })?;
        let content = parsed
            .choices
            .into_iter()
            .find_map(|choice| choice.message.content)
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
            .ok_or_else(|| GroqAttemptFailure {
                model: model.to_owned(),
                key_index,
                status_code: Some(status.as_u16()),
                retry_after_seconds: None,
                retryable: false,
                message: "respuesta sin contenido util".to_owned(),
            })?;

        Ok(content)
    }
}

#[must_use]
pub fn load_groq_api_keys() -> Vec<String> {
    let mut keys = Vec::new();
    for name in ["GROQ_API_1", "GROQ_API_2", "GROQ_API_3"] {
        if let Some(value) = read_env_var(name) {
            push_unique_key(&mut keys, value);
        }
    }

    if keys.is_empty() {
        if let Some(value) = read_env_var("GROQ_API") {
            push_unique_key(&mut keys, value);
        }
    }

    keys
}

#[must_use]
pub fn extract_retry_after_seconds(body: &str) -> Option<f32> {
    let (_, tail) = body.split_once("Please retry in")?;
    let numeric: String = tail
        .chars()
        .skip_while(|character| character.is_whitespace())
        .take_while(|character| character.is_ascii_digit() || *character == '.')
        .collect();

    numeric.parse::<f32>().ok()
}

fn build_attempt_failure(
    model: &str,
    key_index: usize,
    status: StatusCode,
    body: &str,
) -> GroqAttemptFailure {
    let message = extract_error_message(body).unwrap_or_else(|| truncate(body, 500));
    GroqAttemptFailure {
        model: model.to_owned(),
        key_index,
        status_code: Some(status.as_u16()),
        retry_after_seconds: extract_retry_after_seconds(body),
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

fn read_env_var(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.trim().is_empty())
}

fn push_unique_key(keys: &mut Vec<String>, value: String) {
    if value.starts_with("gsk_") && !keys.iter().any(|key| key == &value) {
        keys.push(value);
    }
}

#[derive(Debug, Serialize)]
struct GroqChatPayload<'a> {
    model: &'a str,
    messages: [GroqMessagePayload<'a>; 2],
    temperature: f32,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<GroqResponseFormat<'a>>,
}

#[derive(Debug, Serialize)]
struct GroqMessagePayload<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Debug, Serialize)]
struct GroqResponseFormat<'a> {
    #[serde(rename = "type")]
    kind: &'a str,
}

#[derive(Debug, Deserialize)]
struct GroqChatApiResponse {
    choices: Vec<GroqChoice>,
}

#[derive(Debug, Deserialize)]
struct GroqChoice {
    message: GroqResponseMessage,
}

#[derive(Debug, Deserialize)]
struct GroqResponseMessage {
    content: Option<String>,
}

#[cfg(test)]
#[path = "groq/tests.rs"]
mod tests;
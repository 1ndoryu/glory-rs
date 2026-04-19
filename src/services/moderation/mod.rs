mod local_rules;
mod types;

use crate::audio::ia::groq::{GroqChatRequest, GroqClient, GroqClientError};
use crate::audio::ia::json_repairer::JsonRepairer;
use crate::audio::ia::openai::{OpenAiChatRequest, OpenAiClient, OpenAiClientError};
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;

pub use types::{
    ModerationAdminPanel, ModerationAiAssessment, ModerationCategory, ModerationDecision,
    ModerationEntityKind, ModerationLocalFinding, ModerationOpenAiFailure, ModerationParseFailure,
    ModerationProvider, ModerationProviderFailure, ModerationRequest, ModerationResult,
    ModerationVerdict,
};

use local_rules::inspect_request;

const MODERATION_SYSTEM_PROMPT: &str = "You moderate user-generated metadata for Kamples, a music platform. Return JSON only. Prefer 'revision' when uncertain. Harmless profanity alone is not enough to reject. Focus on spam, scams, explicit sexual content, violent threats, hate, doxxing, illegal activity, copyright risk, and misleading metadata.";
const DEFAULT_MODERATION_GROQ_MODEL_CHAIN: [&str; 4] = [
    "openai/gpt-oss-safeguard-20b",
    "openai/gpt-oss-120b",
    "moonshotai/kimi-k2-instruct-0905",
    "llama-3.3-70b-versatile",
];

/* [174A-43] Servicio de moderación reusable.
 * Separa cuatro capas explícitas: pre-filtro local, categorización IA,
 * decisión final y payload listo para panel admin. Todavía no persiste en una
 * cola porque el runtime nuevo aún no portó `moderation_queue`. */

#[derive(Clone)]
pub struct ModerationService {
    groq: Option<GroqClient>,
    openai: Option<OpenAiClient>,
}

#[derive(Debug, Error)]
pub enum ModerationServiceError {
    #[error("No se pudo inicializar {provider}: {message}")]
    ProviderInitialization {
        provider: &'static str,
        message: String,
    },
}

#[derive(Debug, Default)]
struct AiExecutionOutcome {
    assessment: Option<ModerationAiAssessment>,
    failures: Vec<ModerationProviderFailure>,
    providers_configured: bool,
}

#[derive(Debug, Deserialize)]
struct ModerationAiPayload {
    safe: Option<bool>,
    category: Option<String>,
    confidence: Option<f32>,
    recommended_level: Option<String>,
    reason_code: Option<String>,
    summary: Option<String>,
}

impl ModerationService {
    pub fn from_env() -> Result<Self, ModerationServiceError> {
        let groq = match GroqClient::from_env_with_model_chain(
            DEFAULT_MODERATION_GROQ_MODEL_CHAIN
                .iter()
                .map(ToString::to_string)
                .collect(),
        ) {
            Ok(client) => Some(client),
            Err(GroqClientError::MissingApiKeys) => None,
            Err(error) => {
                return Err(ModerationServiceError::ProviderInitialization {
                    provider: ModerationProvider::Groq.as_str(),
                    message: error.to_string(),
                });
            }
        };

        let openai = match OpenAiClient::from_env() {
            Ok(client) => Some(client),
            Err(OpenAiClientError::MissingApiKey) => None,
            Err(error) => {
                return Err(ModerationServiceError::ProviderInitialization {
                    provider: ModerationProvider::OpenAi.as_str(),
                    message: error.to_string(),
                });
            }
        };

        Ok(Self { groq, openai })
    }

    #[must_use]
    pub fn new(groq: Option<GroqClient>, openai: Option<OpenAiClient>) -> Self {
        Self { groq, openai }
    }

    pub async fn moderate(&self, request: &ModerationRequest) -> ModerationResult {
        let local_findings = inspect_request(request);
        let ai = if local_findings
            .iter()
            .any(|finding| finding.verdict == ModerationVerdict::Rejected)
        {
            AiExecutionOutcome::default()
        } else {
            self.execute_ai_layer(request).await
        };
        let decision = build_decision(request, &local_findings, &ai);
        let admin_panel = build_admin_panel(request, &decision, &local_findings, &ai);

        ModerationResult {
            local_findings,
            ai_assessment: ai.assessment,
            provider_failures: ai.failures,
            decision,
            admin_panel,
        }
    }

    async fn execute_ai_layer(&self, request: &ModerationRequest) -> AiExecutionOutcome {
        if !request.has_content() {
            return AiExecutionOutcome::default();
        }

        let prompt = build_moderation_prompt(request);
        let mut outcome = AiExecutionOutcome {
            providers_configured: self.groq.is_some() || self.openai.is_some(),
            ..AiExecutionOutcome::default()
        };

        if let Some(groq) = &self.groq {
            let request = GroqChatRequest {
                user_prompt: prompt.clone(),
                system_prompt: MODERATION_SYSTEM_PROMPT.to_owned(),
                temperature: 0.1,
                max_tokens: 400,
                require_json_object: true,
            };

            match groq.chat_completion(&request).await {
                Ok(success) => match parse_ai_payload(&success.content) {
                    Ok(payload) => {
                        outcome.assessment = Some(payload.into_assessment(
                            ModerationProvider::Groq,
                            success.model,
                            success.attempt_count,
                            Some(success.key_index),
                        ));
                        return outcome;
                    }
                    Err(message) => outcome.failures.push(ModerationProviderFailure::Parse(
                        ModerationParseFailure {
                            provider: ModerationProvider::Groq,
                            model: success.model,
                            message,
                        },
                    )),
                },
                Err(GroqClientError::Exhausted { failures }) => outcome
                    .failures
                    .extend(failures.into_iter().map(ModerationProviderFailure::Groq)),
                Err(GroqClientError::MissingApiKeys | GroqClientError::MissingModels) => {}
            }
        }

        if let Some(openai) = &self.openai {
            let request = OpenAiChatRequest {
                user_prompt: prompt,
                system_prompt: MODERATION_SYSTEM_PROMPT.to_owned(),
                temperature: 0.1,
                max_tokens: 400,
                require_json_object: true,
            };

            match openai.chat_completion(&request).await {
                Ok(success) => match parse_ai_payload(&success.content) {
                    Ok(payload) => {
                        outcome.assessment = Some(payload.into_assessment(
                            ModerationProvider::OpenAi,
                            success.model,
                            success.attempt_count,
                            None,
                        ));
                    }
                    Err(message) => outcome.failures.push(ModerationProviderFailure::Parse(
                        ModerationParseFailure {
                            provider: ModerationProvider::OpenAi,
                            model: success.model,
                            message,
                        },
                    )),
                },
                Err(error) => outcome
                    .failures
                    .push(ModerationProviderFailure::OpenAi(map_openai_error(error))),
            }
        }

        outcome
    }
}

impl ModerationAiPayload {
    fn into_assessment(
        self,
        provider: ModerationProvider,
        model: String,
        attempt_count: usize,
        provider_key_index: Option<usize>,
    ) -> ModerationAiAssessment {
        let confidence = self.confidence.unwrap_or(0.5).clamp(0.0, 1.0);
        let category = ModerationCategory::from_raw(self.category.as_deref().unwrap_or("safe"));
        let verdict = normalize_verdict(self.recommended_level.as_deref(), self.safe, confidence);
        let reason_code = sanitize_reason_code(
            self.reason_code
                .as_deref()
                .unwrap_or_else(|| category.as_str()),
        );

        ModerationAiAssessment {
            provider,
            model,
            category,
            verdict,
            confidence,
            reason_code,
            summary: self
                .summary
                .unwrap_or_else(|| default_summary(verdict, category)),
            attempt_count,
            provider_key_index,
        }
    }
}

fn parse_ai_payload(raw: &str) -> Result<ModerationAiPayload, String> {
    let object = JsonRepairer::extract_json_object(raw).map_err(|error| error.to_string())?;
    serde_json::from_value(Value::Object(object)).map_err(|error| error.to_string())
}

fn build_decision(
    request: &ModerationRequest,
    local_findings: &[ModerationLocalFinding],
    ai: &AiExecutionOutcome,
) -> ModerationDecision {
    if let Some(local) = strongest_local_finding(local_findings) {
        if local.verdict == ModerationVerdict::Rejected {
            return ModerationDecision {
                verdict: local.verdict,
                category: local.category,
                reason_code: local.reason_code.clone(),
                summary: format!("Prefiltro local detecto '{}'", local.matched_text),
                manual_review: true,
            };
        }
    }

    if let Some(assessment) = &ai.assessment {
        let verdict = match assessment.verdict {
            ModerationVerdict::Rejected if assessment.confidence >= 0.85 => {
                ModerationVerdict::Rejected
            }
            ModerationVerdict::Rejected | ModerationVerdict::Review => ModerationVerdict::Review,
            ModerationVerdict::Approved if strongest_local_finding(local_findings).is_some() => {
                ModerationVerdict::Review
            }
            ModerationVerdict::Approved => ModerationVerdict::Approved,
        };

        return ModerationDecision {
            verdict,
            category: assessment.category,
            reason_code: assessment.reason_code.clone(),
            summary: assessment.summary.clone(),
            manual_review: verdict != ModerationVerdict::Approved,
        };
    }

    if let Some(local) = strongest_local_finding(local_findings) {
        return ModerationDecision {
            verdict: ModerationVerdict::Review,
            category: local.category,
            reason_code: local.reason_code.clone(),
            summary: format!(
                "Prefiltro local marco '{}' para revision",
                local.matched_text
            ),
            manual_review: true,
        };
    }

    if ai.providers_configured && !ai.failures.is_empty() && request.has_content() {
        return ModerationDecision {
            verdict: ModerationVerdict::Review,
            category: ModerationCategory::Other,
            reason_code: "provider_failure".to_owned(),
            summary: "Los proveedores IA fallaron; se requiere revision manual".to_owned(),
            manual_review: true,
        };
    }

    ModerationDecision {
        verdict: ModerationVerdict::Approved,
        category: ModerationCategory::Safe,
        reason_code: if ai.providers_configured {
            "sin_hallazgos"
        } else {
            "local_only_fallback"
        }
        .to_owned(),
        summary: default_summary(ModerationVerdict::Approved, ModerationCategory::Safe),
        manual_review: false,
    }
}

fn build_admin_panel(
    request: &ModerationRequest,
    decision: &ModerationDecision,
    local_findings: &[ModerationLocalFinding],
    ai: &AiExecutionOutcome,
) -> ModerationAdminPanel {
    let mut badges = vec![
        request.entity_kind.as_str().to_owned(),
        decision.verdict.as_str().to_owned(),
        decision.category.as_str().to_owned(),
    ];
    if let Some(assessment) = &ai.assessment {
        badges.push(assessment.provider.as_str().to_owned());
    } else if !ai.providers_configured {
        badges.push("local_only".to_owned());
    }

    let mut evidence = local_findings
        .iter()
        .map(|finding| finding.matched_text.clone())
        .collect::<Vec<_>>();
    if let Some(assessment) = &ai.assessment {
        evidence.push(format!(
            "ia:{} ({:.0}%)",
            assessment.reason_code,
            assessment.confidence * 100.0
        ));
    }
    evidence.extend(ai.failures.iter().take(2).map(summarize_failure));

    ModerationAdminPanel {
        headline: format!(
            "{} {}",
            request.entity_kind.as_str(),
            decision.verdict.as_str()
        ),
        summary: decision.summary.clone(),
        badges,
        evidence,
        priority: match decision.verdict {
            ModerationVerdict::Rejected => 95,
            ModerationVerdict::Review => 70,
            ModerationVerdict::Approved if ai.providers_configured => 15,
            ModerationVerdict::Approved => 10,
        },
    }
}

fn strongest_local_finding(findings: &[ModerationLocalFinding]) -> Option<&ModerationLocalFinding> {
    findings
        .iter()
        .max_by_key(|finding| finding.verdict.priority())
}

fn build_moderation_prompt(request: &ModerationRequest) -> String {
    format!(
        "Moderate this {kind} for Kamples and return JSON with keys safe, category, confidence, recommended_level, reason_code, summary. Use category one of: safe, spam, sexual, violence, hate, harassment, illegal, doxxing, scam, copyright, misleading_metadata, other. recommended_level must be aprobado, revision or rechazado.\n\nContent:\n{content}",
        kind = request.entity_kind.as_str(),
        content = request.combined_text(),
    )
}

fn normalize_verdict(
    level: Option<&str>,
    safe: Option<bool>,
    confidence: f32,
) -> ModerationVerdict {
    match level
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "rechazado" | "rejected" => ModerationVerdict::Rejected,
        "revision" | "review" => ModerationVerdict::Review,
        "aprobado" | "approved" => ModerationVerdict::Approved,
        _ => match safe {
            Some(true) => ModerationVerdict::Approved,
            Some(false) if confidence >= 0.85 => ModerationVerdict::Rejected,
            Some(false) | None => ModerationVerdict::Review,
        },
    }
}

fn sanitize_reason_code(value: &str) -> String {
    let mut sanitized = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    while sanitized.contains("__") {
        sanitized = sanitized.replace("__", "_");
    }
    sanitized.trim_matches('_').to_owned()
}

fn default_summary(verdict: ModerationVerdict, category: ModerationCategory) -> String {
    match verdict {
        ModerationVerdict::Approved => "Sin hallazgos relevantes de moderacion".to_owned(),
        ModerationVerdict::Review => {
            format!("Contenido marcado para revision por {}", category.as_str())
        }
        ModerationVerdict::Rejected => format!("Contenido bloqueado por {}", category.as_str()),
    }
}

fn summarize_failure(failure: &ModerationProviderFailure) -> String {
    match failure {
        ModerationProviderFailure::Groq(failure) => format!("groq:{}", failure.message),
        ModerationProviderFailure::OpenAi(failure) => format!("openai:{}", failure.message),
        ModerationProviderFailure::Parse(failure) => format!("parse:{}", failure.message),
    }
}

fn map_openai_error(error: OpenAiClientError) -> ModerationOpenAiFailure {
    match error {
        OpenAiClientError::MissingApiKey => ModerationOpenAiFailure {
            status_code: None,
            retry_after_seconds: None,
            retryable: false,
            message: "missing_api_key".to_owned(),
        },
        OpenAiClientError::JsonSchemaRejected {
            status_code,
            message,
        } => ModerationOpenAiFailure {
            status_code: Some(status_code),
            retry_after_seconds: None,
            retryable: false,
            message,
        },
        OpenAiClientError::Request {
            status_code,
            retry_after_seconds,
            retryable,
            message,
        } => ModerationOpenAiFailure {
            status_code,
            retry_after_seconds,
            retryable,
            message,
        },
    }
}

#[cfg(test)]
mod tests;

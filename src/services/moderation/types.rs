use crate::audio::ia::groq::GroqAttemptFailure;
use std::fmt::Write as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModerationEntityKind {
    Sample,
    Publication,
    Comment,
    Article,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModerationVerdict {
    Approved,
    Review,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModerationCategory {
    Safe,
    Spam,
    Sexual,
    Violence,
    Hate,
    Harassment,
    Illegal,
    Doxxing,
    Scam,
    Copyright,
    MisleadingMetadata,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModerationProvider {
    Groq,
    OpenAi,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModerationRequest {
    pub entity_kind: ModerationEntityKind,
    pub entity_id: Option<i64>,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
    pub primary_folder: Option<String>,
    pub secondary_folder: Option<String>,
    pub image_urls: Vec<String>,
    pub extra_context: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModerationLocalFinding {
    pub category: ModerationCategory,
    pub verdict: ModerationVerdict,
    pub reason_code: String,
    pub matched_text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModerationAiAssessment {
    pub provider: ModerationProvider,
    pub model: String,
    pub category: ModerationCategory,
    pub verdict: ModerationVerdict,
    pub confidence: f32,
    pub reason_code: String,
    pub summary: String,
    pub attempt_count: usize,
    pub provider_key_index: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModerationDecision {
    pub verdict: ModerationVerdict,
    pub category: ModerationCategory,
    pub reason_code: String,
    pub summary: String,
    pub manual_review: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModerationAdminPanel {
    pub headline: String,
    pub summary: String,
    pub badges: Vec<String>,
    pub evidence: Vec<String>,
    pub priority: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModerationOpenAiFailure {
    pub status_code: Option<u16>,
    pub retry_after_seconds: Option<f32>,
    pub retryable: bool,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModerationParseFailure {
    pub provider: ModerationProvider,
    pub model: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModerationProviderFailure {
    Groq(GroqAttemptFailure),
    OpenAi(ModerationOpenAiFailure),
    Parse(ModerationParseFailure),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModerationResult {
    pub local_findings: Vec<ModerationLocalFinding>,
    pub ai_assessment: Option<ModerationAiAssessment>,
    pub provider_failures: Vec<ModerationProviderFailure>,
    pub decision: ModerationDecision,
    pub admin_panel: ModerationAdminPanel,
}

impl ModerationEntityKind {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sample => "sample",
            Self::Publication => "publication",
            Self::Comment => "comment",
            Self::Article => "article",
        }
    }
}

impl ModerationVerdict {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Approved => "aprobado",
            Self::Review => "revision",
            Self::Rejected => "rechazado",
        }
    }

    #[must_use]
    pub fn priority(self) -> u8 {
        match self {
            Self::Rejected => 3,
            Self::Review => 2,
            Self::Approved => 1,
        }
    }
}

impl ModerationCategory {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Safe => "safe",
            Self::Spam => "spam",
            Self::Sexual => "sexual",
            Self::Violence => "violence",
            Self::Hate => "hate",
            Self::Harassment => "harassment",
            Self::Illegal => "illegal",
            Self::Doxxing => "doxxing",
            Self::Scam => "scam",
            Self::Copyright => "copyright",
            Self::MisleadingMetadata => "misleading_metadata",
            Self::Other => "other",
        }
    }

    #[must_use]
    pub fn from_raw(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "safe" | "none" | "sin_hallazgos" => Self::Safe,
            "spam" => Self::Spam,
            "sexual" | "nsfw" | "porn" => Self::Sexual,
            "violence" | "violent" => Self::Violence,
            "hate" => Self::Hate,
            "harassment" => Self::Harassment,
            "illegal" => Self::Illegal,
            "doxxing" | "doxx" => Self::Doxxing,
            "scam" => Self::Scam,
            "copyright" | "copyright_risk" => Self::Copyright,
            "misleading_metadata" | "metadata_spam" | "misleading" => Self::MisleadingMetadata,
            _ => Self::Other,
        }
    }
}

impl ModerationProvider {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Groq => "Groq",
            Self::OpenAi => "OpenAI",
        }
    }
}

impl ModerationRequest {
    #[must_use]
    pub fn has_content(&self) -> bool {
        !self.combined_text().trim().is_empty() || !self.image_urls.is_empty()
    }

    #[must_use]
    pub fn combined_text(&self) -> String {
        let mut result = String::new();

        push_named_field(&mut result, "title", &self.title);
        push_named_field(&mut result, "body", &self.body);

        if !self.tags.is_empty() {
            let _ = writeln!(result, "tags: {}", self.tags.join(", "));
        }

        if let Some(primary_folder) = self.primary_folder.as_deref() {
            push_named_field(&mut result, "primary_folder", primary_folder);
        }
        if let Some(secondary_folder) = self.secondary_folder.as_deref() {
            push_named_field(&mut result, "secondary_folder", secondary_folder);
        }
        if let Some(extra_context) = self.extra_context.as_deref() {
            push_named_field(&mut result, "extra_context", extra_context);
        }

        if !self.image_urls.is_empty() {
            let _ = writeln!(result, "image_urls: {}", self.image_urls.join(", "));
        }

        result.trim().to_owned()
    }
}

impl ModerationProviderFailure {
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Groq(failure) => failure.retryable,
            Self::OpenAi(failure) => failure.retryable,
            Self::Parse(_) => false,
        }
    }

    #[must_use]
    pub fn retry_after_seconds(&self) -> Option<f32> {
        match self {
            Self::Groq(failure) => failure.retry_after_seconds,
            Self::OpenAi(failure) => failure.retry_after_seconds,
            Self::Parse(_) => None,
        }
    }
}

impl ModerationResult {
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        self.provider_failures
            .iter()
            .any(ModerationProviderFailure::is_retryable)
    }

    #[must_use]
    pub fn retry_after_seconds(&self) -> Option<f32> {
        self.provider_failures
            .iter()
            .filter_map(ModerationProviderFailure::retry_after_seconds)
            .fold(None, |current, value| match current {
                Some(existing) if existing >= value => Some(existing),
                _ => Some(value),
            })
    }

    #[must_use]
    pub fn recommended_sample_state(&self) -> &'static str {
        match self.decision.verdict {
            ModerationVerdict::Approved => "activo",
            ModerationVerdict::Review | ModerationVerdict::Rejected => "en_supervision",
        }
    }
}

fn push_named_field(target: &mut String, name: &str, value: &str) {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return;
    }

    let _ = writeln!(target, "{name}: {trimmed}");
}

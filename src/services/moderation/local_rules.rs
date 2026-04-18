use super::types::{
    ModerationCategory, ModerationLocalFinding, ModerationRequest, ModerationVerdict,
};
use std::collections::HashSet;

const SPAM_TERMS: [&str; 9] = [
    "buy now",
    "free money",
    "crypto signal",
    "dm for promo",
    "telegram",
    "whatsapp",
    "airdrop",
    "100% profit",
    "cashapp",
];
const SEXUAL_TERMS: [&str; 7] = [
    "onlyfans",
    "porn",
    "xxx",
    "nudes",
    "sex tape",
    "explicit sex",
    "adult cam",
];
const ILLEGAL_TERMS: [&str; 7] = [
    "buy cocaine",
    "sell drugs",
    "credit card dump",
    "stolen card",
    "gun for sale",
    "fake passport",
    "malware builder",
];
const COPYRIGHT_TERMS: [&str; 6] = [
    "exclusive leak",
    "stolen from",
    "copyright strike",
    "dmca bait",
    "pirated",
    "unreleased major label",
];

pub(super) fn inspect_request(request: &ModerationRequest) -> Vec<ModerationLocalFinding> {
    let mut findings = Vec::new();
    let normalized_text = normalize(&request.combined_text());

    if let Some(term) = first_match(&normalized_text, &SEXUAL_TERMS) {
        findings.push(ModerationLocalFinding {
            category: ModerationCategory::Sexual,
            verdict: ModerationVerdict::Rejected,
            reason_code: "local_explicit_sexual".to_owned(),
            matched_text: term.to_owned(),
        });
    }

    if let Some(term) = first_match(&normalized_text, &ILLEGAL_TERMS) {
        findings.push(ModerationLocalFinding {
            category: ModerationCategory::Illegal,
            verdict: ModerationVerdict::Rejected,
            reason_code: "local_illegal_activity".to_owned(),
            matched_text: term.to_owned(),
        });
    }

    if count_urls(&normalized_text) >= 2 {
        findings.push(ModerationLocalFinding {
            category: ModerationCategory::Spam,
            verdict: ModerationVerdict::Rejected,
            reason_code: "local_multi_url_spam".to_owned(),
            matched_text: "multiple_urls".to_owned(),
        });
    } else if let Some(term) = first_match(&normalized_text, &SPAM_TERMS) {
        findings.push(ModerationLocalFinding {
            category: ModerationCategory::Spam,
            verdict: ModerationVerdict::Rejected,
            reason_code: "local_promotional_spam".to_owned(),
            matched_text: term.to_owned(),
        });
    }

    if let Some(term) = first_match(&normalized_text, &COPYRIGHT_TERMS) {
        findings.push(ModerationLocalFinding {
            category: ModerationCategory::Copyright,
            verdict: ModerationVerdict::Review,
            reason_code: "local_copyright_risk".to_owned(),
            matched_text: term.to_owned(),
        });
    }

    if has_noisy_tag_pattern(&request.tags) {
        findings.push(ModerationLocalFinding {
            category: ModerationCategory::MisleadingMetadata,
            verdict: ModerationVerdict::Review,
            reason_code: "local_tag_noise".to_owned(),
            matched_text: request.tags.join(", "),
        });
    }

    findings
}

fn normalize(value: &str) -> String {
    value
        .to_ascii_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn first_match<'a>(haystack: &str, needles: &'a [&str]) -> Option<&'a str> {
    needles.iter().copied().find(|needle| haystack.contains(needle))
}

fn count_urls(value: &str) -> usize {
    ["http://", "https://", "www.", "t.me/", "telegram.me/"]
        .iter()
        .map(|needle| value.matches(needle).count())
        .sum()
}

fn has_noisy_tag_pattern(tags: &[String]) -> bool {
    if tags.len() >= 12 {
        return true;
    }

    let mut unique = HashSet::new();
    let mut duplicates = 0_usize;
    for tag in tags {
        let normalized = tag.trim().to_ascii_lowercase();
        if normalized.is_empty() {
            continue;
        }

        if !unique.insert(normalized) {
            duplicates += 1;
        }
    }

    duplicates >= 3
}
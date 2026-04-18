use pgvector::Vector;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const EMBEDDING_DIMENSIONS: usize = 128;
pub const BPM_MAX: u16 = 300;
pub const DURATION_MAX_SECONDS: f32 = 600.0;
const KEY_OFFSET: usize = 1;
const SCALE_OFFSET: usize = 13;
const TYPE_OFFSET: usize = 15;
const TAGS_OFFSET: usize = 22;
const TAGS_SLOTS: usize = EMBEDDING_DIMENSIONS - TAGS_OFFSET;

/* [174A-33] Embedding determinista 128d para similitud y perfilado.
 * Replica la estructura legacy: atributos musicales en slots fijos y tags
 * hasheados con CRC32 a 106 posiciones estables. La normalización L2 solo se
 * usa al construir perfiles ponderados, no en el embedding raw del sample. */

#[derive(Debug, Clone, PartialEq)]
pub struct AudioEmbedding {
    values: [f32; EMBEDDING_DIMENSIONS],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EmbeddingInput {
    pub bpm: Option<u16>,
    pub music_key: Option<String>,
    pub scale: Option<String>,
    pub sample_type: Option<String>,
    pub duration_seconds: Option<f32>,
    pub is_premium: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EmbeddingError {
    #[error("Dimensión de embedding inválida: {0}")]
    InvalidDimension(usize),
}

impl AudioEmbedding {
    #[must_use]
    pub fn generate(input: &EmbeddingInput) -> Self {
        let mut values = [0.0_f32; EMBEDDING_DIMENSIONS];

        values[0] = input
            .bpm
            .map_or(0.0, |bpm| f32::from(bpm.min(BPM_MAX)) / f32::from(BPM_MAX));

        if let Some(key_index) = input.music_key.as_deref().and_then(music_key_index) {
            values[KEY_OFFSET + key_index] = 1.0;
        }

        match input.scale.as_deref().map(normalize_scale) {
            Some(Some("major")) => values[SCALE_OFFSET] = 1.0,
            Some(Some("minor")) => values[SCALE_OFFSET + 1] = 1.0,
            _ => {
                values[SCALE_OFFSET] = 0.5;
                values[SCALE_OFFSET + 1] = 0.5;
            }
        }

        let sample_type_index = input
            .sample_type
            .as_deref()
            .map_or(0, sample_type_index);
        values[TYPE_OFFSET + sample_type_index] = 1.0;

        if let Some(duration_seconds) = input.duration_seconds.filter(|duration| *duration > 0.0) {
            values[20] = ((1.0 + duration_seconds.min(DURATION_MAX_SECONDS)).ln()
                / (1.0 + DURATION_MAX_SECONDS).ln())
                .min(1.0);
        }

        if input.is_premium {
            values[21] = 1.0;
        }

        if !input.tags.is_empty() {
            let weight = 1.0_f32 / f32::from(u16::try_from(input.tags.len()).unwrap_or(u16::MAX));
            for tag in &input.tags {
                let normalized = normalize_tag(tag);
                if normalized.is_empty() {
                    continue;
                }

                let slot = usize::try_from(crc32fast::hash(normalized.as_bytes())).unwrap_or(0) % TAGS_SLOTS;
                values[TAGS_OFFSET + slot] = (values[TAGS_OFFSET + slot] + weight).min(1.0);
            }
        }

        Self { values }
    }

    pub fn from_pgvector(vector: &Vector) -> Result<Self, EmbeddingError> {
        Self::from_slice(vector.as_slice())
    }

    pub fn from_slice(values: &[f32]) -> Result<Self, EmbeddingError> {
        if values.len() != EMBEDDING_DIMENSIONS {
            return Err(EmbeddingError::InvalidDimension(values.len()));
        }

        let mut embedding = [0.0_f32; EMBEDDING_DIMENSIONS];
        embedding.copy_from_slice(values);
        Ok(Self { values: embedding })
    }

    #[must_use]
    pub fn to_pgvector(&self) -> Vector {
        Vector::from(self.values.to_vec())
    }

    #[must_use]
    pub fn as_slice(&self) -> &[f32] {
        &self.values
    }

    #[must_use]
    pub fn to_vec(&self) -> Vec<f32> {
        self.values.to_vec()
    }

    #[must_use]
    pub fn l2_norm(&self) -> f32 {
        self.values.iter().map(|value| value * value).sum::<f32>().sqrt()
    }

    pub fn build_weighted_profile(embeddings: &[Self], weights: &[f32]) -> Option<Self> {
        if embeddings.is_empty() || embeddings.len() != weights.len() {
            return None;
        }

        let mut accumulated = [0.0_f32; EMBEDDING_DIMENSIONS];
        let mut total_weight = 0.0_f32;

        for (embedding, weight) in embeddings.iter().zip(weights.iter()) {
            if *weight <= 0.0 {
                continue;
            }

            for (slot, value) in accumulated.iter_mut().zip(embedding.values.iter()) {
                *slot += value * weight;
            }
            total_weight += weight;
        }

        if total_weight <= 0.0 {
            return None;
        }

        for slot in &mut accumulated {
            *slot /= total_weight;
        }

        let norm = accumulated.iter().map(|value| value * value).sum::<f32>().sqrt();
        if norm > 0.0 {
            for slot in &mut accumulated {
                *slot /= norm;
            }
        }

        Some(Self { values: accumulated })
    }
}

fn music_key_index(value: &str) -> Option<usize> {
    match normalize_music_key(value).as_str() {
        "C" => Some(0),
        "C#" | "DB" => Some(1),
        "D" => Some(2),
        "D#" | "EB" => Some(3),
        "E" => Some(4),
        "F" => Some(5),
        "F#" | "GB" => Some(6),
        "G" => Some(7),
        "G#" | "AB" => Some(8),
        "A" => Some(9),
        "A#" | "BB" => Some(10),
        "B" => Some(11),
        _ => None,
    }
}

fn normalize_music_key(value: &str) -> String {
    value.trim().to_ascii_uppercase()
}

fn normalize_scale(value: &str) -> Option<&'static str> {
    match value.trim().to_ascii_lowercase().as_str() {
        "major" | "mayor" => Some("major"),
        "minor" | "menor" => Some("minor"),
        _ => None,
    }
}

fn sample_type_index(value: &str) -> usize {
    match value.trim().to_ascii_lowercase().as_str() {
        "one_shot" | "one shot" | "oneshot" => 1,
        _ => 0,
    }
}

fn normalize_tag(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

#[cfg(test)]
#[path = "embeddings/tests.rs"]
mod tests;
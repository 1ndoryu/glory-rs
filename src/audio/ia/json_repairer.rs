use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use thiserror::Error;

const DEFAULT_TIPO: &str = "oneshot";
const DEFAULT_FOLDER: &str = "General";
const PRIMARY_FOLDERS: [(&str, &str); 7] = [
    ("drums", "Drums"),
    ("loops", "Loops"),
    ("samples", "Samples"),
    ("fx", "FX"),
    ("instruments", "Instruments"),
    ("vocals", "Vocals"),
    ("general", DEFAULT_FOLDER),
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioCreativeMetadata {
    pub nombre_archivo_base: String,
    pub tags: Vec<String>,
    pub tags_es: Vec<String>,
    pub tipo: String,
    pub genero: Vec<String>,
    pub emocion: Vec<String>,
    pub emocion_es: Vec<String>,
    pub instrumentos: Vec<String>,
    pub artista_vibes: Vec<String>,
    pub descripcion_corta: String,
    pub descripcion_corta_es: String,
    pub descripcion: String,
    pub descripcion_es: String,
    pub carpeta_primaria: String,
    pub carpeta_secundaria: String,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum JsonRepairError {
    #[error("La respuesta del proveedor no es JSON valido")]
    InvalidProviderResponse,
    #[error("La respuesta del proveedor no trae contenido textual")]
    MissingProviderContent,
    #[error("No se pudo extraer un objeto JSON del texto")]
    JsonNotFound,
    #[error("El JSON extraido no es un objeto")]
    JsonRootIsNotObject,
}

pub struct JsonRepairer;

impl JsonRepairer {
    pub fn extract_metadata_from_provider_response(
        raw_response: &str,
    ) -> Result<AudioCreativeMetadata, JsonRepairError> {
        let response: Value =
            serde_json::from_str(raw_response).map_err(|_| JsonRepairError::InvalidProviderResponse)?;
        let content = response
            .get("choices")
            .and_then(Value::as_array)
            .and_then(|choices| choices.first())
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|content| !content.is_empty())
            .ok_or(JsonRepairError::MissingProviderContent)?;

        Self::extract_metadata_from_text(content)
    }

    pub fn extract_metadata_from_text(text: &str) -> Result<AudioCreativeMetadata, JsonRepairError> {
        let object = Self::extract_json_object(text)?;
        Ok(AudioCreativeMetadata::from_object(&object))
    }

    pub fn extract_json_object(text: &str) -> Result<Map<String, Value>, JsonRepairError> {
        for candidate in collect_candidates(text) {
            if let Some(object) = parse_object_candidate(&candidate) {
                return Ok(object);
            }

            let sanitized = sanitize_json_control_chars(&candidate);
            if sanitized != candidate {
                if let Some(object) = parse_object_candidate(&sanitized) {
                    return Ok(object);
                }
            }
        }

        Err(JsonRepairError::JsonNotFound)
    }
}

impl AudioCreativeMetadata {
    #[must_use]
    pub fn from_object(object: &Map<String, Value>) -> Self {
        Self {
            nombre_archivo_base: sanitize_scalar_text(get_string(object, "nombre_archivo_base"), 80),
            tags: sanitize_string_list(object.get("tags"), 15),
            tags_es: sanitize_string_list(object.get("tags_es"), 15),
            tipo: normalize_tipo(get_string(object, "tipo")),
            genero: sanitize_string_list(object.get("genero"), 5),
            emocion: sanitize_string_list(object.get("emocion"), 5),
            emocion_es: sanitize_string_list(object.get("emocion_es"), 5),
            instrumentos: sanitize_string_list(object.get("instrumentos"), 10),
            artista_vibes: sanitize_string_list(object.get("artista_vibes"), 5),
            descripcion_corta: sanitize_scalar_text(get_string(object, "descripcion_corta"), 150),
            descripcion_corta_es: sanitize_scalar_text(get_string(object, "descripcion_corta_es"), 150),
            descripcion: sanitize_scalar_text(get_string(object, "descripcion"), 500),
            descripcion_es: sanitize_scalar_text(get_string(object, "descripcion_es"), 500),
            carpeta_primaria: normalize_primary_folder(get_string(object, "carpeta_primaria")),
            carpeta_secundaria: normalize_secondary_folder(get_string(object, "carpeta_secundaria")),
        }
    }

    #[must_use]
    pub fn to_json_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or_else(|_| Value::Object(Map::new()))
    }
}

fn collect_candidates(text: &str) -> Vec<String> {
    let trimmed = text.trim();
    let mut candidates = Vec::new();

    if !trimmed.is_empty() {
        candidates.push(trimmed.to_owned());
    }

    for block in extract_fenced_blocks(trimmed) {
        if !block.is_empty() {
            candidates.push(block);
        }
    }

    if let Some(object_candidate) = find_first_balanced_object(trimmed) {
        candidates.push(object_candidate);
    }

    candidates
}

fn extract_fenced_blocks(text: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut cursor = text;

    while let Some(start) = cursor.find("```") {
        let after_open = &cursor[start + 3..];
        let Some(end) = after_open.find("```") else {
            break;
        };

        let block = &after_open[..end];
        let block = strip_fence_language(block);
        let block = block.trim();
        if !block.is_empty() {
            blocks.push(block.to_owned());
        }

        cursor = &after_open[end + 3..];
    }

    blocks
}

fn strip_fence_language(block: &str) -> &str {
    let trimmed = block.trim_start();
    if let Some(rest) = trimmed.strip_prefix("json") {
        return rest.trim_start_matches(['\r', '\n', ' ']);
    }

    if let Some(newline_index) = trimmed.find(['\n', '\r']) {
        let (first_line, rest) = trimmed.split_at(newline_index);
        if first_line.chars().all(|character| character.is_ascii_alphabetic()) {
            return rest.trim_start_matches(['\r', '\n']);
        }
    }

    trimmed
}

fn find_first_balanced_object(text: &str) -> Option<String> {
    let mut start_index = None;
    let mut depth = 0_u32;
    let mut in_string = false;
    let mut escape = false;

    for (index, character) in text.char_indices() {
        if in_string {
            if escape {
                escape = false;
                continue;
            }

            match character {
                '\\' => escape = true,
                '"' => in_string = false,
                _ => {}
            }
            continue;
        }

        match character {
            '"' => in_string = true,
            '{' => {
                if depth == 0 {
                    start_index = Some(index);
                }
                depth += 1;
            }
            '}' => {
                if depth == 0 {
                    continue;
                }

                depth -= 1;
                if depth == 0 {
                    let start = start_index?;
                    let end = index + character.len_utf8();
                    return Some(text[start..end].to_owned());
                }
            }
            _ => {}
        }
    }

    None
}

fn parse_object_candidate(candidate: &str) -> Option<Map<String, Value>> {
    parse_candidate(candidate).and_then(|value| match value {
        Value::Object(object) => Some(object),
        _ => None,
    })
}

fn parse_candidate(candidate: &str) -> Option<Value> {
    serde_json::from_str(candidate)
        .ok()
        .or_else(|| json5::from_str(candidate).ok())
}

fn sanitize_json_control_chars(candidate: &str) -> String {
    let mut result = String::with_capacity(candidate.len());
    let mut in_string = false;
    let mut escape = false;

    for character in candidate.chars() {
        if in_string {
            if escape {
                result.push(character);
                escape = false;
                continue;
            }

            match character {
                '\\' => {
                    result.push(character);
                    escape = true;
                }
                '"' => {
                    result.push(character);
                    in_string = false;
                }
                _ if character.is_control() => result.push(' '),
                _ => result.push(character),
            }
            continue;
        }

        if character == '"' {
            in_string = true;
        }

        result.push(character);
    }

    result
}

fn get_string<'a>(object: &'a Map<String, Value>, key: &str) -> &'a str {
    object.get(key).and_then(Value::as_str).unwrap_or_default()
}

fn sanitize_string_list(value: Option<&Value>, max_items: usize) -> Vec<String> {
    let values = match value {
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(Value::as_str)
            .map(sanitize_unbounded_text)
            .filter(|item| !item.is_empty())
            .take(max_items)
            .collect(),
        Some(Value::String(item)) => {
            let sanitized = sanitize_unbounded_text(item);
            if sanitized.is_empty() {
                Vec::new()
            } else {
                vec![sanitized]
            }
        }
        _ => Vec::new(),
    };

    values
}

fn sanitize_scalar_text(value: &str, max_chars: usize) -> String {
    sanitize_and_limit_text(value, Some(max_chars))
}

fn sanitize_unbounded_text(value: &str) -> String {
    sanitize_and_limit_text(value, None)
}

fn sanitize_and_limit_text(value: &str, max_chars: Option<usize>) -> String {
    let mut collapsed = String::new();
    let mut previous_was_space = false;

    for character in value.chars() {
        let normalized = if character.is_control() || character.is_whitespace() {
            ' '
        } else {
            character
        };

        if normalized == ' ' {
            if previous_was_space {
                continue;
            }
            previous_was_space = true;
        } else {
            previous_was_space = false;
        }

        collapsed.push(normalized);
    }

    let trimmed = collapsed.trim();
    match max_chars {
        Some(limit) => trimmed.chars().take(limit).collect(),
        None => trimmed.to_owned(),
    }
}

fn normalize_tipo(tipo_raw: &str) -> String {
    match tipo_raw.trim().to_ascii_lowercase().as_str() {
        "loop" => "loop".to_owned(),
        _ => DEFAULT_TIPO.to_owned(),
    }
}

fn normalize_primary_folder(folder_raw: &str) -> String {
    let normalized = folder_raw.trim().to_ascii_lowercase();
    PRIMARY_FOLDERS
        .iter()
        .find_map(|(candidate, label)| (*candidate == normalized).then(|| (*label).to_owned()))
        .unwrap_or_else(|| DEFAULT_FOLDER.to_owned())
}

fn normalize_secondary_folder(folder_raw: &str) -> String {
    let sanitized = sanitize_scalar_text(folder_raw, 60);
    if sanitized.is_empty() {
        DEFAULT_FOLDER.to_owned()
    } else {
        sanitized
    }
}

#[cfg(test)]
#[path = "json_repairer/tests.rs"]
mod tests;
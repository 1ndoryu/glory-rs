use validator::Validate;

use crate::errors::AppError;
use crate::models::{ListSamplesQuery, ListSamplesResponse, SamplesPagination};
use crate::repositories::{SampleListFilters, SampleRepository};

#[cfg(test)]
mod tests;

const DEFAULT_PAGE: i64 = 1;
const DEFAULT_PER_PAGE: i64 = 20;
const MAX_TAG_FILTERS: usize = 10;
const MAX_CREATOR_LENGTH: usize = 50;

/* [174A-44] Servicio de catálogo público de samples.
 * Centraliza la normalización de filtros para que el handler quede fino y el
 * repositorio solo reciba parámetros ya validados. Así evitamos duplicar reglas
 * de alias (`type`/`tipo`, `creator`/`creador`) en cada capa. */

pub struct SampleCatalogService;

impl SampleCatalogService {
    pub async fn list_public_samples(
        pool: &sqlx::PgPool,
        query: ListSamplesQuery,
    ) -> Result<ListSamplesResponse, AppError> {
        query
            .validate()
            .map_err(|error| AppError::Validation(error.to_string()))?;

        let filters = normalize_filters(query)?;
        let result = SampleRepository::list_public_samples(pool, &filters).await?;
        let pages = if result.total == 0 {
            0
        } else {
            (result.total + filters.per_page - 1) / filters.per_page
        };

        Ok(ListSamplesResponse {
            data: result.items,
            pagination: SamplesPagination {
                page: filters.page,
                per_page: filters.per_page,
                total: result.total,
                pages,
            },
        })
    }
}

fn normalize_filters(query: ListSamplesQuery) -> Result<SampleListFilters, AppError> {
    let creator = normalize_creator(query.creator)?;
    let tags = normalize_tags(query.tags)?;

    Ok(SampleListFilters {
        page: query.page.unwrap_or(DEFAULT_PAGE),
        per_page: query.per_page.unwrap_or(DEFAULT_PER_PAGE),
        bpm: query.bpm,
        music_key: normalize_music_key(query.key)?,
        sample_type: normalize_sample_type(query.sample_type)?,
        tags,
        premium: query.premium,
        creator,
    })
}

fn normalize_creator(raw: Option<String>) -> Result<Option<String>, AppError> {
    let Some(raw) = raw else {
        return Ok(None);
    };

    let creator = raw.trim();
    if creator.is_empty() {
        return Ok(None);
    }

    if creator.len() > MAX_CREATOR_LENGTH {
        return Err(AppError::Validation(format!(
            "creator no puede superar {MAX_CREATOR_LENGTH} caracteres"
        )));
    }

    Ok(Some(creator.to_string()))
}

fn normalize_tags(raw: Option<String>) -> Result<Vec<String>, AppError> {
    let Some(raw) = raw else {
        return Ok(Vec::new());
    };

    let mut tags = Vec::new();
    for tag in raw.split(',') {
        let normalized = tag.trim().to_ascii_lowercase();
        if normalized.is_empty() || tags.iter().any(|value| value == &normalized) {
            continue;
        }
        tags.push(normalized);
    }

    if tags.len() > MAX_TAG_FILTERS {
        return Err(AppError::Validation(format!(
            "tags soporta hasta {MAX_TAG_FILTERS} valores por consulta"
        )));
    }

    Ok(tags)
}

fn normalize_music_key(raw: Option<String>) -> Result<Option<String>, AppError> {
    let Some(raw) = raw else {
        return Ok(None);
    };

    let compact = raw.trim().replace(' ', "");
    if compact.is_empty() {
        return Ok(None);
    }

    let normalized = match compact.to_ascii_uppercase().as_str() {
        "A" => "A",
        "A#" => "A#",
        "BB" => "Bb",
        "B" => "B",
        "C" => "C",
        "C#" => "C#",
        "DB" => "Db",
        "D" => "D",
        "D#" => "D#",
        "EB" => "Eb",
        "E" => "E",
        "F" => "F",
        "F#" => "F#",
        "GB" => "Gb",
        "G" => "G",
        "G#" => "G#",
        "AB" => "Ab",
        _ => {
            return Err(AppError::Validation(
                "key inválida. Usa notas como C, F#, Bb o Eb".into(),
            ))
        }
    };

    Ok(Some(normalized.to_string()))
}

fn normalize_sample_type(raw: Option<String>) -> Result<Option<String>, AppError> {
    let Some(raw) = raw else {
        return Ok(None);
    };

    let compact = raw.trim().to_ascii_lowercase().replace([' ', '-'], "_");
    if compact.is_empty() {
        return Ok(None);
    }

    let normalized = match compact.as_str() {
        "loop" => "loop",
        "oneshot" | "one_shot" => "oneshot",
        "fx" => "fx",
        "vocal" | "vocals" => "vocal",
        "stem" | "stems" => "stem",
        "otro" | "other" => "otro",
        _ => {
            return Err(AppError::Validation(
                "type inválido. Usa loop, oneshot, fx, vocal, stem u otro".into(),
            ))
        }
    };

    Ok(Some(normalized.to_string()))
}

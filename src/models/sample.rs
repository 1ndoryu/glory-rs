/* [174A-28] Modelos para endpoints de samples / check-duplicate.
 * El legado usaba `hashParcial`; la migración sube el listón y responde sobre
 * SHA-256 exacto, pero mantiene contexto suficiente para que desktop/frontend
 * decidan si reutilizan el sample ya existente. */

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Respuesta de `POST /api/samples/check-duplicate`.
#[derive(Debug, Serialize, ToSchema)]
pub struct CheckDuplicateResponse {
    /// SHA-256 hex (64 chars) del archivo recibido o del hash precomputado.
    pub audio_hash: String,
    /// True si ya existe un sample con ese hash y no está marcado como eliminado.
    pub possible_duplicate: bool,
    /// ID del sample existente (solo si `possible_duplicate == true`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_id: Option<i32>,
    /// True si el sample encontrado pertenece al mismo usuario autenticado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_owner: Option<bool>,
    /// Título del sample existente, útil para UI/desktop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Mensaje orientado a UX para el cliente consumidor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Bytes leídos durante el cálculo (informativo). 0 si solo se mandó el hash.
    pub bytes_hashed: u64,
}

/// Request alternativo: el cliente ya calculó el hash y solo quiere consultar.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CheckDuplicateRequest {
    /// SHA-256 hex precomputado por el cliente (64 chars).
    pub audio_hash: String,
}

/// Schema documental para `multipart/form-data` en `POST /api/samples/upload`.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UploadSampleRequestDoc {
    #[schema(value_type = String, format = Binary)]
    pub audio: Vec<u8>,
    pub titulo: Option<String>,
    pub contenido: Option<String>,
    /// JSON array (`["tag1","tag2"]`) o CSV (`tag1,tag2`).
    pub tags: Option<String>,
    pub permitir_descarga: Option<bool>,
    pub licencia_libre: Option<bool>,
    pub es_premium: Option<bool>,
    pub mostrar_en_comunidad: Option<bool>,
    pub sync_upload: Option<bool>,
    pub origen_subida: Option<String>,
    pub precio: Option<f64>,
}

/// Respuesta de `POST /api/samples/upload`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UploadSampleResponse {
    pub ok: bool,
    pub sample_id: i32,
    pub id_corto: String,
    pub slug: String,
    pub url: String,
    pub estado: String,
}

/// Query params de `GET /api/samples`.
#[derive(Debug, Clone, Deserialize, Validate, ToSchema, Default)]
pub struct ListSamplesQuery {
    #[validate(range(min = 1, max = 10_000))]
    pub page: Option<i64>,
    #[validate(range(min = 1, max = 100))]
    pub per_page: Option<i64>,
    #[validate(range(min = 1, max = 400))]
    pub bpm: Option<i32>,
    pub key: Option<String>,
    #[serde(rename = "type", alias = "tipo")]
    pub sample_type: Option<String>,
    /// CSV (`trap,drill`) o valor único. También acepta alias legado `tag`.
    #[serde(alias = "tag")]
    pub tags: Option<String>,
    #[serde(alias = "es_premium")]
    pub premium: Option<bool>,
    #[serde(alias = "creador")]
    pub creator: Option<String>,
}

/// Resumen público del creador incluido en `GET /api/samples`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SampleCreatorSummary {
    pub id: i32,
    pub username: String,
    pub nombre_visible: Option<String>,
    pub avatar_url: Option<String>,
    pub verificado: bool,
}

/// Item resumido del catálogo público de samples.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SampleSummary {
    pub id: i32,
    pub id_corto: Option<String>,
    pub slug: String,
    pub titulo: String,
    pub descripcion: String,
    pub bpm: Option<i32>,
    #[serde(rename = "key")]
    pub music_key: Option<String>,
    pub escala: Option<String>,
    pub duracion: f32,
    pub formato: String,
    pub tags: Vec<String>,
    pub tipo: String,
    pub es_premium: bool,
    pub precio: Option<f64>,
    pub verificado: bool,
    pub ruta_preview: Option<String>,
    pub ruta_waveform: Option<String>,
    pub imagen_url: Option<String>,
    pub total_descargas: i32,
    pub total_likes: i32,
    pub total_reproducciones: i32,
    pub total_comentarios: i32,
    pub publicado_at: Option<chrono::DateTime<chrono::Utc>>,
    pub creador: SampleCreatorSummary,
}

/// Metadatos de paginación para listados de samples.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SamplesPagination {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub pages: i64,
}

/// Respuesta de `GET /api/samples`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ListSamplesResponse {
    pub data: Vec<SampleSummary>,
    pub pagination: SamplesPagination,
}

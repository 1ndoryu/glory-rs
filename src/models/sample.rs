/* [174A-28] Modelos para endpoints de samples / check-duplicate.
 * El legado usaba `hashParcial`; la migración sube el listón y responde sobre
 * SHA-256 exacto, pero mantiene contexto suficiente para que desktop/frontend
 * decidan si reutilizan el sample ya existente. */

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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

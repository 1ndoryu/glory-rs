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
    /* [166A-6] Campos relacionales y portada. */
    /// ID de la cancion de origen (para "subir sample de esta cancion").
    pub cancion_origen_id: Option<i32>,
    /// ID de la relacion de sampleo a vincular.
    pub relacion_id: Option<i32>,
    /// Lado de la relacion: `fuente` o `destino`.
    pub lado_relacion: Option<String>,
    /// Segundo de inicio del sample en la cancion.
    pub inicio_segundos: Option<i32>,
    /// Tipo de elemento sampleado (hook_riff, vocals_lyrics, drums, etc.).
    pub tipo_elemento: Option<String>,
    /// Imagen de portada del sample.
    #[schema(value_type = String, format = Binary)]
    pub portada: Option<Vec<u8>>,
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
    #[serde(alias = "q", alias = "busqueda")]
    pub search: Option<String>,
    #[serde(alias = "busqueda_norm")]
    pub search_normalized: Option<String>,
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
    #[schema(value_type = Object)]
    pub metadata: serde_json::Value,
    pub creador: SampleCreatorSummary,
    /* [296A-1] Flag pre-cargado: sample guardado en al menos 1 coleccion del viewer.
     * Se computa via EXISTS subquery solo cuando hay usuario autenticado. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ya_guardado_en_coleccion: Option<bool>,
    /* [296A-3] Reaccion del usuario autenticado sobre este sample.
     * liked = true si reaccion positiva (like/encanta). reaccion = tipo exacto.
     * Se omiten en la respuesta JSON si el viewer es anonimo (None). */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaccion: Option<String>,
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

/// Query params de `GET /api/samples/{id}/similar`.
#[derive(Debug, Clone, Deserialize, Validate, ToSchema, Default)]
pub struct SimilarSamplesQuery {
    #[serde(alias = "limite")]
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<i64>,
}

/// Respuesta de `GET /api/samples/{id}/similar`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SimilarSamplesResponse {
    pub data: Vec<SampleSummary>,
}

/* [166A-1] QQ51: Datos enriquecidos de canción origen para un sample.
 * Se popula via LEFT JOIN con canciones + artistas_musicales.
 * Solo presente si el sample es un recorte (cancion_origen_id != NULL). */
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CancionOrigenResumen {
    pub id: i32,
    pub titulo: String,
    pub slug: String,
    pub artista: Option<String>,
    pub whosampled_url: Option<String>,
    pub bpm: Option<i16>,
}

/* [166A-1] QQ117: Metadata de extracción para un sample.
 * Se popula via LEFT JOIN LATERAL con cola_extraccion_samples.
 * Contiene datos de la fuente (YouTube/Spotify), timestamps de corte,
 * BPM detectado y los campos QQ23 del JSONB metadata_extraccion. */
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExtraccionSampleResponse {
    pub youtube_id: Option<String>,
    pub spotify_id: Option<String>,
    pub timing_inicio_seg: Option<i32>,
    pub bpm_detectado: Option<i32>,
    pub duracion_compas_seg: Option<f64>,
    pub compas_inicio_seg: Option<f64>,
    pub compas_fin_seg: Option<f64>,
    pub lado: Option<String>,
    pub estado: Option<String>,
    pub ruta_audio_extraido: Option<String>,
    pub tiene_audio_completo: bool,
    /* QQ23: Campos extraidos del JSONB metadata_extraccion */
    pub fuente_url: Option<String>,
    pub fuente_titulo: Option<String>,
    pub fuente_artista: Option<String>,
    pub descarga_metodo: Option<String>,
    pub sampleo_fuente_titulo: Option<String>,
    pub sampleo_fuente_artista: Option<String>,
    pub sampleo_destino_titulo: Option<String>,
    pub sampleo_destino_artista: Option<String>,
    pub fuente_slug: Option<String>,
    pub fuente_album: Option<String>,
    pub destino_slug: Option<String>,
    pub destino_album: Option<String>,
    pub votos_total: Option<i32>,
    pub tipo_elemento: Option<String>,
    pub recorte_por_compas: Option<String>,
    pub duracion_extraida: Option<f64>,
    pub formato_extraido: Option<String>,
    pub tamano_bytes: Option<i64>,
}

/// Respuesta de `GET /api/samples/{slug}` y `GET /api/samples/random`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[allow(clippy::struct_excessive_bools)]
pub struct SampleDetailResponse {
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
    pub tamano: i64,
    pub tags: Vec<String>,
    pub tipo: String,
    pub estado: String,
    pub es_premium: bool,
    pub precio: Option<f64>,
    #[schema(value_type = Object)]
    pub metadata: serde_json::Value,
    pub ruta_preview: Option<String>,
    pub ruta_waveform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruta_original: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruta_optimizada: Option<String>,
    pub permitir_descarga: bool,
    pub licencia_libre: bool,
    pub imagen_url: Option<String>,
    pub total_descargas: i32,
    pub total_likes: i32,
    pub total_reproducciones: i32,
    pub total_comentarios: i32,
    pub audio_hash: Option<String>,
    pub verificado: bool,
    pub mostrar_en_comunidad: bool,
    pub publicado_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub cancion_origen_id: Option<i32>,
    pub relacion_sampleo_id: Option<i32>,

    /* [166A-1] QQ51: Datos enriquecidos de canción origen */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancion_origen: Option<CancionOrigenResumen>,

    /* [166A-1] QQ117: Metadata de extracción */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extraccion: Option<ExtraccionSampleResponse>,

    /* [166A-1] URL de WhoSampled construida desde relaciones_sample.whosampled_id */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whosampled_url: Option<String>,

    pub creador: SampleCreatorSummary,
}

/// Payload parcial para `PATCH /api/samples/{slug}`.
#[derive(Debug, Clone, Deserialize, Validate, ToSchema, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct UpdateSampleRequest {
    #[validate(length(min = 1, max = 200))]
    pub titulo: Option<String>,
    #[validate(length(max = 5_000))]
    pub descripcion: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "type", alias = "tipo")]
    pub sample_type: Option<String>,
    #[serde(alias = "esPremium")]
    pub es_premium: Option<bool>,
    #[validate(range(min = 0.0, max = 9_999.0))]
    pub precio: Option<f64>,
    #[serde(alias = "permitirDescarga")]
    pub permitir_descarga: Option<bool>,
    #[serde(alias = "licenciaLibre")]
    pub licencia_libre: Option<bool>,
    #[serde(alias = "mostrarEnComunidad")]
    pub mostrar_en_comunidad: Option<bool>,
    #[serde(alias = "imagenUrl")]
    pub imagen_url: Option<String>,
}

/// Respuesta de `DELETE /api/samples/{slug}`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeleteSampleResponse {
    pub ok: bool,
    pub eliminado: bool,
}

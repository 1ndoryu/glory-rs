use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

/* [174A-25] DTOs admin: suspension/reactivacion/eliminacion + bloqueo entre usuarios. */

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SuspendUserRequest {
    #[validate(length(min = 3, max = 500))]
    pub razon: String,
    /// Si se omite, suspension indefinida.
    pub hasta: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct DeleteUserRequest {
    /// Dias hasta eliminacion definitiva (default 30).
    pub dias_gracia: Option<i32>,
}

#[derive(Debug, Deserialize, Validate, ToSchema, Default)]
pub struct BlockUserRequest {
    #[validate(length(max = 255))]
    pub razon: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct AdminSummaryStats {
    pub total_usuarios: i64,
    pub total_samples: i64,
    pub total_descargas: i64,
    pub total_publicaciones: i64,
    pub pendientes_moderacion: i64,
    pub reportes_pendientes: i64,
    pub usuarios_pro: i64,
    pub usuarios_premium: i64,
    pub samples_semana: i64,
    pub registros_semana: i64,
}

#[derive(Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct AdminActivityPoint {
    pub fecha: String,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AdminActivityResponse {
    pub registros: Vec<AdminActivityPoint>,
    pub uploads: Vec<AdminActivityPoint>,
    pub descargas: Vec<AdminActivityPoint>,
}

#[derive(Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct AdminUserListItem {
    pub id: i32,
    pub username: String,
    pub nombre_visible: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub plan: String,
    pub rol: String,
    pub verificado: bool,
    pub ban_hasta: Option<DateTime<Utc>>,
    pub estado: String,
    pub suspendido_hasta: Option<DateTime<Utc>>,
    pub suspension_razon: Option<String>,
    pub sera_eliminado_en: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub total_samples: i64,
    pub total_descargas: i64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AdminUsersResponse {
    pub data: Vec<AdminUserListItem>,
    pub total: i64,
    pub page: i64,
}

#[derive(Debug, Clone, Deserialize, ToSchema, Default)]
pub struct AdminUserUpdateRequest {
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub rol: Option<String>,
    #[serde(default)]
    pub verificado: Option<bool>,
    #[serde(default)]
    pub ban_hasta: Option<Option<DateTime<Utc>>>,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct AdminUserSuspendRequest {
    #[validate(range(min = 1, max = 8_760))]
    pub horas: i32,
    #[validate(length(min = 3, max = 500))]
    pub razon: String,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema, Default)]
pub struct AdminUserDeleteRequest {
    #[validate(length(max = 500))]
    pub razon: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AdminOkResponse {
    pub ok: bool,
}

#[derive(Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct AdminScraperItem {
    pub id: i32,
    pub url: String,
    pub tipo_pagina: String,
    pub estado: String,
    pub intentos: i32,
    pub bytes_descargados: i64,
    pub error_mensaje: Option<String>,
    pub re_scrapeable: bool,
    pub veces_rescrapeado: i32,
    pub procesado_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AdminScrapersResponse {
    pub data: Vec<AdminScraperItem>,
    pub total: i64,
    pub page: i64,
    #[serde(rename = "estadosCuenta")]
    pub estados_cuenta: BTreeMap<String, i64>,
}

#[derive(Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct AdminExtractionQueueItem {
    pub id: i32,
    pub relacion_id: i32,
    pub youtube_id: Option<String>,
    pub spotify_id: Option<String>,
    pub estado: String,
    pub intentos: i32,
    pub lado: String,
    pub error_mensaje: Option<String>,
    pub sample_id: Option<i32>,
    pub timing_inicio_seg: Option<i32>,
    pub compas_inicio_seg: Option<f64>,
    pub compas_fin_seg: Option<f64>,
    pub bpm_detectado: Option<i32>,
    pub procesado_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub proximo_intento_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AdminExtractionQueueResponse {
    pub data: Vec<AdminExtractionQueueItem>,
    pub total: i64,
    pub page: i64,
    #[serde(rename = "estadosCuenta")]
    pub estados_cuenta: BTreeMap<String, i64>,
}

#[derive(Debug, Clone, Deserialize, IntoParams, ToSchema)]
pub struct AdminActivityQuery {
    pub dias: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, IntoParams, ToSchema)]
pub struct AdminUsersQuery {
    pub page: Option<i64>,
    pub busqueda: Option<String>,
    pub plan: Option<String>,
    pub orden: Option<String>,
}

#[derive(Debug, Clone, Deserialize, IntoParams, ToSchema)]
pub struct AdminScrapersQuery {
    pub page: Option<i64>,
    pub busqueda: Option<String>,
    pub estado: Option<String>,
    pub sort_col: Option<String>,
    pub sort_dir: Option<String>,
}

#[derive(Debug, Clone, Deserialize, IntoParams, ToSchema)]
pub struct AdminExtractionQueueQuery {
    pub page: Option<i64>,
    pub busqueda: Option<String>,
    pub estado: Option<String>,
    pub sort_col: Option<String>,
    pub sort_dir: Option<String>,
    pub lado: Option<String>,
}

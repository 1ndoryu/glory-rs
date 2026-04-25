/* [254A-4] Endpoints admin/moderacion — paridad con ModeracionController.php legacy.
 * GET /admin/moderacion?page=&reportes_page=&reportes_limit=
 *     - publicaciones pendientes (paginado 20/pag)
 *     - articulos pendientes (paginado 20/pag)
 *     - reportes pendientes (paginado configurable)
 *     - reportesTotal (count global)
 * GET /admin/moderacion/historial?dias=
 *     - publicaciones moderadas (no pendientes) en los ultimos N dias
 *
 * Devuelve `{ data: { ... } }` para alinear con la convencion legacy donde el frontend
 * extrae `respuesta.data` y rellena `imagenes/avatar` con fallbacks vacios.
 */

use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};

use crate::errors::AppError;
use crate::middleware::CurrentUser;
use crate::AppState;

const PAGE_SIZE: i64 = 20;
const HISTORIAL_DIAS_DEFAULT: i32 = 7;
const HISTORIAL_DIAS_MAX: i32 = 90;

#[derive(Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct PublicacionPendiente {
    pub id: i32,
    pub autor_id: i32,
    pub tipo: String,
    pub contenido: String,
    pub imagenes: Vec<String>,
    pub samples_adjuntos: Vec<i32>,
    pub moderacion_estado: Option<String>,
    pub moderacion_detalle: Option<serde_json::Value>,
    pub moderacion_razon: Option<String>,
    pub created_at: DateTime<Utc>,
    pub autor_username: String,
    pub autor_nombre: String,
    pub autor_avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct ArticuloPendiente {
    pub id: i32,
    pub titulo: String,
    pub slug: String,
    pub extracto: String,
    pub categoria: String,
    pub portada_url: Option<String>,
    pub moderacion_estado: String,
    pub moderacion_razon: Option<String>,
    pub created_at: DateTime<Utc>,
    pub autor_id: i32,
    pub autor_username: String,
    pub autor_nombre: String,
    pub autor_avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct ReportePendiente {
    pub id: i32,
    pub tipo: String,
    pub target_id: i32,
    pub razon: String,
    pub detalles: Option<String>,
    pub estado: String,
    pub created_at: DateTime<Utc>,
    pub reportador_id: i32,
    pub reportador_username: String,
    pub reportado_id: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct ListarQuery {
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub reportes_page: Option<i64>,
    #[serde(default)]
    pub reportes_limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListarData {
    pub publicaciones: Vec<PublicacionPendiente>,
    pub articulos: Vec<ArticuloPendiente>,
    pub reportes: Vec<ReportePendiente>,
    #[serde(rename = "reportesTotal")]
    pub reportes_total: i64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListarResponse {
    pub data: ListarData,
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct HistorialQuery {
    #[serde(default)]
    pub dias: Option<i32>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct HistorialData {
    pub publicaciones: Vec<PublicacionPendiente>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct HistorialResponse {
    pub data: HistorialData,
}

#[utoipa::path(get, path = "/api/admin/moderacion", tag = "admin",
    params(ListarQuery),
    security(("bearer_auth" = [])),
    responses((status = 200, body = ListarResponse), (status = 403)))]
pub async fn listar(
    State(state): State<AppState>,
    user: CurrentUser,
    Query(query): Query<ListarQuery>,
) -> Result<Json<ListarResponse>, AppError> {
    user.require_admin()?;
    let page = query.page.unwrap_or(1).max(1);
    let offset = (page - 1) * PAGE_SIZE;
    let reportes_page = query.reportes_page.unwrap_or(1).max(1);
    let reportes_limit = query.reportes_limit.unwrap_or(20).clamp(1, 50);
    let reportes_offset = (reportes_page - 1) * reportes_limit;

    let publicaciones = sqlx::query_as::<_, PublicacionPendiente>(
        r"SELECT p.id, p.autor_id, p.tipo, p.contenido,
                  COALESCE(p.imagenes, '{}')         AS imagenes,
                  COALESCE(p.samples_adjuntos, '{}') AS samples_adjuntos,
                  p.moderacion_estado, p.moderacion_detalle, p.moderacion_razon, p.created_at,
                  u.username AS autor_username,
                  u.nombre_visible AS autor_nombre,
                  u.avatar_url AS autor_avatar
             FROM publicaciones p
             JOIN usuarios_ext u ON u.id = p.autor_id
            WHERE p.moderacion_estado IN ('pendiente', 'revision')
              AND p.eliminado_en IS NULL
            ORDER BY p.created_at DESC
            LIMIT $1 OFFSET $2",
    )
    .bind(PAGE_SIZE)
    .bind(offset)
    .fetch_all(&state.pool)
    .await?;

    let articulos = sqlx::query_as::<_, ArticuloPendiente>(
        r"SELECT a.id, a.titulo, a.slug, a.extracto, a.categoria, a.portada_url,
                  a.moderacion_estado, a.moderacion_razon, a.created_at,
                  a.autor_id,
                  u.username AS autor_username,
                  u.nombre_visible AS autor_nombre,
                  u.avatar_url AS autor_avatar
             FROM articulos a
             JOIN usuarios_ext u ON u.id = a.autor_id
            WHERE a.moderacion_estado IN ('pendiente', 'revision')
              AND a.eliminado_en IS NULL
            ORDER BY a.created_at DESC
            LIMIT $1 OFFSET $2",
    )
    .bind(PAGE_SIZE)
    .bind(offset)
    .fetch_all(&state.pool)
    .await?;

    let reportes = sqlx::query_as::<_, ReportePendiente>(
        r"SELECT r.id, r.tipo, r.target_id, r.razon, r.detalles, r.estado, r.created_at,
                  r.reportador_id,
                  u.username AS reportador_username,
                  r.reportado_id
             FROM reportes r
             JOIN usuarios_ext u ON u.id = r.reportador_id
            WHERE r.estado = 'pendiente'
            ORDER BY r.created_at DESC
            LIMIT $1 OFFSET $2",
    )
    .bind(reportes_limit)
    .bind(reportes_offset)
    .fetch_all(&state.pool)
    .await?;

    let reportes_total: (i64,) =
        sqlx::query_as(r"SELECT COUNT(*)::bigint FROM reportes WHERE estado = 'pendiente'")
            .fetch_one(&state.pool)
            .await?;

    Ok(Json(ListarResponse {
        data: ListarData {
            publicaciones,
            articulos,
            reportes,
            reportes_total: reportes_total.0,
        },
    }))
}

#[utoipa::path(get, path = "/api/admin/moderacion/historial", tag = "admin",
    params(HistorialQuery),
    security(("bearer_auth" = [])),
    responses((status = 200, body = HistorialResponse), (status = 403)))]
pub async fn historial(
    State(state): State<AppState>,
    user: CurrentUser,
    Query(query): Query<HistorialQuery>,
) -> Result<Json<HistorialResponse>, AppError> {
    user.require_admin()?;
    let dias = query
        .dias
        .unwrap_or(HISTORIAL_DIAS_DEFAULT)
        .clamp(1, HISTORIAL_DIAS_MAX);

    let publicaciones = sqlx::query_as::<_, PublicacionPendiente>(
        r"SELECT p.id, p.autor_id, p.tipo, p.contenido,
                  COALESCE(p.imagenes, '{}')         AS imagenes,
                  COALESCE(p.samples_adjuntos, '{}') AS samples_adjuntos,
                  p.moderacion_estado, p.moderacion_detalle, p.moderacion_razon, p.created_at,
                  u.username AS autor_username,
                  u.nombre_visible AS autor_nombre,
                  u.avatar_url AS autor_avatar
             FROM publicaciones p
             JOIN usuarios_ext u ON u.id = p.autor_id
            WHERE p.moderacion_estado IS NOT NULL
              AND p.moderacion_estado <> 'pendiente'
              AND p.created_at >= NOW() - make_interval(days => $1)
            ORDER BY p.created_at DESC
            LIMIT 100",
    )
    .bind(dias)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(HistorialResponse {
        data: HistorialData { publicaciones },
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/admin/moderacion", get(listar))
        .route("/admin/moderacion/historial", get(historial))
}

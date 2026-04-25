/* [254A-4] Endpoints admin/contribuciones — paridad con ContribucionesController.php legacy.
 * GET /admin/contribuciones?page=&limit= — listar contribuciones pendientes (admin).
 *
 * Frontend (apiContribuciones.ts) consume `{ ok, items, total, page, limit }`. El stub
 * wpJsonStub convierte snake_case -> camelCase, asi que aqui usamos los nombres legacy
 * tal cual llegan al frontend.
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

#[derive(Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct ContribucionPendiente {
    pub id: i32,
    pub contribuidor_id: i32,
    pub contribuidor_username: String,
    pub cancion_destino_id: Option<i32>,
    pub cancion_fuente_id: Option<i32>,
    pub cancion_nueva_titulo: Option<String>,
    pub cancion_nueva_artista: Option<String>,
    pub cancion_nueva_youtube_url: Option<String>,
    pub cancion_nueva_lado: Option<String>,
    pub tipo_relacion: Option<String>,
    pub tipo_elemento: Option<String>,
    pub tipo_contribucion: Option<String>,
    pub relacion_existente_id: Option<i32>,
    pub cambios_propuestos: Option<serde_json::Value>,
    pub estado: String,
    pub moderador_nota: Option<String>,
    pub created_at: DateTime<Utc>,
    pub resuelto_at: Option<DateTime<Utc>>,
    pub destino_titulo: Option<String>,
    pub fuente_titulo: Option<String>,
    pub cancion_destino_slug: Option<String>,
    pub cancion_fuente_slug: Option<String>,
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct ListarQuery {
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListarResponse {
    pub ok: bool,
    pub items: Vec<ContribucionPendiente>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

#[utoipa::path(get, path = "/api/admin/contribuciones", tag = "admin",
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
    let limit = query.limit.unwrap_or(20).clamp(1, 50);
    let offset = (page - 1) * limit;

    let items = sqlx::query_as::<_, ContribucionPendiente>(
        r"SELECT cp.id,
                  cp.contribuidor_id,
                  u.username AS contribuidor_username,
                  cp.cancion_destino_id,
                  cp.cancion_fuente_id,
                  cp.cancion_nueva_titulo,
                  cp.cancion_nueva_artista,
                  cp.cancion_nueva_youtube_url,
                  cp.cancion_nueva_lado,
                  cp.tipo_relacion,
                  cp.tipo_elemento,
                  cp.tipo_contribucion,
                  cp.relacion_existente_id,
                  cp.cambios_propuestos,
                  cp.estado,
                  cp.moderador_nota,
                  cp.created_at,
                  cp.resuelto_at,
                  cd.titulo AS destino_titulo,
                  cf.titulo AS fuente_titulo,
                  cd.slug   AS cancion_destino_slug,
                  cf.slug   AS cancion_fuente_slug
             FROM contribuciones_pendientes cp
             JOIN usuarios_ext u ON u.id = cp.contribuidor_id
             LEFT JOIN canciones cd ON cd.id = cp.cancion_destino_id
             LEFT JOIN canciones cf ON cf.id = cp.cancion_fuente_id
            WHERE cp.estado = 'pendiente'
            ORDER BY cp.created_at DESC
            LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        r"SELECT COUNT(*)::bigint FROM contribuciones_pendientes WHERE estado = 'pendiente'",
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ListarResponse {
        ok: true,
        items,
        total: total.0,
        page,
        limit,
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/admin/contribuciones", get(listar))
}

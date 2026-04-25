/* [254A-4] Endpoints admin/duplicados — paridad con DuplicadosController.php legacy.
 * GET /admin/duplicados?estado=&tipo=&pagina=&porPagina= — listar duplicados con joins a samples
 * GET /admin/duplicados/contar — contador de pendientes (badge nav)
 *
 * Devuelve los samples original/duplicado con titulo, slug, creador, ruta_preview, audio_hash.
 * El frontend usa estos campos para mostrar el preview de audio y agrupar por hash.
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

const ESTADOS_VALIDOS: [&str; 4] = ["pendiente", "aprobado", "rechazado", "fusionado"];
const TIPOS_VALIDOS: [&str; 3] = ["cross_usuario", "mismo_usuario", "backfill"];

#[derive(Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct DuplicadoFila {
    pub id: i32,
    pub tipo: String,
    pub estado: String,
    pub created_at: DateTime<Utc>,

    pub original_id: i32,
    pub original_titulo: String,
    pub original_subido_at: DateTime<Utc>,
    pub original_ruta_preview: Option<String>,
    pub original_ruta_waveform: Option<String>,
    pub original_creador: String,
    pub original_creador_id: i32,
    pub original_slug: Option<String>,
    pub original_hash: Option<String>,

    pub duplicado_id: i32,
    pub duplicado_titulo: String,
    pub duplicado_subido_at: DateTime<Utc>,
    pub duplicado_ruta_preview: Option<String>,
    pub duplicado_ruta_waveform: Option<String>,
    pub duplicado_creador: String,
    pub duplicado_creador_id: i32,
    pub duplicado_slug: Option<String>,
    pub duplicado_hash: Option<String>,
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct ListarQuery {
    #[serde(default)]
    pub estado: Option<String>,
    #[serde(default)]
    pub tipo: Option<String>,
    #[serde(default)]
    pub pagina: Option<i64>,
    #[serde(default, rename = "porPagina")]
    pub por_pagina: Option<i64>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListarResponse {
    pub ok: bool,
    pub total: i64,
    pub pagina: i64,
    #[serde(rename = "porPagina")]
    pub por_pagina: i64,
    pub duplicados: Vec<DuplicadoFila>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ContarResponse {
    pub ok: bool,
    pub total: i64,
}

#[utoipa::path(get, path = "/api/admin/duplicados", tag = "admin",
    params(ListarQuery),
    security(("bearer_auth" = [])),
    responses((status = 200, body = ListarResponse), (status = 403)))]
pub async fn listar(
    State(state): State<AppState>,
    user: CurrentUser,
    Query(query): Query<ListarQuery>,
) -> Result<Json<ListarResponse>, AppError> {
    user.require_admin()?;
    let pagina = query.pagina.unwrap_or(1).max(1);
    let por_pagina = query.por_pagina.unwrap_or(20).clamp(1, 50);
    let offset = (pagina - 1) * por_pagina;

    let estado = query
        .estado
        .as_deref()
        .filter(|e| ESTADOS_VALIDOS.contains(e))
        .unwrap_or("pendiente")
        .to_string();
    let tipo = match query.tipo.as_deref() {
        Some(t) if TIPOS_VALIDOS.contains(&t) => Some(t.to_string()),
        Some("") | None => None,
        Some(other) => {
            return Err(AppError::BadRequest(format!(
                "tipo invalido: '{other}'. Validos: cross_usuario, mismo_usuario, backfill"
            )))
        }
    };

    let duplicados = sqlx::query_as::<_, DuplicadoFila>(
        r"SELECT d.id,
                  d.tipo,
                  d.estado,
                  d.created_at,
                  so.id              AS original_id,
                  so.titulo          AS original_titulo,
                  so.created_at      AS original_subido_at,
                  so.ruta_preview    AS original_ruta_preview,
                  so.ruta_waveform   AS original_ruta_waveform,
                  uo.username        AS original_creador,
                  uo.id              AS original_creador_id,
                  so.slug            AS original_slug,
                  so.audio_hash      AS original_hash,
                  sd.id              AS duplicado_id,
                  sd.titulo          AS duplicado_titulo,
                  sd.created_at      AS duplicado_subido_at,
                  sd.ruta_preview    AS duplicado_ruta_preview,
                  sd.ruta_waveform   AS duplicado_ruta_waveform,
                  ud.username        AS duplicado_creador,
                  ud.id              AS duplicado_creador_id,
                  sd.slug            AS duplicado_slug,
                  sd.audio_hash      AS duplicado_hash
             FROM duplicados_pendientes d
             JOIN samples so       ON so.id = d.sample_original_id
             JOIN usuarios_ext uo  ON uo.id = so.creador_id
             JOIN samples sd       ON sd.id = d.sample_duplicado_id
             JOIN usuarios_ext ud  ON ud.id = sd.creador_id
            WHERE d.estado = $1
              AND ($2::text IS NULL OR d.tipo = $2)
            ORDER BY d.created_at DESC
            LIMIT $3 OFFSET $4",
    )
    .bind(&estado)
    .bind(tipo.as_deref())
    .bind(por_pagina)
    .bind(offset)
    .fetch_all(&state.pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        r"SELECT COUNT(*)::bigint FROM duplicados_pendientes WHERE estado = 'pendiente'",
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ListarResponse {
        ok: true,
        total: total.0,
        pagina,
        por_pagina,
        duplicados,
    }))
}

#[utoipa::path(get, path = "/api/admin/duplicados/contar", tag = "admin",
    security(("bearer_auth" = [])),
    responses((status = 200, body = ContarResponse), (status = 403)))]
pub async fn contar(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<ContarResponse>, AppError> {
    user.require_admin()?;
    let total: (i64,) = sqlx::query_as(
        r"SELECT COUNT(*)::bigint FROM duplicados_pendientes WHERE estado = 'pendiente'",
    )
    .fetch_one(&state.pool)
    .await?;
    Ok(Json(ContarResponse {
        ok: true,
        total: total.0,
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/admin/duplicados", get(listar))
        .route("/admin/duplicados/contar", get(contar))
}

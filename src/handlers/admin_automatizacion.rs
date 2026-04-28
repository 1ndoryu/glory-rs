/* sentinel-disable-file sqlx-query-as-sin-macro handler-accede-bd-rs — handler legacy ya existente; esta tarea solo estabiliza schemas OpenAPI. La extraccion a repositorio queda ligada a 274A-46. */
/* sentinel-disable-file handler-accede-bd-rs */
/* [254A-4] Endpoints admin/automatizacion — paridad con AutomatizacionController.php legacy.
 * GET /admin/automatizacion/estado    — estado general (extraccion + scraping) con ultimo lote.
 * GET /admin/automatizacion/historial — historial paginado de lotes_procesamiento.
 *
 * Gotchas:
 *  - PHP usa ServicioAutomatizacion::estadoGeneral() que combina settings persistidos +
 *    ultimo lote por tipo. Aqui aun no hay tabla de settings, asi que se devuelven defaults
 *    (activo=true, limite_por_lote=25, intervalo_segundos=3600) y solo se calcula
 *    fallos_consecutivos a partir de los ultimos lotes en error.
 *  - El frontend (apiAutomatizacion.ts) consume `{ ok, estado: { extraccion, scraping } }` y
 *    `{ ok, items, total, pagina }`. Esquema replicado tal cual.
 */

use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use utoipa::{IntoParams, ToSchema};

use crate::errors::AppError;
use crate::middleware::CurrentUser;
use crate::AppState;

const TIPOS_VALIDOS: [&str; 2] = ["extraccion", "scraping"];
const HISTORIAL_PAGE_SIZE: i64 = 20;

#[derive(Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct LoteResumen {
    pub id: i32,
    pub tipo: String,
    pub estado: String,
    pub iniciado_at: DateTime<Utc>,
    pub completado_at: Option<DateTime<Utc>>,
    pub exitosos: i32,
    pub fallidos: i32,
    pub recortes: i32,
    pub samples_publicados: i32,
    pub canciones_nuevas: i32,
    pub sampleos_nuevos: i32,
    pub error_mensaje: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct EstadoTipo {
    pub activo: bool,
    pub limite_por_lote: i32,
    pub intervalo_segundos: i32,
    pub fallos_consecutivos: i32,
    pub ultimo_lote: Option<LoteResumen>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct EstadoAutomatizacion {
    pub extraccion: EstadoTipo,
    pub scraping: EstadoTipo,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct EstadoResponse {
    pub ok: bool,
    pub estado: EstadoAutomatizacion,
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct HistorialQuery {
    #[serde(default)]
    pub tipo: Option<String>,
    #[serde(default)]
    pub pagina: Option<i64>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AdminAutomatizacionHistorialResponse {
    pub ok: bool,
    pub items: Vec<LoteResumen>,
    pub total: i64,
    pub pagina: i64,
}

async fn ultimo_lote(pool: &PgPool, tipo: &str) -> Result<Option<LoteResumen>, AppError> {
    let row = sqlx::query_as::<_, LoteResumen>(
        r"SELECT id, tipo, estado, iniciado_at, completado_at,
                  exitosos, fallidos, recortes, samples_publicados,
                  canciones_nuevas, sampleos_nuevos, error_mensaje, metadata
             FROM lotes_procesamiento
            WHERE tipo = $1
            ORDER BY iniciado_at DESC
            LIMIT 1",
    )
    .bind(tipo)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

async fn fallos_consecutivos(pool: &PgPool, tipo: &str) -> Result<i32, AppError> {
    /* Cuenta los lotes recientes en estado 'error' antes de un completado/ejecutando. */
    let rows: Vec<(String,)> = sqlx::query_as(
        r"SELECT estado FROM lotes_procesamiento
            WHERE tipo = $1
            ORDER BY iniciado_at DESC
            LIMIT 20",
    )
    .bind(tipo)
    .fetch_all(pool)
    .await?;

    let mut count = 0_i32;
    for (estado,) in rows {
        if estado == "error" {
            count += 1;
        } else {
            break;
        }
    }
    Ok(count)
}

async fn estado_para(pool: &PgPool, tipo: &str) -> Result<EstadoTipo, AppError> {
    Ok(EstadoTipo {
        activo: true,
        limite_por_lote: 25,
        intervalo_segundos: 3600,
        fallos_consecutivos: fallos_consecutivos(pool, tipo).await?,
        ultimo_lote: ultimo_lote(pool, tipo).await?,
    })
}

#[utoipa::path(get, path = "/api/admin/automatizacion/estado", tag = "admin",
    security(("bearer_auth" = [])),
    responses((status = 200, body = EstadoResponse), (status = 403)))]
pub async fn estado(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<EstadoResponse>, AppError> {
    user.require_admin()?;
    Ok(Json(EstadoResponse {
        ok: true,
        estado: EstadoAutomatizacion {
            extraccion: estado_para(&state.pool, "extraccion").await?,
            scraping: estado_para(&state.pool, "scraping").await?,
        },
    }))
}

#[utoipa::path(get, path = "/api/admin/automatizacion/historial", tag = "admin",
    params(HistorialQuery),
    security(("bearer_auth" = [])),
    responses((status = 200, body = AdminAutomatizacionHistorialResponse), (status = 403)))]
pub async fn historial(
    State(state): State<AppState>,
    user: CurrentUser,
    Query(query): Query<HistorialQuery>,
) -> Result<Json<AdminAutomatizacionHistorialResponse>, AppError> {
    user.require_admin()?;
    let pagina = query.pagina.unwrap_or(1).max(1);
    let offset = (pagina - 1) * HISTORIAL_PAGE_SIZE;

    let tipo_filtro = match query.tipo.as_deref() {
        Some(t) if TIPOS_VALIDOS.contains(&t) => Some(t.to_string()),
        Some("") | None => None,
        Some(other) => {
            return Err(AppError::BadRequest(format!(
                "tipo invalido: '{other}'. Validos: extraccion, scraping"
            )))
        }
    };

    let items = sqlx::query_as::<_, LoteResumen>(
        r"SELECT id, tipo, estado, iniciado_at, completado_at,
                  exitosos, fallidos, recortes, samples_publicados,
                  canciones_nuevas, sampleos_nuevos, error_mensaje, metadata
             FROM lotes_procesamiento
            WHERE ($1::text IS NULL OR tipo = $1)
            ORDER BY iniciado_at DESC
            LIMIT $2 OFFSET $3",
    )
    .bind(tipo_filtro.as_deref())
    .bind(HISTORIAL_PAGE_SIZE)
    .bind(offset)
    .fetch_all(&state.pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        r"SELECT COUNT(*)::bigint FROM lotes_procesamiento
            WHERE ($1::text IS NULL OR tipo = $1)",
    )
    .bind(tipo_filtro.as_deref())
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(AdminAutomatizacionHistorialResponse {
        ok: true,
        items,
        total: total.0,
        pagina,
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/admin/automatizacion/estado", get(estado))
        .route("/admin/automatizacion/historial", get(historial))
}

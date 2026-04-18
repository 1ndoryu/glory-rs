use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;
use utoipa::ToSchema;

use crate::algorithm::InteractionKind;
use crate::errors::AppError;
use crate::middleware::CurrentUser;
use crate::repositories::DownloadRepository;
use crate::AppState;

/* [174A-61] Downloads — port mínimo de DescargasController.php.
 *
 * Endpoints:
 * - POST /api/samples/:id/descargar → registra descarga (consume crédito si
 *   aplica), retorna URL relativa del archivo y restantes del día.
 * - GET  /api/descargas/limites    → informa límite del plan + restantes hoy.
 *
 * Reglas portadas (mínimas):
 * - Sample debe existir y permitir descarga (a menos que el usuario sea
 *   creador) → 403 si no permite.
 * - Re-descargas y samples propios NO consumen crédito.
 * - Plan free: límite 5/día. Plan pro/premium: ilimitado.
 * - Sample con `precio > 0` y usuario no comprador y no plan pro+ → 403
 *   `requiere_compra`.
 * - Sample `es_premium` sin precio → requiere plan pro+ (403 si free).
 * - Trigger AlgoPlanner Descarga al registrar.
 *
 * NO portado:
 * - Códigos de descarga gratis (174A futuras).
 * - Compras individuales reales (TransaccionesRepository) — todavía no
 *   existe en Rust, así que el chequeo de comprado se omite (todos los
 *   samples con precio bloquean a free → coherente con UX legacy).
 * - Anti-abuso por IP.
 * - Calidad por plan (siempre se sirve `wav` legacy).
 * - Stream con token HMAC (174A-63).
 */

const PLAN_LIMIT_FREE: i64 = 5;

fn plan_daily_limit(plan: &str) -> Option<i64> {
    match plan {
        "free" => Some(PLAN_LIMIT_FREE),
        _ => None, // ilimitado
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DownloadResponse {
    pub ok: bool,
    pub url: String,
    pub calidad: String,
    pub consume_credito: bool,
    pub restantes: Option<i64>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DownloadLimitsResponse {
    pub plan: String,
    pub limite: Option<i64>,
    pub usadas: i64,
    pub restantes: Option<i64>,
    pub calidad: String,
}

#[utoipa::path(
    post,
    path = "/api/samples/{id}/descargar",
    tag = "downloads",
    params(("id" = i32, Path, description = "ID del sample a descargar")),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Descarga registrada", body = DownloadResponse),
        (status = 401, description = "No autenticado"),
        (status = 403, description = "Sample bloqueado para el usuario"),
        (status = 404, description = "Sample no encontrado"),
        (status = 429, description = "Límite diario excedido"),
    )
)]
pub async fn register_download(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(sample_id): Path<i32>,
) -> Result<Json<DownloadResponse>, AppError> {
    let info = DownloadRepository::fetch_sample_info(&state.pool, sample_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("sample {sample_id} no existe")))?;

    let es_propietario = info.creador_id == user.user_id;
    if !info.permitir_descarga && !es_propietario {
        return Err(AppError::Forbidden(
            "Este sample no permite descargas".into(),
        ));
    }

    let ya_descargado = !es_propietario
        && DownloadRepository::already_downloaded(&state.pool, user.user_id, sample_id).await?;
    let mut consume_credito = !es_propietario && !ya_descargado;

    let plan = DownloadRepository::user_plan(&state.pool, user.user_id).await?;

    if !es_propietario {
        let precio = info.precio.unwrap_or(0.0);
        if precio > 0.0 && !ya_descargado {
            /* Sin TransaccionesRepository todavía: bloquear a free, permitir
             * pro/premium descargar samples premium con precio (consume crédito 0). */
            if info.es_premium && plan != "free" {
                consume_credito = false;
            } else {
                return Err(AppError::Forbidden(format!(
                    "Este sample requiere compra individual (precio {precio:.2})"
                )));
            }
        } else if info.es_premium && plan == "free" {
            return Err(AppError::Forbidden(
                "Se requiere plan Pro o Premium para descargar este sample".into(),
            ));
        }
    }

    let limite = plan_daily_limit(&plan);
    let usadas = DownloadRepository::count_today(&state.pool, user.user_id).await?;
    let restantes = limite.map(|l| (l - usadas).max(0));

    if consume_credito {
        if let Some(limit) = limite {
            if usadas >= limit {
                return Err(AppError::RateLimited);
            }
        }
    }

    DownloadRepository::register(&state.pool, user.user_id, sample_id, "wav").await?;

    state
        .algo_planner
        .register_interaction(
            &state.pool,
            &state.redis,
            user.user_id,
            InteractionKind::Descarga,
        )
        .await?;

    let restantes_post = limite.map(|l| (l - usadas - i64::from(consume_credito)).max(0));

    Ok(Json(DownloadResponse {
        ok: true,
        url: format!("/api/samples/{sample_id}/file"),
        calidad: "wav".into(),
        consume_credito,
        restantes: restantes_post.or(restantes),
    }))
}

#[utoipa::path(
    get,
    path = "/api/descargas/limites",
    tag = "downloads",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Límites del plan", body = DownloadLimitsResponse),
        (status = 401, description = "No autenticado"),
    )
)]
pub async fn download_limits(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<DownloadLimitsResponse>, AppError> {
    let plan = DownloadRepository::user_plan(&state.pool, user.user_id).await?;
    let limite = plan_daily_limit(&plan);
    let usadas = DownloadRepository::count_today(&state.pool, user.user_id).await?;
    let restantes = limite.map(|l| (l - usadas).max(0));
    Ok(Json(DownloadLimitsResponse {
        plan,
        limite,
        usadas,
        restantes,
        calidad: "wav".into(),
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/samples/:id/descargar", post(register_download))
        .route("/descargas/limites", get(download_limits))
}

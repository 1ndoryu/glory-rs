/* [174A-56] Endpoints del feed personalizado.
 *
 * Tres rutas:
 *   - `GET /api/feed`          → feed personalizado (requiere auth).
 *   - `GET /api/me/feed`       → alias del anterior (compatibilidad legado).
 *   - `GET /api/samples/random` ya está en sample_catalog (no se duplica).
 *
 * Internamente delegan en `RecommenderService::feed`, que ya implementa la
 * cascada cache fresh → stale → cómputo síncrono. El handler solo valida
 * paginación y empaqueta el resultado.
 *
 * Decisión: usar `RecommenderConfig::legacy_defaults()` por ahora (no en
 * AppState). Si en el futuro se necesita exponer configurabilidad por env,
 * mover a `AppState::recommender_config`. */

use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::algorithm::recommender::{RankedSample, RecommenderConfig, RecommenderService};
use crate::errors::AppError;
#[allow(unused_imports)]
use crate::errors::ErrorResponse;
use crate::middleware::CurrentUser;
use crate::models::SampleSummary;
use crate::repositories::{ReportRepository, AUTO_HIDE_SAMPLE_REPORT_THRESHOLD};
use crate::services::SampleCatalogService;
use crate::AppState;

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct FeedQuery {
    /// Tamaño de página. Default: 20, máximo: 100.
    pub limit: Option<i64>,
    /// Offset desde 0. Default: 0.
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct FeedResponse {
    pub items: Vec<SampleSummary>,
    pub limit: i64,
    pub offset: i64,
    pub hay_mas: bool,
}

const DEFAULT_LIMIT: i64 = 20;
const DEFAULT_LIMIT_USIZE: usize = 20;
const MAX_LIMIT: i64 = 100;

fn normalize_pagination(query: &FeedQuery) -> (usize, usize) {
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let offset = query.offset.unwrap_or(0).max(0);
    (
        usize::try_from(limit).unwrap_or(DEFAULT_LIMIT_USIZE),
        usize::try_from(offset).unwrap_or(0),
    )
}

#[utoipa::path(
    get,
    path = "/api/feed",
    params(FeedQuery),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Feed personalizado del usuario autenticado", body = FeedResponse),
        (status = 401, description = "Autenticación requerida", body = ErrorResponse)
    )
)]
pub async fn get_feed(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<FeedQuery>,
) -> Result<Json<FeedResponse>, AppError> {
    let (limit, offset) = normalize_pagination(&query);
    let config = RecommenderConfig::legacy_defaults();
    let ranked_items = RecommenderService::feed(
        state.pool.clone(),
        state.redis.clone(),
        current_user.user_id,
        limit,
        offset,
        &config,
    )
    .await?;
    let hay_mas = ranked_items.len() == limit;
    let visible_ranked_items =
        filter_hidden_samples(&state, current_user.user_id, ranked_items).await?;
    let sample_ids = visible_ranked_items
        .iter()
        .map(|item| item.id)
        .collect::<Vec<_>>();
    let items = SampleCatalogService::list_public_samples_by_ids(
        &state.pool,
        state.public_base_url.as_deref(),
        Some(current_user.user_id),
        &sample_ids,
    )
    .await?;

    Ok(Json(FeedResponse {
        items,
        limit: i64::try_from(limit).unwrap_or(DEFAULT_LIMIT),
        offset: i64::try_from(offset).unwrap_or(0),
        hay_mas,
    }))
}

#[utoipa::path(
    get,
    path = "/api/me/feed",
    params(FeedQuery),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Alias de /api/feed (compatibilidad con cliente legado)", body = FeedResponse),
        (status = 401, description = "Autenticación requerida", body = ErrorResponse)
    )
)]
pub async fn get_me_feed(
    state: State<AppState>,
    current_user: CurrentUser,
    query: Query<FeedQuery>,
) -> Result<Json<FeedResponse>, AppError> {
    get_feed(state, current_user, query).await
}

/* [274A-18] Alias `/feed/inicio` consumido por apiSocial.ts. Comparte
 * lógica con get_feed; mantiene el contrato del frontend. */
#[utoipa::path(
    get,
    path = "/api/feed/inicio",
    params(FeedQuery),
    tag = "feed",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Alias de /api/feed (compatibilidad cliente legado)", body = FeedResponse),
        (status = 401, description = "No autenticado", body = ErrorResponse),
    )
)]
pub async fn get_feed_inicio(
    state: State<AppState>,
    current_user: CurrentUser,
    query: Query<FeedQuery>,
) -> Result<Json<FeedResponse>, AppError> {
    get_feed(state, current_user, query).await
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FeedRecargarResponse {
    pub ok: bool,
}

/* [274A-19] POST /feed/recargar — invalida cache fresco del feed para forzar
 * recálculo en la próxima petición. Port de SamplesController::recargarFeed. */
#[utoipa::path(
    post,
    path = "/api/feed/recargar",
    tag = "feed",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Cache fresco invalidado", body = FeedRecargarResponse),
        (status = 401, description = "No autenticado", body = ErrorResponse),
    )
)]
pub async fn recargar_feed(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<FeedRecargarResponse>, AppError> {
    RecommenderService::invalidate_user_feed(&state.redis, user.user_id).await?;
    Ok(Json(FeedRecargarResponse { ok: true }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/feed", get(get_feed))
        .route("/feed/inicio", get(get_feed_inicio))
        .route("/feed/recargar", post(recargar_feed))
        .route("/me/feed", get(get_me_feed))
}

async fn filter_hidden_samples(
    state: &AppState,
    viewer_id: i32,
    items: Vec<RankedSample>,
) -> Result<Vec<RankedSample>, AppError> {
    let sample_ids = items.iter().map(|item| item.id).collect::<Vec<_>>();
    let pending_counts =
        ReportRepository::pending_counts_for_targets(&state.pool, "sample", &sample_ids).await?;

    Ok(items
        .into_iter()
        .filter(|item| {
            item.creador_id == viewer_id
                || pending_counts.get(&item.id).copied().unwrap_or(0)
                    < AUTO_HIDE_SAMPLE_REPORT_THRESHOLD
        })
        .collect())
}

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
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::algorithm::recommender::{RankedSample, RecommenderConfig, RecommenderService};
use crate::errors::AppError;
#[allow(unused_imports)]
use crate::errors::ErrorResponse;
use crate::middleware::CurrentUser;
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
    pub items: Vec<RankedSample>,
    pub limit: i64,
    pub offset: i64,
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
    let items = RecommenderService::feed(
        state.pool.clone(),
        state.redis.clone(),
        current_user.user_id,
        limit,
        offset,
        &config,
    )
    .await?;

    Ok(Json(FeedResponse {
        items,
        limit: i64::try_from(limit).unwrap_or(DEFAULT_LIMIT),
        offset: i64::try_from(offset).unwrap_or(0),
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

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/feed", get(get_feed))
        .route("/me/feed", get(get_me_feed))
}

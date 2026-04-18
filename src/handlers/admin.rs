use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use utoipa::IntoParams;
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::CurrentUser;
use crate::models::{DeleteUserRequest, SuspendUserRequest};
use crate::repositories::ModerationRepository;
use crate::services::algo_timing::{TimingEntry, ALGO_TIMING};
use crate::AppState;

/* [174A-25] Endpoints admin: requiere rol admin (validado via require_admin). */

#[utoipa::path(post, path = "/api/admin/users/{id}/suspend",
    params(("id" = i32, Path, description = "User id")),
    request_body = SuspendUserRequest,
    security(("bearer_auth" = [])),
    responses((status = 204, description = "Suspendido"), (status = 401, description = "no auth"), (status = 403, description = "no admin"), (status = 404, description = "no encontrado")))]
pub async fn suspend(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i32>,
    Json(req): Json<SuspendUserRequest>,
) -> Result<StatusCode, AppError> {
    user.require_admin()?;
    req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    ModerationRepository::suspend(&state.pool, id, &req.razon, req.hasta).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(post, path = "/api/admin/users/{id}/activate",
    params(("id" = i32, Path, description = "User id")),
    security(("bearer_auth" = [])),
    responses((status = 204, description = "Reactivado"), (status = 401, description = "no auth"), (status = 403, description = "no admin"), (status = 404, description = "no encontrado")))]
pub async fn activate(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    user.require_admin()?;
    ModerationRepository::activate(&state.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(post, path = "/api/admin/users/{id}/delete",
    params(("id" = i32, Path, description = "User id")),
    request_body = DeleteUserRequest,
    security(("bearer_auth" = [])),
    responses((status = 204, description = "Marcado para eliminacion"), (status = 401, description = "no auth"), (status = 403, description = "no admin"), (status = 404, description = "no encontrado")))]
pub async fn mark_delete(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i32>,
    Json(req): Json<DeleteUserRequest>,
) -> Result<StatusCode, AppError> {
    user.require_admin()?;
    let dias = req.dias_gracia.unwrap_or(30).clamp(1, 365);
    ModerationRepository::mark_for_deletion(&state.pool, id, dias).await?;
    Ok(StatusCode::NO_CONTENT)
}

/* [174A-57] Endpoint admin para inspeccionar las últimas mediciones del
 * algoritmo. Solo se acumulan para `KAMPLES_ALGO_TIMING_USER_ID` (default 1). */

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct AlgoTimingQuery {
    /// Máximo de entradas a devolver. Default 50, máximo 100.
    pub limit: Option<usize>,
}

#[utoipa::path(
    get,
    path = "/api/admin/algo-timing",
    params(AlgoTimingQuery),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Historial de mediciones del feed (más reciente primero)", body = Vec<TimingEntry>),
        (status = 401, description = "no auth"),
        (status = 403, description = "no admin")
    )
)]
pub async fn algo_timing_history(
    user: CurrentUser,
    Query(query): Query<AlgoTimingQuery>,
) -> Result<Json<Vec<TimingEntry>>, AppError> {
    user.require_admin()?;
    let limit = query.limit.unwrap_or(50).clamp(1, 100);
    Ok(Json(ALGO_TIMING.history(limit)))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/admin/users/:id/suspend", post(suspend))
        .route("/admin/users/:id/activate", post(activate))
        .route("/admin/users/:id/delete", post(mark_delete))
        .route("/admin/algo-timing", get(algo_timing_history))
}

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::CurrentUser;
use crate::models::{DeleteUserRequest, SuspendUserRequest};
use crate::repositories::ModerationRepository;
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

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/admin/users/:id/suspend", post(suspend))
        .route("/admin/users/:id/activate", post(activate))
        .route("/admin/users/:id/delete", post(mark_delete))
}

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use validator::Validate;

use crate::errors::AppError;
#[allow(unused_imports)]
use crate::errors::ErrorResponse;
use crate::middleware::CurrentUser;
use crate::models::{BlockUserRequest, PrivateProfileResponse, PublicProfileResponse, UpdateProfileRequest};
use crate::repositories::{ModerationRepository, ProfileRepository};
use crate::AppState;

/* [174A-24] Endpoints de perfil. */

#[utoipa::path(get, path = "/api/users/me",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Perfil propio", body = PrivateProfileResponse),
        (status = 401, body = ErrorResponse)
    ))]
pub async fn me(State(state): State<AppState>, user: CurrentUser)
    -> Result<Json<PrivateProfileResponse>, AppError> {
    let p = ProfileRepository::find_by_id(&state.pool, user.user_id).await?
        .ok_or(AppError::NotFound("Usuario".into()))?;
    Ok(Json(PrivateProfileResponse::from(p)))
}

#[utoipa::path(patch, path = "/api/users/me",
    request_body = UpdateProfileRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Perfil actualizado", body = PrivateProfileResponse),
        (status = 401, body = ErrorResponse),
        (status = 422, body = ErrorResponse)
    ))]
pub async fn update_me(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<PrivateProfileResponse>, AppError> {
    req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let p = ProfileRepository::update(&state.pool, user.user_id, &req).await?;
    Ok(Json(PrivateProfileResponse::from(p)))
}

#[utoipa::path(get, path = "/api/users/{username}",
    params(("username" = String, Path, description = "Username publico")),
    responses(
        (status = 200, description = "Perfil publico", body = PublicProfileResponse),
        (status = 404, body = ErrorResponse)
    ))]
pub async fn public_profile(State(state): State<AppState>, Path(username): Path<String>)
    -> Result<Json<PublicProfileResponse>, AppError> {
    let p = ProfileRepository::find_by_username(&state.pool, &username).await?
        .ok_or(AppError::NotFound(format!("usuario {username}")))?;
    if p.estado != "activo" {
        return Err(AppError::NotFound(format!("usuario {username}")));
    }
    Ok(Json(PublicProfileResponse::from(p)))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users/me", get(me).patch(update_me))
        .route("/users/me/blocked", get(list_blocked))
        .route("/users/:username", get(public_profile))
        .route("/users/:username/block", post(block).delete(unblock))
}

#[utoipa::path(post, path = "/api/users/{username}/block",
    params(("username" = String, Path, description = "Username")),
    request_body = BlockUserRequest,
    security(("bearer_auth" = [])),
    responses((status = 204, description = "Bloqueado"), (status = 400, description = "auto-bloqueo"), (status = 401, description = "no auth"), (status = 404, description = "no encontrado")))]
pub async fn block(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(username): Path<String>,
    Json(req): Json<BlockUserRequest>,
) -> Result<StatusCode, AppError> {
    req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let target = ProfileRepository::find_by_username(&state.pool, &username).await?
        .ok_or(AppError::NotFound(format!("usuario {username}")))?;
    let razon = req.razon.unwrap_or_default();
    ModerationRepository::block(&state.pool, user.user_id, target.id, &razon).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(delete, path = "/api/users/{username}/block",
    params(("username" = String, Path, description = "Username")),
    security(("bearer_auth" = [])),
    responses((status = 204, description = "Desbloqueado"), (status = 401, description = "no auth"), (status = 404, description = "no encontrado")))]
pub async fn unblock(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(username): Path<String>,
) -> Result<StatusCode, AppError> {
    let target = ProfileRepository::find_by_username(&state.pool, &username).await?
        .ok_or(AppError::NotFound(format!("usuario {username}")))?;
    ModerationRepository::unblock(&state.pool, user.user_id, target.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(get, path = "/api/users/me/blocked",
    security(("bearer_auth" = [])),
    responses((status = 200, description = "Lista de IDs bloqueados", body = Vec<i32>), (status = 401, description = "no auth")))]
pub async fn list_blocked(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<Vec<i32>>, AppError> {
    let ids = ModerationRepository::list_blocked(&state.pool, user.user_id).await?;
    Ok(Json(ids))
}

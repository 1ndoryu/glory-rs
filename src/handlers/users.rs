use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::CurrentUser;
use crate::models::{PrivateProfileResponse, PublicProfileResponse, UpdateProfileRequest};
use crate::repositories::ProfileRepository;
use crate::AppState;

/* [174A-24] Endpoints de perfil. */

#[utoipa::path(get, path = "/api/users/me",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Perfil propio", body = PrivateProfileResponse),
        (status = 401, body = crate::errors::ErrorResponse)
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
        (status = 401, body = crate::errors::ErrorResponse),
        (status = 422, body = crate::errors::ErrorResponse)
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
        (status = 404, body = crate::errors::ErrorResponse)
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
        .route("/users/:username", get(public_profile))
}

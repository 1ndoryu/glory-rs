use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use validator::Validate;

use crate::errors::AppError;
use crate::models::{AuthResponse, LoginRequest, RegisterRequest};
use crate::services::AuthService;
use crate::AppState;

#[utoipa::path(post, path = "/api/auth/register", request_body = RegisterRequest,
    responses(
        (status = 201, description = "Usuario registrado", body = AuthResponse),
        (status = 409, description = "Email/username duplicado", body = crate::errors::ErrorResponse),
        (status = 422, description = "Validacion", body = crate::errors::ErrorResponse)
    ))]
pub async fn register(State(state): State<AppState>, Json(req): Json<RegisterRequest>)
    -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let resp = AuthService::register(&state.pool, req, &state.jwt_secret).await?;
    Ok((StatusCode::CREATED, Json(resp)))
}

#[utoipa::path(post, path = "/api/auth/login", request_body = LoginRequest,
    responses(
        (status = 200, description = "Login OK", body = AuthResponse),
        (status = 401, description = "Credenciales invalidas", body = crate::errors::ErrorResponse),
        (status = 403, description = "Cuenta no activa", body = crate::errors::ErrorResponse)
    ))]
pub async fn login(State(state): State<AppState>, Json(req): Json<LoginRequest>)
    -> Result<Json<AuthResponse>, AppError> {
    req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let resp = AuthService::login(&state.pool, req, &state.jwt_secret).await?;
    Ok(Json(resp))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}
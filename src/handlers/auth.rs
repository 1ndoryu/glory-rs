use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::CurrentUser;
use crate::models::{AuthResponse, GoogleAuthRequest, LoginRequest, LogoutRequest, RefreshRequest, RegisterRequest};
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
    let resp = AuthService::register(&state.pool, &state.redis, req, &state.jwt_secret).await?;
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
    let resp = AuthService::login(&state.pool, &state.redis, req, &state.jwt_secret).await?;
    Ok(Json(resp))
}

#[utoipa::path(post, path = "/api/auth/refresh", request_body = RefreshRequest,
    responses(
        (status = 200, description = "Tokens rotados", body = AuthResponse),
        (status = 401, description = "Refresh token invalido o expirado", body = crate::errors::ErrorResponse)
    ))]
pub async fn refresh(State(state): State<AppState>, Json(req): Json<RefreshRequest>)
    -> Result<Json<AuthResponse>, AppError> {
    let resp = AuthService::refresh(&state.pool, &state.redis, &req.refresh_token, &state.jwt_secret).await?;
    Ok(Json(resp))
}

#[utoipa::path(post, path = "/api/auth/logout",
    request_body = LogoutRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "Logout OK"),
        (status = 401, description = "No autenticado", body = crate::errors::ErrorResponse)
    ))]
pub async fn logout(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<LogoutRequest>,
) -> Result<StatusCode, AppError> {
    AuthService::logout(&state.redis, &user.jti, req.refresh_token.as_deref()).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh))
        .route("/auth/logout", post(logout))
        .route("/auth/google", post(google_login))
}

#[utoipa::path(post, path = "/api/auth/google", request_body = GoogleAuthRequest,
    responses(
        (status = 200, description = "Login Google OK", body = AuthResponse),
        (status = 401, description = "ID token invalido", body = crate::errors::ErrorResponse),
        (status = 403, description = "Email no verificado o cuenta bloqueada", body = crate::errors::ErrorResponse)
    ))]
pub async fn google_login(State(state): State<AppState>, Json(req): Json<GoogleAuthRequest>)
    -> Result<Json<AuthResponse>, AppError> {
    req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let resp = AuthService::google_login(&state.pool, &state.redis, &state.google, &req.id_token, &state.jwt_secret).await?;
    Ok(Json(resp))
}
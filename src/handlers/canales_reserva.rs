/* 263A-9: Handlers de canales de reserva — CRUD */

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::AuthUser;
use crate::models::{CanalReserva, CrearCanalReservaRequest};
use crate::services::CanalReservaService;
use crate::AppState;

/// Listar canales de reserva
#[utoipa::path(
    get,
    path = "/api/canales-reserva",
    tag = "Canales",
    responses(
        (status = 200, description = "Lista de canales", body = Vec<CanalReserva>),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn listar_canales(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<CanalReserva>>, AppError> {
    let canales = CanalReservaService::list(&state.pool, auth.user_id).await?;
    Ok(Json(canales))
}

/// Crear un canal de reserva
#[utoipa::path(
    post,
    path = "/api/canales-reserva",
    tag = "Canales",
    request_body = CrearCanalReservaRequest,
    responses(
        (status = 201, description = "Canal creado", body = CanalReserva),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 422, description = "Error de validación", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn crear_canal(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CrearCanalReservaRequest>,
) -> Result<(StatusCode, Json<CanalReserva>), AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let canal = CanalReservaService::create(&state.pool, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(canal)))
}

/// Eliminar un canal de reserva
#[utoipa::path(
    delete,
    path = "/api/canales-reserva/{id}",
    tag = "Canales",
    params(("id" = Uuid, Path, description = "ID del canal")),
    responses(
        (status = 204, description = "Canal eliminado"),
        (status = 404, description = "Canal no encontrado", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn eliminar_canal(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    CanalReservaService::delete(&state.pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/canales-reserva",
            get(listar_canales).post(crear_canal),
        )
        .route("/canales-reserva/{id}", axum::routing::delete(eliminar_canal))
}

/* [263A-17] Handlers de configuración del restaurante.
 * GET /api/configuracion — obtener config actual (crea defaults si no existe).
 * PATCH /api/configuracion — actualizar campos parcialmente. */

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::AuthUser;
use crate::models::{ActualizarConfiguracionRequest, ConfiguracionRestaurante};
use crate::services::ConfiguracionService;
use crate::AppState;

/// Obtener la configuración del restaurante (crea defaults si es primera vez)
#[utoipa::path(
    get,
    path = "/api/configuracion",
    tag = "Configuracion",
    responses(
        (status = 200, description = "Configuración actual", body = ConfiguracionRestaurante),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn obtener_configuracion(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<ConfiguracionRestaurante>, AppError> {
    let config = ConfiguracionService::obtener(&state.pool, auth.user_id).await?;
    Ok(Json(config))
}

/// Actualizar la configuración del restaurante (parcial)
#[utoipa::path(
    patch,
    path = "/api/configuracion",
    tag = "Configuracion",
    request_body = ActualizarConfiguracionRequest,
    responses(
        (status = 200, description = "Configuración actualizada", body = ConfiguracionRestaurante),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 422, description = "Error de validación", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn actualizar_configuracion(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<ActualizarConfiguracionRequest>,
) -> Result<Json<ConfiguracionRestaurante>, AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let config = ConfiguracionService::actualizar(&state.pool, auth.user_id, &req).await?;
    Ok(Json(config))
}

pub fn routes() -> Router<AppState> {
    Router::new().route(
        "/configuracion",
        get(obtener_configuracion).patch(actualizar_configuracion),
    )
}

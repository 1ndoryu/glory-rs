/* [064A-62] Handler admin para gestión de datos de prueba (seed).
 * Solo admin. Permite recrear o borrar datos de prueba desde el panel. */

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serde::Serialize;

use crate::errors::AppError;
use crate::middleware::AuthUser;
use crate::models::UserRole;
use crate::services::SeedService;
use crate::AppState;

#[derive(Serialize)]
pub struct SeedResponse {
    pub message: String,
}

/// Recrear datos de prueba (borrar + crear nuevos)
pub async fn recreate_seed(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<SeedResponse>, AppError> {
    auth.require_role(&[UserRole::Admin])?;

    let message = SeedService::recreate_test_data(&state.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Error en seed: {e}")))?;

    Ok(Json(SeedResponse { message }))
}

/// Borrar datos de prueba
pub async fn delete_seed(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<(StatusCode, Json<SeedResponse>), AppError> {
    auth.require_role(&[UserRole::Admin])?;

    let deleted = SeedService::delete_test_data(&state.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Error al borrar seed: {e}")))?;

    Ok((
        StatusCode::OK,
        Json(SeedResponse {
            message: format!("Datos de prueba eliminados ({deleted} usuarios borrados)."),
        }),
    ))
}

pub fn seed_routes() -> Router<AppState> {
    Router::new()
        .route("/admin/seed", post(recreate_seed).delete(delete_seed))
}

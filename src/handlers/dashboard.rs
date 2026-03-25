/* 253A-5: Handler de dashboard — resumen económico */

use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::errors::AppError;
use crate::middleware::AuthUser;
use crate::models::ResumenEconomico;
use crate::services::DashboardService;
use crate::AppState;

/// Query params para el resumen económico
#[derive(Debug, Deserialize, IntoParams)]
pub struct ResumenQuery {
    /// Año (ej: 2026)
    pub year: i32,
    /// Mes (1-12)
    pub month: u32,
}

/// Resumen económico mensual: ventas, gastos, margen
#[utoipa::path(
    get,
    path = "/api/dashboard/resumen",
    params(ResumenQuery),
    responses(
        (status = 200, description = "Resumen económico", body = ResumenEconomico),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn resumen(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<ResumenQuery>,
) -> Result<Json<ResumenEconomico>, AppError> {
    let resumen =
        DashboardService::resumen_mes(&state.pool, auth.user_id, params.year, params.month).await?;
    Ok(Json(resumen))
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/dashboard/resumen", get(resumen))
}

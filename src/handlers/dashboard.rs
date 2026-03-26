/* 253A-5: Handler de dashboard — resumen económico
 * 263A-13: Dashboard de reservas Fase 2 */

use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::errors::AppError;
use crate::middleware::AuthUser;
use crate::models::{DashboardReservas, ResumenEconomico};
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
    tag = "Dashboard",
    params(ResumenQuery),
    responses(
        (status = 200, description = "Resumen económico", body = ResumenEconomico),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn resumen(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<ResumenQuery>,
) -> Result<Json<ResumenEconomico>, AppError> {
    let data =
        DashboardService::resumen_mes(&state.pool, auth.user_id, params.year, params.month).await?;
    Ok(Json(data))
}

/// Dashboard completo de reservas: resumen, ocupacion y analisis
#[utoipa::path(
    get,
    path = "/api/dashboard/reservas",
    tag = "Dashboard",
    params(ResumenQuery),
    responses(
        (status = 200, description = "Dashboard de reservas", body = DashboardReservas),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn dashboard_reservas(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<ResumenQuery>,
) -> Result<Json<DashboardReservas>, AppError> {
    let data = DashboardService::dashboard_reservas(
        &state.pool,
        auth.user_id,
        params.year,
        params.month,
    )
    .await?;
    Ok(Json(data))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard/resumen", get(resumen))
        .route("/dashboard/reservas", get(dashboard_reservas))
}

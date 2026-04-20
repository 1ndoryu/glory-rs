use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};

use crate::errors::AppError;
#[allow(unused_imports)]
use crate::errors::ErrorResponse;
use crate::middleware::CurrentUser;
use crate::models::{
    CreatorDashboardIncomePoint, CreatorDashboardIncomeQuery, CreatorDashboardSampleStat,
    CreatorDashboardStats, CreatorDashboardTransaction, CreatorDashboardTransactionsQuery,
};
use crate::repositories::{BillingRepository, CreatorDashboardRepository};
use crate::AppState;

#[utoipa::path(
    get,
    path = "/api/dashboard/stats",
    tag = "payments",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "KPIs del dashboard de creador", body = CreatorDashboardStats),
        (status = 401, description = "No autenticado", body = ErrorResponse),
        (status = 404, description = "Usuario no encontrado", body = ErrorResponse)
    )
)]
pub async fn stats(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<CreatorDashboardStats>, AppError> {
    ensure_user_exists(&state, user.user_id).await?;
    let response = CreatorDashboardRepository::stats(&state.pool, user.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Usuario {} no encontrado", user.user_id)))?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/dashboard/top-samples",
    tag = "payments",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Top samples del creador", body = [CreatorDashboardSampleStat]),
        (status = 401, description = "No autenticado", body = ErrorResponse),
        (status = 404, description = "Usuario no encontrado", body = ErrorResponse)
    )
)]
pub async fn top_samples(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<Vec<CreatorDashboardSampleStat>>, AppError> {
    ensure_user_exists(&state, user.user_id).await?;
    Ok(Json(
        CreatorDashboardRepository::top_samples(&state.pool, user.user_id).await?,
    ))
}

#[utoipa::path(
    get,
    path = "/api/dashboard/transacciones",
    tag = "payments",
    params(CreatorDashboardTransactionsQuery),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Historial de transacciones del creador", body = [CreatorDashboardTransaction]),
        (status = 401, description = "No autenticado", body = ErrorResponse),
        (status = 404, description = "Usuario no encontrado", body = ErrorResponse)
    )
)]
pub async fn transactions(
    State(state): State<AppState>,
    user: CurrentUser,
    Query(query): Query<CreatorDashboardTransactionsQuery>,
) -> Result<Json<Vec<CreatorDashboardTransaction>>, AppError> {
    ensure_user_exists(&state, user.user_id).await?;
    Ok(Json(
        CreatorDashboardRepository::transactions(&state.pool, user.user_id, &query).await?,
    ))
}

#[utoipa::path(
    get,
    path = "/api/dashboard/ingresos",
    tag = "payments",
    params(CreatorDashboardIncomeQuery),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Serie de ingresos del dashboard de creador", body = [CreatorDashboardIncomePoint]),
        (status = 401, description = "No autenticado", body = ErrorResponse),
        (status = 404, description = "Usuario no encontrado", body = ErrorResponse)
    )
)]
pub async fn income_series(
    State(state): State<AppState>,
    user: CurrentUser,
    Query(query): Query<CreatorDashboardIncomeQuery>,
) -> Result<Json<Vec<CreatorDashboardIncomePoint>>, AppError> {
    ensure_user_exists(&state, user.user_id).await?;
    Ok(Json(
        CreatorDashboardRepository::income_series(&state.pool, user.user_id, &query).await?,
    ))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard/stats", get(stats))
        .route("/dashboard/top-samples", get(top_samples))
        .route("/dashboard/transacciones", get(transactions))
        .route("/dashboard/ingresos", get(income_series))
}

async fn ensure_user_exists(state: &AppState, user_id: i32) -> Result<(), AppError> {
    let exists = BillingRepository::find_stripe_user_profile(&state.pool, user_id)
        .await?
        .is_some();
    if exists {
        Ok(())
    } else {
        Err(AppError::NotFound(format!("Usuario {user_id} no encontrado")))
    }
}
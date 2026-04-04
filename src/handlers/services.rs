/* [044A-38] Handlers del catálogo de servicios — endpoints públicos.
 * Cualquier visitante puede ver servicios y planes sin autenticación. */

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};

use crate::errors::AppError;
use crate::models::ServiceDetailResponse;
use crate::services::OrderService;
use crate::AppState;

/// Listar todos los servicios activos con planes y fases
#[utoipa::path(
    get,
    path = "/api/services",
    responses(
        (status = 200, description = "Lista de servicios", body = Vec<ServiceDetailResponse>),
    ),
    tag = "services"
)]
pub async fn list_services(
    State(state): State<AppState>,
) -> Result<Json<Vec<ServiceDetailResponse>>, AppError> {
    let services = OrderService::list_services(&state.pool).await?;
    Ok(Json(services))
}

/// Detalle de un servicio por slug
#[utoipa::path(
    get,
    path = "/api/services/{slug}",
    params(("slug" = String, Path, description = "Slug del servicio")),
    responses(
        (status = 200, description = "Detalle del servicio", body = ServiceDetailResponse),
        (status = 404, description = "No encontrado", body = crate::errors::ErrorResponse),
    ),
    tag = "services"
)]
pub async fn get_service(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<ServiceDetailResponse>, AppError> {
    let service = OrderService::get_service(&state.pool, &slug).await?;
    Ok(Json(service))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/services", get(list_services))
        .route("/services/:slug", get(get_service))
}

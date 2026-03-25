/* 253A-5: Handlers de gastos — CRUD + categorías */

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::AuthUser;
use crate::models::{CategoriaGasto, CrearGastoRequest, Gasto, GastosPaginados, GastosQuery};
use crate::services::GastoService;
use crate::AppState;

/// Crear un gasto
#[utoipa::path(
    post,
    path = "/api/gastos",
    request_body = CrearGastoRequest,
    responses(
        (status = 201, description = "Gasto creado", body = Gasto),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 422, description = "Error de validación", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn crear_gasto(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CrearGastoRequest>,
) -> Result<(StatusCode, Json<Gasto>), AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let gasto = GastoService::create(&state.pool, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(gasto)))
}

/// Obtener un gasto por ID
#[utoipa::path(
    get,
    path = "/api/gastos/{id}",
    params(("id" = Uuid, Path, description = "ID del gasto")),
    responses(
        (status = 200, description = "Gasto encontrado", body = Gasto),
        (status = 404, description = "Gasto no encontrado", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn obtener_gasto(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Gasto>, AppError> {
    let gasto = GastoService::get(&state.pool, id, auth.user_id).await?;
    Ok(Json(gasto))
}

/// Listar gastos con paginación y filtros
#[utoipa::path(
    get,
    path = "/api/gastos",
    params(GastosQuery),
    responses(
        (status = 200, description = "Lista de gastos", body = GastosPaginados),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn listar_gastos(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<GastosQuery>,
) -> Result<Json<GastosPaginados>, AppError> {
    let gastos = GastoService::list(
        &state.pool,
        auth.user_id,
        params.page,
        params.per_page,
        params.desde,
        params.hasta,
        params.categoria_id,
    )
    .await?;
    Ok(Json(gastos))
}

/// Eliminar un gasto
#[utoipa::path(
    delete,
    path = "/api/gastos/{id}",
    params(("id" = Uuid, Path, description = "ID del gasto")),
    responses(
        (status = 204, description = "Gasto eliminado"),
        (status = 404, description = "Gasto no encontrado", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn eliminar_gasto(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    GastoService::delete(&state.pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Listar categorías de gasto — endpoint público
#[utoipa::path(
    get,
    path = "/api/gastos/categorias",
    responses(
        (status = 200, description = "Lista de categorías", body = Vec<CategoriaGasto>)
    )
)]
pub async fn listar_categorias(
    State(state): State<AppState>,
) -> Result<Json<Vec<CategoriaGasto>>, AppError> {
    let cats = GastoService::categorias(&state.pool).await?;
    Ok(Json(cats))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/gastos/categorias", get(listar_categorias))
        .route("/gastos", post(crear_gasto).get(listar_gastos))
        .route("/gastos/{id}", get(obtener_gasto).delete(eliminar_gasto))
}

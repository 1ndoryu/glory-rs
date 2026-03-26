/* 263A-1: Handlers de clientes — CRUD CRM con búsqueda y paginación */

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::AuthUser;
use crate::models::{
    ActualizarClienteRequest, Cliente, ClientesPaginados, ClientesQuery, CrearClienteRequest,
};
use crate::services::ClienteService;
use crate::AppState;

/// Crear un cliente
#[utoipa::path(
    post,
    path = "/api/clientes",
    request_body = CrearClienteRequest,
    responses(
        (status = 201, description = "Cliente creado", body = Cliente),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 422, description = "Error de validación", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn crear_cliente(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CrearClienteRequest>,
) -> Result<(StatusCode, Json<Cliente>), AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let cliente = ClienteService::create(&state.pool, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(cliente)))
}

/// Obtener un cliente por ID
#[utoipa::path(
    get,
    path = "/api/clientes/{id}",
    params(("id" = Uuid, Path, description = "ID del cliente")),
    responses(
        (status = 200, description = "Cliente encontrado", body = Cliente),
        (status = 404, description = "Cliente no encontrado", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn obtener_cliente(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Cliente>, AppError> {
    let cliente = ClienteService::get(&state.pool, id, auth.user_id).await?;
    Ok(Json(cliente))
}

/// Listar clientes con paginación y búsqueda
#[utoipa::path(
    get,
    path = "/api/clientes",
    params(ClientesQuery),
    responses(
        (status = 200, description = "Lista de clientes", body = ClientesPaginados),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn listar_clientes(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<ClientesQuery>,
) -> Result<Json<ClientesPaginados>, AppError> {
    let resultado = ClienteService::list(&state.pool, auth.user_id, query).await?;
    Ok(Json(resultado))
}

/// Actualizar un cliente
#[utoipa::path(
    put,
    path = "/api/clientes/{id}",
    params(("id" = Uuid, Path, description = "ID del cliente")),
    request_body = ActualizarClienteRequest,
    responses(
        (status = 200, description = "Cliente actualizado", body = Cliente),
        (status = 404, description = "Cliente no encontrado", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn actualizar_cliente(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<ActualizarClienteRequest>,
) -> Result<Json<Cliente>, AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let cliente = ClienteService::update(&state.pool, id, auth.user_id, req).await?;
    Ok(Json(cliente))
}

/// Eliminar un cliente
#[utoipa::path(
    delete,
    path = "/api/clientes/{id}",
    params(("id" = Uuid, Path, description = "ID del cliente")),
    responses(
        (status = 204, description = "Cliente eliminado"),
        (status = 404, description = "Cliente no encontrado", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn eliminar_cliente(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    ClienteService::delete(&state.pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/clientes", post(crear_cliente).get(listar_clientes))
        .route(
            "/clientes/{id}",
            get(obtener_cliente)
                .put(actualizar_cliente)
                .delete(eliminar_cliente),
        )
}

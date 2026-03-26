/* 263A-1: Handlers de etiquetas — categorías, tags, asignaciones a clientes y reservas */

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::AuthUser;
use crate::models::{
    CategoriaEtiqueta, CrearCategoriaEtiquetaRequest, CrearEtiquetaRequest, EtiquetaConCategoria,
    EtiquetasQuery,
};
use crate::services::EtiquetaService;
use crate::AppState;

/// Body para asignar/desasignar etiqueta
#[derive(Deserialize, utoipa::ToSchema)]
pub struct TagAssignBody {
    pub etiqueta_id: Uuid,
}

/* --- Categorías --- */

/// Listar categorías de etiquetas
#[utoipa::path(
    get,
    path = "/api/etiquetas/categorias",
    tag = "Etiquetas",
    responses(
        (status = 200, description = "Lista de categorías", body = Vec<CategoriaEtiqueta>),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn listar_categorias(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<CategoriaEtiqueta>>, AppError> {
    let cats = EtiquetaService::list_categorias(&state.pool, auth.user_id).await?;
    Ok(Json(cats))
}

/// Crear una categoría de etiquetas
#[utoipa::path(
    post,
    path = "/api/etiquetas/categorias",
    tag = "Etiquetas",
    request_body = CrearCategoriaEtiquetaRequest,
    responses(
        (status = 201, description = "Categoría creada", body = CategoriaEtiqueta),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 422, description = "Error de validación", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn crear_categoria(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CrearCategoriaEtiquetaRequest>,
) -> Result<(StatusCode, Json<CategoriaEtiqueta>), AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let cat =
        EtiquetaService::create_categoria(&state.pool, auth.user_id, &req.nombre, &req.aplica_a)
            .await?;
    Ok((StatusCode::CREATED, Json(cat)))
}

/* --- Etiquetas --- */

/// Listar etiquetas (con filtro opcional por categoría)
#[utoipa::path(
    get,
    path = "/api/etiquetas",
    tag = "Etiquetas",
    params(EtiquetasQuery),
    responses(
        (status = 200, description = "Lista de etiquetas", body = Vec<EtiquetaConCategoria>),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn listar_etiquetas(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<EtiquetasQuery>,
) -> Result<Json<Vec<EtiquetaConCategoria>>, AppError> {
    let tags = EtiquetaService::list_etiquetas(&state.pool, auth.user_id, query).await?;
    Ok(Json(tags))
}

/// Crear una etiqueta
#[utoipa::path(
    post,
    path = "/api/etiquetas",
    tag = "Etiquetas",
    request_body = CrearEtiquetaRequest,
    responses(
        (status = 201, description = "Etiqueta creada", body = EtiquetaConCategoria),
        (status = 401, description = "No autorizado", body = ErrorResponse),
        (status = 422, description = "Error de validación", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn crear_etiqueta(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CrearEtiquetaRequest>,
) -> Result<(StatusCode, Json<EtiquetaConCategoria>), AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let tag = EtiquetaService::create_etiqueta(&state.pool, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(tag)))
}

/// Eliminar una etiqueta
#[utoipa::path(
    delete,
    path = "/api/etiquetas/{id}",
    tag = "Etiquetas",
    params(("id" = Uuid, Path, description = "ID de la etiqueta")),
    responses(
        (status = 204, description = "Etiqueta eliminada"),
        (status = 404, description = "Etiqueta no encontrada", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn eliminar_etiqueta(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    EtiquetaService::delete_etiqueta(&state.pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/* --- Asignaciones a clientes --- */

/// Asignar etiqueta a un cliente
#[utoipa::path(
    post,
    path = "/api/clientes/{id}/etiquetas",
    tag = "Etiquetas",
    params(("id" = Uuid, Path, description = "ID del cliente")),
    request_body = TagAssignBody,
    responses(
        (status = 204, description = "Etiqueta asignada"),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn asignar_etiqueta_cliente(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(cliente_id): Path<Uuid>,
    Json(body): Json<TagAssignBody>,
) -> Result<StatusCode, AppError> {
    EtiquetaService::assign_to_client(&state.pool, cliente_id, body.etiqueta_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Desasignar etiqueta de un cliente
#[utoipa::path(
    delete,
    path = "/api/clientes/{cliente_id}/etiquetas/{etiqueta_id}",
    tag = "Etiquetas",
    params(
        ("cliente_id" = Uuid, Path, description = "ID del cliente"),
        ("etiqueta_id" = Uuid, Path, description = "ID de la etiqueta")
    ),
    responses(
        (status = 204, description = "Etiqueta desasignada"),
        (status = 404, description = "Asignación no encontrada", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn desasignar_etiqueta_cliente(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path((cliente_id, etiqueta_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    EtiquetaService::unassign_from_client(&state.pool, cliente_id, etiqueta_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Obtener etiquetas de un cliente
#[utoipa::path(
    get,
    path = "/api/clientes/{id}/etiquetas",
    tag = "Etiquetas",
    params(("id" = Uuid, Path, description = "ID del cliente")),
    responses(
        (status = 200, description = "Etiquetas del cliente", body = Vec<EtiquetaConCategoria>),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn obtener_etiquetas_cliente(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(cliente_id): Path<Uuid>,
) -> Result<Json<Vec<EtiquetaConCategoria>>, AppError> {
    let tags = EtiquetaService::get_client_tags(&state.pool, cliente_id).await?;
    Ok(Json(tags))
}

/* --- Asignaciones a reservas --- */

/// Asignar etiqueta a una reserva
#[utoipa::path(
    post,
    path = "/api/reservas/{id}/etiquetas",
    tag = "Etiquetas",
    params(("id" = Uuid, Path, description = "ID de la reserva")),
    request_body = TagAssignBody,
    responses(
        (status = 204, description = "Etiqueta asignada"),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn asignar_etiqueta_reserva(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(reserva_id): Path<Uuid>,
    Json(body): Json<TagAssignBody>,
) -> Result<StatusCode, AppError> {
    EtiquetaService::assign_to_reservation(&state.pool, reserva_id, body.etiqueta_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Desasignar etiqueta de una reserva
#[utoipa::path(
    delete,
    path = "/api/reservas/{reserva_id}/etiquetas/{etiqueta_id}",
    tag = "Etiquetas",
    params(
        ("reserva_id" = Uuid, Path, description = "ID de la reserva"),
        ("etiqueta_id" = Uuid, Path, description = "ID de la etiqueta")
    ),
    responses(
        (status = 204, description = "Etiqueta desasignada"),
        (status = 404, description = "Asignación no encontrada", body = ErrorResponse),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn desasignar_etiqueta_reserva(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path((reserva_id, etiqueta_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    EtiquetaService::unassign_from_reservation(&state.pool, reserva_id, etiqueta_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Obtener etiquetas de una reserva
#[utoipa::path(
    get,
    path = "/api/reservas/{id}/etiquetas",
    tag = "Etiquetas",
    params(("id" = Uuid, Path, description = "ID de la reserva")),
    responses(
        (status = 200, description = "Etiquetas de la reserva", body = Vec<EtiquetaConCategoria>),
        (status = 401, description = "No autorizado", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn obtener_etiquetas_reserva(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(reserva_id): Path<Uuid>,
) -> Result<Json<Vec<EtiquetaConCategoria>>, AppError> {
    let tags = EtiquetaService::get_reservation_tags(&state.pool, reserva_id).await?;
    Ok(Json(tags))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        /* Categorías y etiquetas */
        .route(
            "/etiquetas/categorias",
            get(listar_categorias).post(crear_categoria),
        )
        .route("/etiquetas", get(listar_etiquetas).post(crear_etiqueta))
        .route("/etiquetas/:id", axum::routing::delete(eliminar_etiqueta))
        /* Asignaciones a clientes */
        .route(
            "/clientes/:id/etiquetas",
            get(obtener_etiquetas_cliente).post(asignar_etiqueta_cliente),
        )
        .route(
            "/clientes/:cliente_id/etiquetas/:etiqueta_id",
            axum::routing::delete(desasignar_etiqueta_cliente),
        )
        /* Asignaciones a reservas */
        .route(
            "/reservas/:id/etiquetas",
            get(obtener_etiquetas_reserva).post(asignar_etiqueta_reserva),
        )
        .route(
            "/reservas/:reserva_id/etiquetas/:etiqueta_id",
            axum::routing::delete(desasignar_etiqueta_reserva),
        )
}

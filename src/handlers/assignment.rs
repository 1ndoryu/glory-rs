/* [044A-38 Fase 4] Handlers de asignación y delegación.
 * Endpoints: tomar orden, listar sin asignar, lista empleados,
 * delegación (crear/responder/listar), solicitud de ayuda. */

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::AuthUser;
use crate::models::{
    CreateDelegationRequest, DelegationResponse, EmployeeListItem, OrderResponse,
    RespondDelegationRequest, UserRole,
};
use crate::services::AssignmentService;
use crate::AppState;

/* ============================================================
   ASIGNACIÓN
   ============================================================ */

/// Empleado toma una orden sin asignar
#[utoipa::path(
    post,
    path = "/api/orders/{order_id}/take",
    params(("order_id" = Uuid, Path, description = "ID de la orden")),
    responses(
        (status = 200, description = "Orden tomada", body = OrderResponse),
        (status = 400, description = "No disponible o límite alcanzado", body = crate::errors::ErrorResponse),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
        (status = 403, description = "Solo empleados", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "assignment"
)]
pub async fn take_order(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(order_id): Path<Uuid>,
) -> Result<Json<OrderResponse>, AppError> {
    auth.require_role(&[UserRole::Employee, UserRole::Admin])?;
    let order = AssignmentService::take_order(&state.pool, order_id, auth.user_id).await?;
    Ok(Json(order))
}

/// Listar órdenes sin asignar (admin y empleados)
#[utoipa::path(
    get,
    path = "/api/orders/unassigned",
    responses(
        (status = 200, description = "Órdenes sin asignar", body = Vec<OrderResponse>),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "assignment"
)]
pub async fn list_unassigned(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<OrderResponse>>, AppError> {
    auth.require_role(&[UserRole::Employee, UserRole::Admin])?;
    let orders = AssignmentService::list_unassigned(&state.pool).await?;
    Ok(Json(orders))
}

/* ============================================================
   ADMIN — EMPLEADOS
   ============================================================ */

/// Listar empleados con estadísticas (solo admin)
#[utoipa::path(
    get,
    path = "/api/admin/employees",
    responses(
        (status = 200, description = "Lista de empleados", body = Vec<EmployeeListItem>),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
        (status = 403, description = "Solo admin", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "admin"
)]
pub async fn list_employees(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<EmployeeListItem>>, AppError> {
    auth.require_role(&[UserRole::Admin])?;
    let employees = AssignmentService::list_employees(&state.pool).await?;
    Ok(Json(employees))
}

/* ============================================================
   DELEGACIONES
   ============================================================ */

/// Crear delegación (empleado pide que otro tome su orden)
#[utoipa::path(
    post,
    path = "/api/orders/{order_id}/delegate",
    params(("order_id" = Uuid, Path, description = "ID de la orden")),
    request_body = CreateDelegationRequest,
    responses(
        (status = 201, description = "Delegación creada", body = DelegationResponse),
        (status = 400, description = "Datos inválidos", body = crate::errors::ErrorResponse),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
        (status = 403, description = "Solo el empleado asignado", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "delegation"
)]
pub async fn create_delegation(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(order_id): Path<Uuid>,
    Json(req): Json<CreateDelegationRequest>,
) -> Result<(StatusCode, Json<DelegationResponse>), AppError> {
    auth.require_role(&[UserRole::Employee, UserRole::Admin])?;
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let d = AssignmentService::create_delegation(
        &state.pool,
        order_id,
        auth.user_id,
        &req.reason,
        "delegate",
    )
    .await?;
    Ok((StatusCode::CREATED, Json(d)))
}

/// Solicitar ayuda para una orden (no transfiere, busca colaborador)
#[utoipa::path(
    post,
    path = "/api/orders/{order_id}/help",
    params(("order_id" = Uuid, Path, description = "ID de la orden")),
    request_body = CreateDelegationRequest,
    responses(
        (status = 201, description = "Solicitud de ayuda creada", body = DelegationResponse),
        (status = 400, description = "Datos inválidos", body = crate::errors::ErrorResponse),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
        (status = 403, description = "Solo el empleado asignado", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "delegation"
)]
pub async fn create_help_request(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(order_id): Path<Uuid>,
    Json(req): Json<CreateDelegationRequest>,
) -> Result<(StatusCode, Json<DelegationResponse>), AppError> {
    auth.require_role(&[UserRole::Employee, UserRole::Admin])?;
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let d = AssignmentService::create_delegation(
        &state.pool,
        order_id,
        auth.user_id,
        &req.reason,
        "help_request",
    )
    .await?;
    Ok((StatusCode::CREATED, Json(d)))
}

/// Responder a delegación (aceptar/rechazar)
#[utoipa::path(
    patch,
    path = "/api/delegations/{delegation_id}",
    params(("delegation_id" = Uuid, Path, description = "ID de la delegación")),
    request_body = RespondDelegationRequest,
    responses(
        (status = 200, description = "Delegación respondida", body = DelegationResponse),
        (status = 400, description = "Ya resuelta o sin capacidad", body = crate::errors::ErrorResponse),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "delegation"
)]
pub async fn respond_delegation(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(delegation_id): Path<Uuid>,
    Json(req): Json<RespondDelegationRequest>,
) -> Result<Json<DelegationResponse>, AppError> {
    auth.require_role(&[UserRole::Employee, UserRole::Admin])?;
    let d = AssignmentService::respond_to_delegation(
        &state.pool,
        delegation_id,
        auth.user_id,
        req.accept,
    )
    .await?;
    Ok(Json(d))
}

/// Listar delegaciones del empleado (o todas para admin)
#[utoipa::path(
    get,
    path = "/api/delegations",
    responses(
        (status = 200, description = "Lista de delegaciones", body = Vec<DelegationResponse>),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "delegation"
)]
pub async fn list_delegations(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<DelegationResponse>>, AppError> {
    auth.require_role(&[UserRole::Employee, UserRole::Admin])?;
    let delegations = AssignmentService::list_delegations(
        &state.pool,
        auth.user_id,
        auth.effective_role,
    )
    .await?;
    Ok(Json(delegations))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/orders/unassigned", get(list_unassigned))
        .route("/orders/:order_id/take", post(take_order))
        .route("/orders/:order_id/delegate", post(create_delegation))
        .route("/orders/:order_id/help", post(create_help_request))
        .route("/delegations", get(list_delegations))
        .route("/delegations/:delegation_id", patch(respond_delegation))
        .route("/admin/employees", get(list_employees))
}

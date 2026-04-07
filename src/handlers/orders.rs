/* [044A-38] Handlers de órdenes — CRUD con autenticación.
 * Cliente crea órdenes, admin/employee ven según rol, admin asigna. */

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post, put};
use axum::{Json, Router};
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::middleware::AuthUser;
use crate::models::{
    CreateOrderRequest, OrderResponse, OrderStatus, SwitchRoleRequest, UserRole,
};
use crate::repositories::{OrderRepository, UserRepository};
use crate::services::{AuthService, OrderService, PaymentService};
use crate::AppState;

/* ============================================================
   ÓRDENES
   ============================================================ */

/// Crear una nueva orden (solo clientes o admin operando como cliente)
#[utoipa::path(
    post,
    path = "/api/orders",
    request_body = CreateOrderRequest,
    responses(
        (status = 201, description = "Orden creada", body = OrderResponse),
        (status = 400, description = "Datos inválidos", body = crate::errors::ErrorResponse),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
        (status = 403, description = "Sin permisos", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "orders"
)]
pub async fn create_order(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<OrderResponse>), AppError> {
    auth.require_role(&[UserRole::Client, UserRole::Admin])?;
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let order = OrderService::create_order(&state.pool, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(order)))
}

/// Listar órdenes del usuario autenticado (filtrado por rol)
#[utoipa::path(
    get,
    path = "/api/orders",
    responses(
        (status = 200, description = "Lista de órdenes", body = Vec<OrderResponse>),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "orders"
)]
pub async fn list_orders(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<OrderResponse>>, AppError> {
    /* [064A-58] Admin siempre ve todas las órdenes, incluso si switcheó effective_role
     * a client/employee. El effective_role solo afecta la UI (tabs), no el filtrado. */
    let query_role = if auth.role == UserRole::Admin {
        UserRole::Admin
    } else {
        auth.effective_role
    };
    let orders =
        OrderService::list_orders_for_user(&state.pool, auth.user_id, query_role).await?;
    Ok(Json(orders))
}

/// Detalle de una orden con sus fases
#[utoipa::path(
    get,
    path = "/api/orders/{order_id}",
    params(("order_id" = Uuid, Path, description = "ID de la orden")),
    responses(
        (status = 200, description = "Detalle de la orden"),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
        (status = 404, description = "No encontrada", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "orders"
)]
pub async fn get_order(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(order_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (client_id, order, phases) = OrderService::get_order(&state.pool, order_id).await?;

    /* [074A-50] Admin real siempre puede ver cualquier orden, incluso con effective_role
     * switcheado a client/employee. El effective_role solo afecta la UI, no el acceso. */
    if auth.role != UserRole::Admin {
        match auth.effective_role {
            UserRole::Admin => { /* imposible si role != admin, pero por completitud */ }
            UserRole::Client => {
                if client_id != auth.user_id {
                    return Err(AppError::Forbidden("No tienes acceso a esta orden".into()));
                }
            }
            UserRole::Employee => {
                if order.assigned_employee_id != Some(auth.user_id) {
                    return Err(AppError::Forbidden("No tienes acceso a esta orden".into()));
                }
            }
        }
    }

    Ok(Json(serde_json::json!({
        "order": order,
        "phases": phases,
    })))
}

/// Asignar empleado a una orden (solo admin)
#[utoipa::path(
    put,
    path = "/api/orders/{order_id}/assign/{employee_id}",
    params(
        ("order_id" = Uuid, Path, description = "ID de la orden"),
        ("employee_id" = Uuid, Path, description = "ID del empleado"),
    ),
    responses(
        (status = 200, description = "Orden asignada"),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
        (status = 403, description = "Sin permisos", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "orders"
)]
pub async fn assign_order(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((order_id, employee_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, AppError> {
    auth.require_role(&[UserRole::Admin])?;

    let order = OrderService::assign_order(&state.pool, order_id, employee_id).await?;
    Ok(Json(serde_json::json!({ "status": order.status, "assigned_employee_id": order.assigned_employee_id })))
}

/* ============================================================
   CAMBIO DE ROL (admin)
   ============================================================ */

/// Cambiar el `active_role` del admin (permite operar como otro tipo de usuario)
#[utoipa::path(
    post,
    path = "/api/auth/switch-role",
    request_body = SwitchRoleRequest,
    responses(
        (status = 200, description = "Rol cambiado, nuevo token"),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
        (status = 403, description = "Solo admins pueden cambiar rol", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "auth"
)]
pub async fn switch_role(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<SwitchRoleRequest>,
) -> Result<Json<crate::models::AuthResponse>, AppError> {
    /* Solo admin real puede cambiar de rol */
    if auth.role != UserRole::Admin {
        return Err(AppError::Forbidden(
            "Solo administradores pueden cambiar de rol".into(),
        ));
    }

    /* Actualizar en BD y generar nuevo token con el rol efectivo cambiado */
    let user = UserRepository::update_active_role(&state.pool, auth.user_id, Some(req.role)).await?;
    let effective = user.effective_role();
    let token = AuthService::generate_token(user.id, user.role, effective, &state.jwt_secret)?;

    Ok(Json(crate::models::AuthResponse {
        token,
        user_id: user.id,
        role: user.role,
        effective_role: effective,
    }))
}

/* ============================================================
   [044A-38 Fase 2] CANCELAR, ENTREGAR, APROBAR, REVISIÓN
   ============================================================ */

/// Cancelar una orden (solo el cliente dueño o admin, solo en estados iniciales)
#[utoipa::path(
    post,
    path = "/api/orders/{order_id}/cancel",
    params(("order_id" = Uuid, Path, description = "ID de la orden")),
    responses(
        (status = 200, description = "Orden cancelada"),
        (status = 400, description = "Estado no permite cancelación", body = crate::errors::ErrorResponse),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
        (status = 403, description = "Sin permisos", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "orders"
)]
pub async fn cancel_order_handler(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(order_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let order = OrderService::cancel_order(&state.pool, order_id, auth.user_id, auth.effective_role).await?;
    Ok(Json(serde_json::json!({ "status": order.status })))
}

/* [044A-38 Fase 6] deliver_phase movido a handlers/deliverables.rs con soporte multipart.
 * La ruta POST /orders/:id/phases/:n/deliver ahora acepta archivos + notas. */

/// Cliente aprueba una fase entregada
#[utoipa::path(
    put,
    path = "/api/orders/{order_id}/phases/{phase_number}/approve",
    params(
        ("order_id" = Uuid, Path, description = "ID de la orden"),
        ("phase_number" = i32, Path, description = "Número de fase"),
    ),
    responses(
        (status = 200, description = "Fase aprobada", body = crate::models::OrderPhaseResponse),
        (status = 400, description = "Estado no permite aprobación", body = crate::errors::ErrorResponse),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
        (status = 403, description = "Solo el cliente dueño", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "orders"
)]
pub async fn approve_phase(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((order_id, phase_number)): Path<(Uuid, i32)>,
) -> Result<Json<crate::models::OrderPhaseResponse>, AppError> {
    auth.require_role(&[UserRole::Client, UserRole::Admin])?;
    let phase = OrderService::approve_phase(&state.pool, order_id, phase_number, auth.user_id).await?;

    /* [044A-38 Fase 3] Si la orden se completó, capturar los pagos retenidos en Stripe */
    if let Some(order) = OrderRepository::find_order_by_id(&state.pool, order_id).await? {
        if order.status == OrderStatus::Completed {
            if let Some(ref stripe_key) = state.stripe_secret_key {
                if let Err(e) = PaymentService::capture_held_payments(
                    &state.pool,
                    &state.http_client,
                    stripe_key,
                    order_id,
                )
                .await
                {
                    tracing::error!("Error capturando pagos de orden {order_id}: {e}");
                }
            }
        }
    }

    Ok(Json(crate::models::OrderPhaseResponse::from(phase)))
}

/// Cliente solicita revisión de una fase entregada
#[utoipa::path(
    put,
    path = "/api/orders/{order_id}/phases/{phase_number}/revision",
    params(
        ("order_id" = Uuid, Path, description = "ID de la orden"),
        ("phase_number" = i32, Path, description = "Número de fase"),
    ),
    responses(
        (status = 200, description = "Revisión solicitada", body = crate::models::OrderPhaseResponse),
        (status = 400, description = "No se puede pedir revisión", body = crate::errors::ErrorResponse),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
        (status = 403, description = "Solo el cliente dueño", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "orders"
)]
pub async fn request_revision(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((order_id, phase_number)): Path<(Uuid, i32)>,
) -> Result<Json<crate::models::OrderPhaseResponse>, AppError> {
    auth.require_role(&[UserRole::Client, UserRole::Admin])?;
    let phase = OrderService::request_revision(&state.pool, order_id, phase_number, auth.user_id).await?;
    Ok(Json(crate::models::OrderPhaseResponse::from(phase)))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/orders", post(create_order).get(list_orders))
        .route("/orders/:order_id", get(get_order))
        .route("/orders/:order_id/cancel", post(cancel_order_handler))
        .route(
            "/orders/:order_id/assign/:employee_id",
            put(assign_order),
        )
        .route("/orders/:order_id/phases/:phase_number/approve", put(approve_phase))
        .route("/orders/:order_id/phases/:phase_number/revision", put(request_revision))
        .route("/auth/switch-role", post(switch_role))
}

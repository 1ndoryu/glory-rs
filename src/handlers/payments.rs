/* [044A-38 Fase 3] Handlers de pagos: Stripe checkout, webhook, historial.
 * Webhook no requiere auth — se verifica con firma HMAC-SHA256. */

use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::AuthUser;
use crate::models::{InitiatePaymentRequest, PaymentIntentResponse, PaymentResponse, UserRole};
use crate::repositories::OrderRepository;
use crate::services::PaymentService;
use crate::AppState;

/// Iniciar pago de una orden (crea `PaymentIntent` en Stripe)
#[utoipa::path(
    post,
    path = "/api/orders/{order_id}/pay",
    params(("order_id" = Uuid, Path, description = "ID de la orden")),
    request_body = InitiatePaymentRequest,
    responses(
        (status = 200, description = "PaymentIntent creado", body = PaymentIntentResponse),
        (status = 400, description = "Datos inválidos", body = crate::errors::ErrorResponse),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "payments"
)]
pub async fn initiate_payment(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(order_id): Path<Uuid>,
    Json(req): Json<InitiatePaymentRequest>,
) -> Result<Json<PaymentIntentResponse>, AppError> {
    auth.require_role(&[UserRole::Client, UserRole::Admin])?;

    let stripe_key = state
        .stripe_secret_key
        .as_ref()
        .ok_or_else(|| AppError::Internal("Stripe no está configurado".into()))?;

    /* [064A-65] Admin puede pagar cualquier orden (testing/soporte).
     * Para clientes, el servicio verifica ownership por client_id. */
    let caller_id = if auth.role == UserRole::Admin {
        None
    } else {
        Some(auth.user_id)
    };

    let result = PaymentService::initiate_payment(
        &state.pool,
        &state.http_client,
        stripe_key,
        order_id,
        caller_id,
        req.phase_number,
    )
    .await?;

    Ok(Json(result))
}

/// Webhook de Stripe — sin autenticación, verificado por firma HMAC
#[utoipa::path(
    post,
    path = "/api/webhooks/stripe",
    responses(
        (status = 200, description = "Webhook procesado"),
        (status = 400, description = "Firma inválida", body = crate::errors::ErrorResponse),
    ),
    tag = "payments"
)]
pub async fn stripe_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<StatusCode, AppError> {
    let webhook_secret = state
        .stripe_webhook_secret
        .as_ref()
        .ok_or_else(|| AppError::Internal("Stripe webhook secret no configurado".into()))?;

    let signature = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::BadRequest("Missing Stripe-Signature header".into()))?;

    PaymentService::verify_webhook_signature(&body, signature, webhook_secret)?;

    let event: serde_json::Value = serde_json::from_slice(&body)
        .map_err(|e| AppError::BadRequest(format!("JSON inválido: {e}")))?;

    let event_type = event["type"]
        .as_str()
        .ok_or_else(|| AppError::BadRequest("Missing event type".into()))?;

    PaymentService::handle_webhook(&state.pool, event_type, &event["data"]).await?;

    Ok(StatusCode::OK)
}

/// Historial de pagos de una orden
#[utoipa::path(
    get,
    path = "/api/orders/{order_id}/payments",
    params(("order_id" = Uuid, Path, description = "ID de la orden")),
    responses(
        (status = 200, description = "Historial de pagos", body = Vec<PaymentResponse>),
        (status = 401, description = "No autorizado", body = crate::errors::ErrorResponse),
        (status = 404, description = "Orden no encontrada", body = crate::errors::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "payments"
)]
pub async fn list_payments(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(order_id): Path<Uuid>,
) -> Result<Json<Vec<PaymentResponse>>, AppError> {
    /* Verificar acceso: dueño, asignado o admin */
    let order = OrderRepository::find_order_by_id(&state.pool, order_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Orden no encontrada".into()))?;

    match auth.effective_role {
        UserRole::Admin => {}
        UserRole::Client => {
            if order.client_id != auth.user_id {
                return Err(AppError::Forbidden("No tienes acceso a esta orden".into()));
            }
        }
        UserRole::Employee => {
            if order.assigned_employee_id != Some(auth.user_id) {
                return Err(AppError::Forbidden("No tienes acceso a esta orden".into()));
            }
        }
    }

    let payments = PaymentService::list_payments(&state.pool, order_id).await?;
    Ok(Json(payments))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/orders/:order_id/pay", post(initiate_payment))
        .route("/orders/:order_id/payments", get(list_payments))
        .route("/webhooks/stripe", post(stripe_webhook))
}

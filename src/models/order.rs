/* [044A-38] Modelos del marketplace: órdenes, fases, servicios, pagos.
 * Mapean directamente a las tablas de la migración 20260404. */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use super::UserRole;

/* ============================================================
   ENUMS — mapean a tipos PostgreSQL
   ============================================================ */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "order_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    PendingPayment,
    PaymentHeld,
    AwaitingAssignment,
    InProgress,
    UnderReview,
    Completed,
    Cancelled,
    Disputed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "payment_mode", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PaymentMode {
    Full,
    HalfHalf,
    Phased,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "phase_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PhaseStatus {
    Locked,
    PendingPayment,
    Paid,
    InProgress,
    Delivered,
    RevisionRequested,
    Approved,
    Skipped,
}

/* ============================================================
   MODELOS DE BD
   ============================================================ */

#[derive(Debug, Clone, FromRow)]
pub struct ServiceRecord {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub base_price_cents: i32,
    pub currency: String,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct ServicePlan {
    pub id: Uuid,
    pub service_id: Uuid,
    pub slug: String,
    pub name: String,
    pub price_cents: i32,
    pub description: Option<String>,
    pub features: serde_json::Value,
    pub is_highlighted: bool,
    pub is_custom: bool,
    pub stripe_price_id: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct ServicePlanPhase {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub phase_number: i32,
    pub title: String,
    pub description: Option<String>,
    pub percentage_of_total: i32,
    pub estimated_days: i32,
    pub max_revisions: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct Order {
    pub id: Uuid,
    pub order_number: i32,
    pub client_id: Uuid,
    pub service_id: Uuid,
    pub plan_id: Uuid,
    pub payment_mode: PaymentMode,
    pub base_price_cents: i32,
    pub discount_percent: i32,
    pub final_price_cents: i32,
    pub currency: String,
    pub status: OrderStatus,
    pub assigned_employee_id: Option<Uuid>,
    pub assigned_at: Option<DateTime<Utc>>,
    pub auto_assign_deadline: Option<DateTime<Utc>>,
    pub current_phase: i32,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub client_notes: Option<String>,
    pub internal_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct OrderPhase {
    pub id: Uuid,
    pub order_id: Uuid,
    pub phase_number: i32,
    pub title: String,
    pub description: Option<String>,
    pub price_cents: i32,
    pub status: PhaseStatus,
    pub max_revisions: i32,
    pub revisions_used: i32,
    pub estimated_days: i32,
    pub started_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub deadline: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/* ============================================================
   REQUESTS
   ============================================================ */

/// Request para crear una orden (contratar servicio)
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateOrderRequest {
    pub service_slug: String,
    pub plan_slug: String,
    pub payment_mode: PaymentMode,
    #[validate(length(max = 2000))]
    pub client_notes: Option<String>,
}

/// Request para cambiar el `active_role` del admin
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SwitchRoleRequest {
    pub role: UserRole,
}

/* ============================================================
   RESPONSES
   ============================================================ */

#[derive(Debug, Serialize, ToSchema)]
pub struct OrderResponse {
    pub id: Uuid,
    pub order_number: i32,
    pub service_title: String,
    pub service_slug: String,
    pub plan_name: String,
    pub payment_mode: PaymentMode,
    pub base_price_cents: i32,
    pub discount_percent: i32,
    pub final_price_cents: i32,
    pub currency: String,
    pub status: OrderStatus,
    pub assigned_employee_id: Option<Uuid>,
    pub current_phase: i32,
    pub total_phases: i32,
    pub client_notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrderPhaseResponse {
    pub phase_number: i32,
    pub title: String,
    pub description: Option<String>,
    pub price_cents: i32,
    pub status: PhaseStatus,
    pub max_revisions: i32,
    pub revisions_used: i32,
    pub estimated_days: i32,
    pub deadline: Option<DateTime<Utc>>,
}

impl From<OrderPhase> for OrderPhaseResponse {
    fn from(p: OrderPhase) -> Self {
        Self {
            phase_number: p.phase_number,
            title: p.title,
            description: p.description,
            price_cents: p.price_cents,
            status: p.status,
            max_revisions: p.max_revisions,
            revisions_used: p.revisions_used,
            estimated_days: p.estimated_days,
            deadline: p.deadline,
        }
    }
}

/// Detalle completo de un servicio con sus planes
#[derive(Debug, Serialize, ToSchema)]
pub struct ServiceDetailResponse {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub plans: Vec<ServicePlanResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ServicePlanResponse {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub price_cents: i32,
    pub description: Option<String>,
    pub features: serde_json::Value,
    pub is_highlighted: bool,
    pub is_custom: bool,
    pub phases: Vec<ServicePlanPhaseResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ServicePlanPhaseResponse {
    pub phase_number: i32,
    pub title: String,
    pub description: Option<String>,
    pub percentage_of_total: i32,
    pub estimated_days: i32,
    pub max_revisions: i32,
}

impl From<ServicePlanPhase> for ServicePlanPhaseResponse {
    fn from(p: ServicePlanPhase) -> Self {
        Self {
            phase_number: p.phase_number,
            title: p.title,
            description: p.description,
            percentage_of_total: p.percentage_of_total,
            estimated_days: p.estimated_days,
            max_revisions: p.max_revisions,
        }
    }
}

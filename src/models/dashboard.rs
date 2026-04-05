/* [044A-38 Fase 10] Modelos del dashboard admin.
 * Estructuras para métricas, revenue y alertas del panel de administración. */

use serde::Serialize;
use utoipa::ToSchema;

/* ========== Revenue ========== */

#[derive(Debug, Serialize, ToSchema)]
pub struct RevenueStats {
    pub total_revenue: f64,
    pub monthly_revenue: f64,
    pub held_amount: f64,
    pub refunded_amount: f64,
}

/* ========== Contadores de órdenes ========== */

#[derive(Debug, Serialize, ToSchema)]
pub struct OrderCounts {
    pub total: i64,
    pub active: i64,
    pub completed: i64,
    pub cancelled: i64,
    pub awaiting_assignment: i64,
}

/* ========== Rendimiento de empleados ========== */

#[derive(Debug, Serialize, ToSchema)]
pub struct EmployeePerformance {
    pub employee_id: String,
    pub email: String,
    pub active_orders: i64,
    pub completed_orders: i64,
    pub average_rating: Option<f64>,
}

/* ========== Alertas ========== */

#[derive(Debug, Serialize, ToSchema)]
pub struct DashboardAlerts {
    pub unassigned_orders: i64,
    pub pending_refunds: i64,
    pub overdue_orders: i64,
    pub unread_admin_notifications: i64,
}

/* ========== Respuesta consolidada del dashboard ========== */

#[derive(Debug, Serialize, ToSchema)]
pub struct DashboardResponse {
    pub revenue: RevenueStats,
    pub orders: OrderCounts,
    pub employees: Vec<EmployeePerformance>,
    pub alerts: DashboardAlerts,
}

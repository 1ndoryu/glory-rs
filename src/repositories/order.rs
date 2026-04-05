/* [044A-38] Repositorio de órdenes: CRUD sobre orders, order_phases, services, service_plans.
 * Todas las queries usan prepared statements vía sqlx::query_as. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    Order, OrderPhase, OrderStatus, PaymentMode, PhaseStatus,
    ServicePlan, ServicePlanPhase, ServiceRecord,
};

/* Structs de parámetros para evitar too_many_arguments (clippy) */

pub struct CreateOrderParams<'a> {
    pub client_id: Uuid,
    pub service_id: Uuid,
    pub plan_id: Uuid,
    pub payment_mode: PaymentMode,
    pub base_price_cents: i32,
    pub discount_percent: i32,
    pub final_price_cents: i32,
    pub client_notes: Option<&'a str>,
}

pub struct CreatePhaseParams<'a> {
    pub order_id: Uuid,
    pub phase_number: i32,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub price_cents: i32,
    pub status: PhaseStatus,
    pub max_revisions: i32,
    pub estimated_days: i32,
}

pub struct OrderRepository;

impl OrderRepository {
    /* ============================================================
       SERVICIOS (catálogo — solo lectura)
       ============================================================ */

    pub async fn list_services(pool: &PgPool) -> Result<Vec<ServiceRecord>, sqlx::Error> {
        sqlx::query_as::<_, ServiceRecord>(
            "SELECT id, slug, title, description, base_price_cents, currency, is_active, sort_order, created_at \
             FROM services WHERE is_active = true ORDER BY sort_order",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_service_by_slug(
        pool: &PgPool,
        slug: &str,
    ) -> Result<Option<ServiceRecord>, sqlx::Error> {
        sqlx::query_as::<_, ServiceRecord>(
            "SELECT id, slug, title, description, base_price_cents, currency, is_active, sort_order, created_at \
             FROM services WHERE slug = $1 AND is_active = true",
        )
        .bind(slug)
        .fetch_optional(pool)
        .await
    }

    pub async fn list_plans_for_service(
        pool: &PgPool,
        service_id: Uuid,
    ) -> Result<Vec<ServicePlan>, sqlx::Error> {
        sqlx::query_as::<_, ServicePlan>(
            "SELECT id, service_id, slug, name, price_cents, description, features, \
             is_highlighted, is_custom, stripe_price_id, sort_order \
             FROM service_plans WHERE service_id = $1 ORDER BY sort_order",
        )
        .bind(service_id)
        .fetch_all(pool)
        .await
    }

    pub async fn find_plan_by_slug(
        pool: &PgPool,
        service_id: Uuid,
        plan_slug: &str,
    ) -> Result<Option<ServicePlan>, sqlx::Error> {
        sqlx::query_as::<_, ServicePlan>(
            "SELECT id, service_id, slug, name, price_cents, description, features, \
             is_highlighted, is_custom, stripe_price_id, sort_order \
             FROM service_plans WHERE service_id = $1 AND slug = $2",
        )
        .bind(service_id)
        .bind(plan_slug)
        .fetch_all(pool)
        .await
        .map(|mut v| v.pop())
    }

    pub async fn list_plan_phases(
        pool: &PgPool,
        plan_id: Uuid,
    ) -> Result<Vec<ServicePlanPhase>, sqlx::Error> {
        sqlx::query_as::<_, ServicePlanPhase>(
            "SELECT id, plan_id, phase_number, title, description, \
             percentage_of_total, estimated_days, max_revisions \
             FROM service_plan_phases WHERE plan_id = $1 ORDER BY phase_number",
        )
        .bind(plan_id)
        .fetch_all(pool)
        .await
    }

    /* ============================================================
       ÓRDENES
       ============================================================ */

    pub async fn create_order(
        pool: &PgPool,
        params: CreateOrderParams<'_>,
    ) -> Result<Order, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            "INSERT INTO orders (client_id, service_id, plan_id, payment_mode, \
             base_price_cents, discount_percent, final_price_cents, client_notes) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) \
             RETURNING *",
        )
        .bind(params.client_id)
        .bind(params.service_id)
        .bind(params.plan_id)
        .bind(params.payment_mode)
        .bind(params.base_price_cents)
        .bind(params.discount_percent)
        .bind(params.final_price_cents)
        .bind(params.client_notes)
        .fetch_one(pool)
        .await
    }

    pub async fn find_order_by_id(
        pool: &PgPool,
        order_id: Uuid,
    ) -> Result<Option<Order>, sqlx::Error> {
        sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1")
            .bind(order_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn list_orders_for_client(
        pool: &PgPool,
        client_id: Uuid,
    ) -> Result<Vec<Order>, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            "SELECT * FROM orders WHERE client_id = $1 ORDER BY created_at DESC",
        )
        .bind(client_id)
        .fetch_all(pool)
        .await
    }

    pub async fn list_orders_for_employee(
        pool: &PgPool,
        employee_id: Uuid,
    ) -> Result<Vec<Order>, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            "SELECT * FROM orders WHERE assigned_employee_id = $1 ORDER BY created_at DESC",
        )
        .bind(employee_id)
        .fetch_all(pool)
        .await
    }

    pub async fn list_all_orders(pool: &PgPool) -> Result<Vec<Order>, sqlx::Error> {
        sqlx::query_as::<_, Order>("SELECT * FROM orders ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn list_unassigned_orders(pool: &PgPool) -> Result<Vec<Order>, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            "SELECT * FROM orders WHERE status = 'awaiting_assignment' ORDER BY created_at ASC",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn update_order_status(
        pool: &PgPool,
        order_id: Uuid,
        status: OrderStatus,
    ) -> Result<Order, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            "UPDATE orders SET status = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(order_id)
        .bind(status)
        .fetch_one(pool)
        .await
    }

    pub async fn assign_order(
        pool: &PgPool,
        order_id: Uuid,
        employee_id: Uuid,
    ) -> Result<Order, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            "UPDATE orders SET assigned_employee_id = $2, assigned_at = NOW(), \
             status = 'in_progress', started_at = COALESCE(started_at, NOW()), updated_at = NOW() \
             WHERE id = $1 RETURNING *",
        )
        .bind(order_id)
        .bind(employee_id)
        .fetch_one(pool)
        .await
    }

    /* ============================================================
       FASES DE ORDEN
       ============================================================ */

    pub async fn create_order_phase(
        pool: &PgPool,
        params: CreatePhaseParams<'_>,
    ) -> Result<OrderPhase, sqlx::Error> {
        sqlx::query_as::<_, OrderPhase>(
            "INSERT INTO order_phases (order_id, phase_number, title, description, \
             price_cents, status, max_revisions, estimated_days) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
        )
        .bind(params.order_id)
        .bind(params.phase_number)
        .bind(params.title)
        .bind(params.description)
        .bind(params.price_cents)
        .bind(params.status)
        .bind(params.max_revisions)
        .bind(params.estimated_days)
        .fetch_one(pool)
        .await
    }

    pub async fn list_order_phases(
        pool: &PgPool,
        order_id: Uuid,
    ) -> Result<Vec<OrderPhase>, sqlx::Error> {
        sqlx::query_as::<_, OrderPhase>(
            "SELECT * FROM order_phases WHERE order_id = $1 ORDER BY phase_number",
        )
        .bind(order_id)
        .fetch_all(pool)
        .await
    }

    pub async fn update_phase_status(
        pool: &PgPool,
        phase_id: Uuid,
        status: PhaseStatus,
    ) -> Result<OrderPhase, sqlx::Error> {
        sqlx::query_as::<_, OrderPhase>(
            "UPDATE order_phases SET status = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(phase_id)
        .bind(status)
        .fetch_one(pool)
        .await
    }

    /* [044A-38 Fase 2] Busca una fase por order_id + phase_number */
    pub async fn find_phase_by_number(
        pool: &PgPool,
        order_id: Uuid,
        phase_number: i32,
    ) -> Result<Option<OrderPhase>, sqlx::Error> {
        sqlx::query_as::<_, OrderPhase>(
            "SELECT * FROM order_phases WHERE order_id = $1 AND phase_number = $2",
        )
        .bind(order_id)
        .bind(phase_number)
        .fetch_optional(pool)
        .await
    }

    /* [044A-38 Fase 2] Marca fase como entregada */
    pub async fn deliver_phase(
        pool: &PgPool,
        phase_id: Uuid,
    ) -> Result<OrderPhase, sqlx::Error> {
        sqlx::query_as::<_, OrderPhase>(
            "UPDATE order_phases SET status = 'delivered', delivered_at = NOW(), updated_at = NOW() \
             WHERE id = $1 RETURNING *",
        )
        .bind(phase_id)
        .fetch_one(pool)
        .await
    }

    /* [044A-38 Fase 2] Marca fase como aprobada */
    pub async fn approve_phase(
        pool: &PgPool,
        phase_id: Uuid,
    ) -> Result<OrderPhase, sqlx::Error> {
        sqlx::query_as::<_, OrderPhase>(
            "UPDATE order_phases SET status = 'approved', approved_at = NOW(), updated_at = NOW() \
             WHERE id = $1 RETURNING *",
        )
        .bind(phase_id)
        .fetch_one(pool)
        .await
    }

    /* [044A-38 Fase 2] Incrementa revisiones y pone status revision_requested */
    pub async fn request_revision(
        pool: &PgPool,
        phase_id: Uuid,
    ) -> Result<OrderPhase, sqlx::Error> {
        sqlx::query_as::<_, OrderPhase>(
            "UPDATE order_phases SET status = 'revision_requested', \
             revisions_used = revisions_used + 1, updated_at = NOW() \
             WHERE id = $1 RETURNING *",
        )
        .bind(phase_id)
        .fetch_one(pool)
        .await
    }

    /* [044A-38 Fase 2] Cancela una orden: marca cancelled + timestamp */
    pub async fn cancel_order(
        pool: &PgPool,
        order_id: Uuid,
    ) -> Result<Order, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            "UPDATE orders SET status = 'cancelled', cancelled_at = NOW(), updated_at = NOW() \
             WHERE id = $1 RETURNING *",
        )
        .bind(order_id)
        .fetch_one(pool)
        .await
    }

    /* [044A-38 Fase 2] Actualiza current_phase de una orden */
    pub async fn update_current_phase(
        pool: &PgPool,
        order_id: Uuid,
        phase_number: i32,
    ) -> Result<Order, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            "UPDATE orders SET current_phase = $2, updated_at = NOW() \
             WHERE id = $1 RETURNING *",
        )
        .bind(order_id)
        .bind(phase_number)
        .fetch_one(pool)
        .await
    }

    /* [044A-38 Fase 2] Marca orden como completada */
    pub async fn complete_order(
        pool: &PgPool,
        order_id: Uuid,
    ) -> Result<Order, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            "UPDATE orders SET status = 'completed', completed_at = NOW(), updated_at = NOW() \
             WHERE id = $1 RETURNING *",
        )
        .bind(order_id)
        .fetch_one(pool)
        .await
    }

    /// Obtiene el título del servicio y nombre del plan para un order
    pub async fn get_order_display_info(
        pool: &PgPool,
        service_id: Uuid,
        plan_id: Uuid,
    ) -> Result<(String, String), sqlx::Error> {
        let service_title: String = sqlx::query_scalar(
            "SELECT title FROM services WHERE id = $1",
        )
        .bind(service_id)
        .fetch_one(pool)
        .await?;

        let plan_name: String = sqlx::query_scalar(
            "SELECT name FROM service_plans WHERE id = $1",
        )
        .bind(plan_id)
        .fetch_one(pool)
        .await?;

        Ok((service_title, plan_name))
    }

    /* ============================================================
       [044A-38 Fase 4] ASIGNACIÓN Y AUTO-ASIGNACIÓN
       ============================================================ */

    /// Transiciona orden a `awaiting_assignment` y establece deadline de 24h para auto-asignación
    pub async fn set_awaiting_assignment(
        pool: &PgPool,
        order_id: Uuid,
    ) -> Result<Order, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            "UPDATE orders SET status = 'awaiting_assignment', \
             auto_assign_deadline = NOW() + INTERVAL '24 hours', \
             updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(order_id)
        .fetch_one(pool)
        .await
    }

    /// Órdenes que pasaron su deadline de auto-asignación (24h en `awaiting_assignment`)
    pub async fn list_overdue_unassigned(pool: &PgPool) -> Result<Vec<Order>, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            "SELECT * FROM orders WHERE status = 'awaiting_assignment' \
             AND auto_assign_deadline IS NOT NULL AND auto_assign_deadline < NOW() \
             ORDER BY auto_assign_deadline ASC",
        )
        .fetch_all(pool)
        .await
    }

    /// Busca servicio por UUID (para resolver slug en auto-asignación)
    pub async fn find_service_by_id(
        pool: &PgPool,
        service_id: Uuid,
    ) -> Result<Option<ServiceRecord>, sqlx::Error> {
        sqlx::query_as::<_, ServiceRecord>(
            "SELECT id, slug, title, description, base_price_cents, currency, is_active, sort_order, created_at \
             FROM services WHERE id = $1",
        )
        .bind(service_id)
        .fetch_optional(pool)
        .await
    }
}

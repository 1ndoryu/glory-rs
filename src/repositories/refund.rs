/* [044A-38 Fase 7] Repositorio de reembolsos.
 * CRUD sobre order_refunds: crear solicitud, listar, aprobar/rechazar/completar.
 * Solo un reembolso activo por orden (constraint en handler). */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::OrderRefund;

pub struct RefundRepository;

impl RefundRepository {
    /// Crear solicitud de reembolso
    pub async fn create(
        pool: &PgPool,
        order_id: Uuid,
        payment_id: Uuid,
        requested_by: Uuid,
        amount_cents: i32,
        reason: &str,
    ) -> Result<OrderRefund, sqlx::Error> {
        sqlx::query_as::<_, OrderRefund>(
            "INSERT INTO order_refunds (order_id, payment_id, requested_by, amount_cents, reason)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING *",
        )
        .bind(order_id)
        .bind(payment_id)
        .bind(requested_by)
        .bind(amount_cents)
        .bind(reason)
        .fetch_one(pool)
        .await
    }

    /// Buscar reembolso por ID
    pub async fn find_by_id(
        pool: &PgPool,
        refund_id: Uuid,
    ) -> Result<Option<OrderRefund>, sqlx::Error> {
        sqlx::query_as::<_, OrderRefund>(
            "SELECT * FROM order_refunds WHERE id = $1",
        )
        .bind(refund_id)
        .fetch_optional(pool)
        .await
    }

    /// Buscar reembolso activo (no rejected/completed) para una orden
    pub async fn find_active_for_order(
        pool: &PgPool,
        order_id: Uuid,
    ) -> Result<Option<OrderRefund>, sqlx::Error> {
        sqlx::query_as::<_, OrderRefund>(
            "SELECT * FROM order_refunds
             WHERE order_id = $1
               AND status NOT IN ('rejected', 'completed')
             ORDER BY requested_at DESC
             LIMIT 1",
        )
        .bind(order_id)
        .fetch_optional(pool)
        .await
    }

    /// Listar todos los reembolsos pendientes (admin)
    pub async fn list_pending(
        pool: &PgPool,
    ) -> Result<Vec<OrderRefund>, sqlx::Error> {
        sqlx::query_as::<_, OrderRefund>(
            "SELECT * FROM order_refunds
             WHERE status IN ('requested', 'under_review', 'approved')
             ORDER BY requested_at ASC",
        )
        .fetch_all(pool)
        .await
    }

    /// Listar reembolsos de un cliente específico
    pub async fn list_for_user(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<OrderRefund>, sqlx::Error> {
        sqlx::query_as::<_, OrderRefund>(
            "SELECT * FROM order_refunds
             WHERE requested_by = $1
             ORDER BY requested_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// Buscar reembolso de una orden para un cliente
    pub async fn find_for_order(
        pool: &PgPool,
        order_id: Uuid,
    ) -> Result<Option<OrderRefund>, sqlx::Error> {
        sqlx::query_as::<_, OrderRefund>(
            "SELECT * FROM order_refunds
             WHERE order_id = $1
             ORDER BY requested_at DESC
             LIMIT 1",
        )
        .bind(order_id)
        .fetch_optional(pool)
        .await
    }

    /// Aprobar reembolso (admin)
    pub async fn approve(
        pool: &PgPool,
        refund_id: Uuid,
        admin_id: Uuid,
        admin_response: Option<&str>,
    ) -> Result<OrderRefund, sqlx::Error> {
        sqlx::query_as::<_, OrderRefund>(
            "UPDATE order_refunds
             SET status = 'approved',
                 reviewed_by = $2,
                 admin_response = $3,
                 reviewed_at = NOW()
             WHERE id = $1
             RETURNING *",
        )
        .bind(refund_id)
        .bind(admin_id)
        .bind(admin_response)
        .fetch_one(pool)
        .await
    }

    /// Rechazar reembolso (admin)
    pub async fn reject(
        pool: &PgPool,
        refund_id: Uuid,
        admin_id: Uuid,
        admin_response: Option<&str>,
    ) -> Result<OrderRefund, sqlx::Error> {
        sqlx::query_as::<_, OrderRefund>(
            "UPDATE order_refunds
             SET status = 'rejected',
                 reviewed_by = $2,
                 admin_response = $3,
                 reviewed_at = NOW()
             WHERE id = $1
             RETURNING *",
        )
        .bind(refund_id)
        .bind(admin_id)
        .bind(admin_response)
        .fetch_one(pool)
        .await
    }

    /// Marcar reembolso como completado (Stripe refund exitoso)
    pub async fn mark_completed(
        pool: &PgPool,
        refund_id: Uuid,
        stripe_refund_id: &str,
    ) -> Result<OrderRefund, sqlx::Error> {
        sqlx::query_as::<_, OrderRefund>(
            "UPDATE order_refunds
             SET status = 'completed',
                 stripe_refund_id = $2,
                 completed_at = NOW()
             WHERE id = $1
             RETURNING *",
        )
        .bind(refund_id)
        .bind(stripe_refund_id)
        .fetch_one(pool)
        .await
    }

    /// Actualizar status a `under_review`
    pub async fn set_under_review(
        pool: &PgPool,
        refund_id: Uuid,
    ) -> Result<OrderRefund, sqlx::Error> {
        sqlx::query_as::<_, OrderRefund>(
            "UPDATE order_refunds
             SET status = 'under_review'
             WHERE id = $1
             RETURNING *",
        )
        .bind(refund_id)
        .fetch_one(pool)
        .await
    }
}

/* [044A-38 Fase 3] Repositorio de pagos: CRUD sobre order_payments.
 * Todas las queries usan prepared statements vía sqlx::query_as. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{OrderPayment, PaymentMode, PaymentStatus};

pub struct CreatePaymentParams<'a> {
    pub order_id: Uuid,
    pub phase_id: Option<Uuid>,
    pub amount_cents: i32,
    pub currency: &'a str,
    pub payment_mode: PaymentMode,
    pub stripe_payment_intent_id: &'a str,
    pub description: Option<&'a str>,
}

pub struct PaymentRepository;

impl PaymentRepository {
    pub async fn create_payment(
        pool: &PgPool,
        params: CreatePaymentParams<'_>,
    ) -> Result<OrderPayment, sqlx::Error> {
        sqlx::query_as::<_, OrderPayment>(
            "INSERT INTO order_payments (order_id, phase_id, amount_cents, currency, status, \
             payment_mode, stripe_payment_intent_id, description) \
             VALUES ($1, $2, $3, $4, 'pending', $5, $6, $7) RETURNING *",
        )
        .bind(params.order_id)
        .bind(params.phase_id)
        .bind(params.amount_cents)
        .bind(params.currency)
        .bind(params.payment_mode)
        .bind(params.stripe_payment_intent_id)
        .bind(params.description)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_stripe_intent(
        pool: &PgPool,
        stripe_pi_id: &str,
    ) -> Result<Option<OrderPayment>, sqlx::Error> {
        sqlx::query_as::<_, OrderPayment>(
            "SELECT * FROM order_payments WHERE stripe_payment_intent_id = $1",
        )
        .bind(stripe_pi_id)
        .fetch_optional(pool)
        .await
    }

    pub async fn update_status_held(
        pool: &PgPool,
        payment_id: Uuid,
    ) -> Result<OrderPayment, sqlx::Error> {
        sqlx::query_as::<_, OrderPayment>(
            "UPDATE order_payments SET status = 'held', held_at = NOW(), updated_at = NOW() \
             WHERE id = $1 RETURNING *",
        )
        .bind(payment_id)
        .fetch_one(pool)
        .await
    }

    pub async fn update_status_released(
        pool: &PgPool,
        payment_id: Uuid,
    ) -> Result<OrderPayment, sqlx::Error> {
        sqlx::query_as::<_, OrderPayment>(
            "UPDATE order_payments SET status = 'released', released_at = NOW(), updated_at = NOW() \
             WHERE id = $1 RETURNING *",
        )
        .bind(payment_id)
        .fetch_one(pool)
        .await
    }

    pub async fn update_status(
        pool: &PgPool,
        payment_id: Uuid,
        status: PaymentStatus,
    ) -> Result<OrderPayment, sqlx::Error> {
        sqlx::query_as::<_, OrderPayment>(
            "UPDATE order_payments SET status = $2, updated_at = NOW() \
             WHERE id = $1 RETURNING *",
        )
        .bind(payment_id)
        .bind(status)
        .fetch_one(pool)
        .await
    }

    pub async fn update_charge_id(
        pool: &PgPool,
        payment_id: Uuid,
        charge_id: &str,
    ) -> Result<OrderPayment, sqlx::Error> {
        sqlx::query_as::<_, OrderPayment>(
            "UPDATE order_payments SET stripe_charge_id = $2, updated_at = NOW() \
             WHERE id = $1 RETURNING *",
        )
        .bind(payment_id)
        .bind(charge_id)
        .fetch_one(pool)
        .await
    }

    pub async fn list_for_order(
        pool: &PgPool,
        order_id: Uuid,
    ) -> Result<Vec<OrderPayment>, sqlx::Error> {
        sqlx::query_as::<_, OrderPayment>(
            "SELECT * FROM order_payments WHERE order_id = $1 ORDER BY created_at DESC",
        )
        .bind(order_id)
        .fetch_all(pool)
        .await
    }

    pub async fn find_held_for_order(
        pool: &PgPool,
        order_id: Uuid,
    ) -> Result<Vec<OrderPayment>, sqlx::Error> {
        sqlx::query_as::<_, OrderPayment>(
            "SELECT * FROM order_payments WHERE order_id = $1 AND status = 'held' \
             ORDER BY created_at",
        )
        .bind(order_id)
        .fetch_all(pool)
        .await
    }
}

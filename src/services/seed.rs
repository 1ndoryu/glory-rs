/* [064A-62] Servicio de seed: recrear/borrar datos de prueba desde el panel admin.
 * Lógica extraída de examples/seed_test_data.rs. Crea usuarios de test (cliente, empleado)
 * y órdenes en estados variados para testear flujos del marketplace.
 * Solo accesible por admin. */

use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use sqlx::PgPool;
use uuid::Uuid;

const TEST_EMAILS: [&str; 2] = ["cliente@test.com", "empleado@test.com"];

pub struct SeedService;

impl SeedService {
    /// Elimina todos los datos generados por el seed (usuarios test + sus órdenes/fases/chat)
    pub async fn delete_test_data(pool: &PgPool) -> Result<u64, sqlx::Error> {
        let ids: Vec<Uuid> = sqlx::query_scalar(
            "SELECT id FROM users WHERE email = ANY($1)",
        )
        .bind(&TEST_EMAILS[..])
        .fetch_all(pool)
        .await?;

        if ids.is_empty() {
            return Ok(0);
        }

        /* Borrar en orden por FKs: fases → órdenes → chat → usuarios */
        sqlx::query(
            "DELETE FROM order_phases WHERE order_id IN (SELECT id FROM orders WHERE client_id = ANY($1))",
        )
        .bind(&ids)
        .execute(pool)
        .await?;

        sqlx::query("DELETE FROM orders WHERE client_id = ANY($1)")
            .bind(&ids)
            .execute(pool)
            .await?;

        sqlx::query("DELETE FROM chat_messages WHERE session_id IN (SELECT id FROM chat_sessions WHERE user_id = ANY($1))")
            .bind(&ids)
            .execute(pool)
            .await?;

        sqlx::query("DELETE FROM chat_sessions WHERE user_id = ANY($1)")
            .bind(&ids)
            .execute(pool)
            .await?;

        let result = sqlx::query("DELETE FROM users WHERE id = ANY($1)")
            .bind(&ids)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    /// Recrea datos de prueba: borra existentes y crea usuarios + órdenes variadas
    pub async fn recreate_test_data(pool: &PgPool) -> Result<String, sqlx::Error> {
        Self::delete_test_data(pool).await?;

        let client_id = Self::upsert_user(pool, "cliente@test.com", "cliente", "client").await?;
        let employee_id = Self::upsert_user(pool, "empleado@test.com", "empleado", "employee").await?;

        let plans = Self::get_plans(pool).await?;
        if plans.is_empty() {
            return Ok("Usuarios creados, pero no hay planes de servicio para crear órdenes.".into());
        }

        let mut created = 0u32;
        /* Orden 1: full + in_progress */
        if let Some(p) = plans.iter().find(|p| p.slug == "basico" && p.service_title.contains("Sitios Web")) {
            Self::create_seed_order(pool, client_id, p, "full", 20, "in_progress", Some(employee_id)).await?;
            created += 1;
        }
        /* Orden 2: phased + pending_payment */
        if let Some(p) = plans.iter().find(|p| p.slug == "avanzado" && p.service_title.contains("Aplicaciones")) {
            Self::create_seed_order(pool, client_id, p, "phased", 0, "pending_payment", None).await?;
            created += 1;
        }
        /* Orden 3: full + completed */
        if let Some(p) = plans.iter().find(|p| p.slug == "basico" && p.service_title.contains("IA")) {
            Self::create_seed_order(pool, client_id, p, "full", 20, "completed", Some(employee_id)).await?;
            created += 1;
        }
        /* Orden 4: half_half + awaiting_assignment */
        if let Some(p) = plans.iter().find(|p| p.slug == "avanzado" && p.service_title.contains("Marca")) {
            Self::create_seed_order(pool, client_id, p, "half_half", 10, "awaiting_assignment", None).await?;
            created += 1;
        }

        Ok(format!(
            "Seed completado: 2 usuarios + {created} órdenes. Credenciales: cliente@test.com/cliente, empleado@test.com/empleado"
        ))
    }

    async fn upsert_user(pool: &PgPool, email: &str, password: &str, role: &str) -> Result<Uuid, sqlx::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .expect("Error al hashear contraseña")
            .to_string();
        let id = Uuid::new_v4();
        let display_name = {
            let part = email.split('@').next().unwrap_or(email);
            let mut chars = part.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
                None => part.to_string(),
            }
        };

        sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO users (id, email, password_hash, role, display_name)
             VALUES ($1, $2, $3, $4::user_role, $5)
             ON CONFLICT (email) DO UPDATE SET password_hash = $3, role = $4::user_role, display_name = $5
             RETURNING id",
        )
        .bind(id).bind(email).bind(&password_hash).bind(role).bind(&display_name)
        .fetch_one(pool)
        .await
    }

    async fn get_plans(pool: &PgPool) -> Result<Vec<SeedPlan>, sqlx::Error> {
        sqlx::query_as::<_, SeedPlan>(
            "SELECT sp.id, sp.slug, sp.price_cents, s.id as service_id, s.title as service_title
             FROM service_plans sp JOIN services s ON sp.service_id = s.id
             WHERE sp.is_custom = false ORDER BY s.sort_order, sp.sort_order",
        )
        .fetch_all(pool)
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn create_seed_order(
        pool: &PgPool, client_id: Uuid, plan: &SeedPlan,
        payment_mode: &str, discount: i32, status: &str, employee_id: Option<Uuid>,
    ) -> Result<(), sqlx::Error> {
        let final_price = plan.price_cents - (plan.price_cents * discount / 100);
        let started_at = if ["in_progress", "under_review", "completed"].contains(&status) {
            Some(chrono::Utc::now() - chrono::Duration::days(14))
        } else { None };
        let completed_at = if status == "completed" { Some(chrono::Utc::now() - chrono::Duration::days(2)) } else { None };
        let assigned_at = employee_id.map(|_| chrono::Utc::now() - chrono::Duration::days(12));

        let order_id: Uuid = sqlx::query_scalar(
            "INSERT INTO orders (client_id, service_id, plan_id, payment_mode, base_price_cents,
             discount_percent, final_price_cents, status, assigned_employee_id, assigned_at, started_at, completed_at)
             VALUES ($1,$2,$3,$4::payment_mode,$5,$6,$7,$8::order_status,$9,$10,$11,$12) RETURNING id",
        )
        .bind(client_id).bind(plan.service_id).bind(plan.id)
        .bind(payment_mode).bind(plan.price_cents).bind(discount).bind(final_price)
        .bind(status).bind(employee_id).bind(assigned_at).bind(started_at).bind(completed_at)
        .fetch_one(pool)
        .await?;

        /* Crear fases desde template del plan */
        let templates = sqlx::query_as::<_, SeedPhaseTemplate>(
            "SELECT phase_number, title, description, percentage_of_total, estimated_days, max_revisions
             FROM service_plan_phases WHERE plan_id = $1 ORDER BY phase_number",
        )
        .bind(plan.id)
        .fetch_all(pool)
        .await?;

        let total = templates.len();
        for (idx, t) in templates.iter().enumerate() {
            let phase_price = final_price * t.percentage_of_total / 100;
            let phase_status = Self::resolve_phase_status(payment_mode, status, idx, total);
            sqlx::query(
                "INSERT INTO order_phases (order_id, phase_number, title, description, price_cents, status, max_revisions, estimated_days)
                 VALUES ($1,$2,$3,$4,$5,$6::phase_status,$7,$8)",
            )
            .bind(order_id).bind(t.phase_number).bind(&t.title).bind(&t.description)
            .bind(phase_price).bind(phase_status).bind(t.max_revisions).bind(t.estimated_days)
            .execute(pool)
            .await?;
        }
        Ok(())
    }

    fn resolve_phase_status(mode: &str, order_status: &str, idx: usize, total: usize) -> &'static str {
        match (mode, order_status) {
            (_, "completed") => "approved",
            ("full", "under_review") => if idx < total - 1 { "approved" } else { "delivered" },
            ("full" | "half_half", "in_progress") => if idx == 0 { "in_progress" } else { "paid" },
            ("full", _) => "paid",
            ("half_half", "awaiting_assignment") => if idx < (total / 2).max(1) { "paid" } else { "pending_payment" },
            ("phased", "in_progress") => if idx == 0 { "in_progress" } else if idx == 1 { "pending_payment" } else { "locked" },
            ("phased", "pending_payment") => if idx == 0 { "pending_payment" } else { "locked" },
            _ => "locked",
        }
    }
}

#[derive(sqlx::FromRow)]
struct SeedPlan {
    id: Uuid,
    slug: String,
    price_cents: i32,
    service_id: Uuid,
    service_title: String,
}

#[derive(sqlx::FromRow)]
struct SeedPhaseTemplate {
    phase_number: i32,
    title: String,
    description: Option<String>,
    percentage_of_total: i32,
    estimated_days: i32,
    max_revisions: i32,
}

/* [064A-15+064A-21] Seed: datos de prueba realistas para vistas de cliente y empleado.
 * Crea usuarios de prueba (cliente, empleado) y órdenes en estados coherentes.
 * Ejecutar: cargo run --example seed_test_data
 * Requiere DATABASE_URL en .env.
 *
 * Regla clave: órdenes con payment_mode=full NO deben tener status=pending_payment
 * porque el pago único se hace al crear la orden. Solo phased y half_half pueden
 * tener pending_payment como estado inicial. */

use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use argon2::password_hash::rand_core::OsRng;
use sqlx::PgPool;
use uuid::Uuid;

#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar definida en .env");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("No se pudo conectar a la base de datos");

    /* 1. Crear usuarios de prueba */
    let client_id = upsert_user(&pool, "cliente@test.com", "cliente", "client").await;
    let employee_id = upsert_user(&pool, "empleado@test.com", "empleado", "employee").await;

    println!("Usuarios creados: cliente={client_id}, empleado={employee_id}");

    /* [064A-33] Limpiar órdenes previas del cliente de prueba para idempotencia.
     * Borra fases primero (FK), luego órdenes. Solo afecta al usuario de test. */
    sqlx::query("DELETE FROM order_phases WHERE order_id IN (SELECT id FROM orders WHERE client_id = $1)")
        .bind(client_id)
        .execute(&pool)
        .await
        .expect("Error al limpiar fases previas");
    sqlx::query("DELETE FROM orders WHERE client_id = $1")
        .bind(client_id)
        .execute(&pool)
        .await
        .expect("Error al limpiar órdenes previas");
    println!("Órdenes previas del cliente de prueba eliminadas.");

    /* 2. Obtener servicios y planes existentes */
    let plans = sqlx::query_as::<_, PlanRow>(
        "SELECT sp.id, sp.slug, sp.name, sp.price_cents, s.id as service_id, s.title as service_title
         FROM service_plans sp JOIN services s ON sp.service_id = s.id
         WHERE sp.is_custom = false
         ORDER BY s.sort_order, sp.sort_order"
    )
    .fetch_all(&pool)
    .await
    .expect("Error al obtener planes");

    if plans.is_empty() {
        eprintln!("No hay planes de servicio. Ejecuta las migraciones primero.");
        return;
    }

    /* 3. Crear órdenes de prueba con estados realistas
     *
     * [064A-21] Regla: full → nunca pending_payment. El pago único ya ocurrió.
     * - full → in_progress (ya pagado, ya asignado, trabajando)
     * - full → completed (orden terminada)
     * - phased → in_progress (primera fase pagada, resto locked/pending)
     * - half_half → awaiting_assignment (primer pago hecho)
     * - phased → pending_payment (orden recién creada, esperando primer pago de fase)
     */

    /* Orden 1: Diseño Web Básico — full, in_progress (ya pagado, empleado asignado) */
    if let Some(plan) = plans.iter().find(|p| p.service_title == "Diseño de Sitios Web" && p.slug == "basico") {
        let order_id = create_order(
            &pool, client_id, plan.service_id, plan.id,
            "full", plan.price_cents, 20, "in_progress",
            Some(employee_id),
        ).await;
        create_phases_from_template(&pool, order_id, plan.id, "full", "in_progress").await;
        println!("Orden 1 creada: Diseño Web Básico (full, in_progress)");
    } else {
        eprintln!("WARN: No se encontró plan Diseño de Sitios Web / basico");
    }

    /* Orden 2: Desarrollo Apps Avanzado — phased, in_progress (fase 1 pagada y en ejecución) */
    if let Some(plan) = plans.iter().find(|p| p.service_title == "Desarrollo de Aplicaciones" && p.slug == "avanzado") {
        let order_id = create_order(
            &pool, client_id, plan.service_id, plan.id,
            "phased", plan.price_cents, 0, "in_progress",
            Some(employee_id),
        ).await;
        create_phases_from_template(&pool, order_id, plan.id, "phased", "in_progress").await;
        println!("Orden 2 creada: Desarrollo Apps Avanzado (phased, in_progress)");
    } else {
        eprintln!("WARN: No se encontró plan Desarrollo de Aplicaciones / avanzado");
    }

    /* Orden 3: Agentes IA Básico — full, completed */
    if let Some(plan) = plans.iter().find(|p| p.service_title == "Agentes de IA" && p.slug == "basico") {
        let order_id = create_order(
            &pool, client_id, plan.service_id, plan.id,
            "full", plan.price_cents, 20, "completed",
            Some(employee_id),
        ).await;
        create_phases_from_template(&pool, order_id, plan.id, "full", "completed").await;
        println!("Orden 3 creada: Agentes IA Básico (full, completed)");
    } else {
        eprintln!("WARN: No se encontró plan Agentes de IA / basico");
    }

    /* Orden 4: Branding Avanzado — half_half, awaiting_assignment (primer pago hecho) */
    if let Some(plan) = plans.iter().find(|p| p.service_title == "Identidad de Marca" && p.slug == "avanzado") {
        let order_id = create_order(
            &pool, client_id, plan.service_id, plan.id,
            "half_half", plan.price_cents, 10, "awaiting_assignment",
            None,
        ).await;
        create_phases_from_template(&pool, order_id, plan.id, "half_half", "awaiting_assignment").await;
        println!("Orden 4 creada: Branding Avanzado (half_half, awaiting_assignment)");
    } else {
        eprintln!("WARN: No se encontró plan Identidad de Marca / avanzado");
    }

    /* Orden 5: SEO Básico — phased, pending_payment (recién creada, sin pagar) */
    if let Some(plan) = plans.iter().find(|p| p.service_title == "SEO" && p.slug == "basico") {
        let order_id = create_order(
            &pool, client_id, plan.service_id, plan.id,
            "phased", plan.price_cents, 0, "pending_payment",
            None,
        ).await;
        create_phases_from_template(&pool, order_id, plan.id, "phased", "pending_payment").await;
        println!("Orden 5 creada: SEO Básico (phased, pending_payment)");
    } else {
        eprintln!("WARN: No se encontró plan SEO / basico");
    }

    /* Orden 6: E-commerce Avanzado — full, under_review (empleado entregó, cliente revisa) */
    if let Some(plan) = plans.iter().find(|p| p.service_title == "E-commerce" && p.slug == "avanzado") {
        let order_id = create_order(
            &pool, client_id, plan.service_id, plan.id,
            "full", plan.price_cents, 20, "under_review",
            Some(employee_id),
        ).await;
        create_phases_from_template(&pool, order_id, plan.id, "full", "under_review").await;
        println!("Orden 6 creada: E-commerce Avanzado (full, under_review)");
    } else {
        eprintln!("WARN: No se encontró plan E-commerce / avanzado");
    }

    println!("\nSeed completado. Credenciales:");
    println!("  Cliente:  cliente@test.com / cliente");
    println!("  Empleado: empleado@test.com / empleado");
    println!("  Admin:    admin@admin.com / admin");
}

#[derive(sqlx::FromRow)]
struct PlanRow {
    id: Uuid,
    slug: String,
    #[allow(dead_code)]
    name: String,
    price_cents: i32,
    service_id: Uuid,
    service_title: String,
}

#[derive(sqlx::FromRow)]
struct PhaseTemplate {
    phase_number: i32,
    title: String,
    description: Option<String>,
    percentage_of_total: i32,
    estimated_days: i32,
    max_revisions: i32,
}

async fn upsert_user(pool: &PgPool, email: &str, password: &str, role: &str) -> Uuid {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Error al hashear contraseña")
        .to_string();

    let id = Uuid::new_v4();

    /* [064A-42] display_name derivado del email para que se muestre en la UI. */
    let display_name = email.split('@').next().unwrap_or(email);
    let display_name = display_name[..1].to_uppercase() + &display_name[1..];

    sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO users (id, email, password_hash, role, display_name)
         VALUES ($1, $2, $3, $4::user_role, $5)
         ON CONFLICT (email) DO UPDATE SET password_hash = $3, role = $4::user_role, display_name = $5
         RETURNING id"
    )
    .bind(id)
    .bind(email)
    .bind(&password_hash)
    .bind(role)
    .bind(&display_name)
    .fetch_one(pool)
    .await
    .expect("Error al crear usuario")
}

#[allow(clippy::too_many_arguments)]
async fn create_order(
    pool: &PgPool,
    client_id: Uuid,
    service_id: Uuid,
    plan_id: Uuid,
    payment_mode: &str,
    base_price_cents: i32,
    discount_percent: i32,
    status: &str,
    assigned_employee_id: Option<Uuid>,
) -> Uuid {
    let final_price = base_price_cents - (base_price_cents * discount_percent / 100);

    let started_at = if ["in_progress", "under_review", "completed"].contains(&status) {
        Some(chrono::Utc::now() - chrono::Duration::days(14))
    } else {
        None
    };

    let completed_at = if status == "completed" {
        Some(chrono::Utc::now() - chrono::Duration::days(2))
    } else {
        None
    };

    let assigned_at = assigned_employee_id.map(|_| chrono::Utc::now() - chrono::Duration::days(12));

    sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO orders (
            client_id, service_id, plan_id,
            payment_mode, base_price_cents, discount_percent, final_price_cents,
            status, assigned_employee_id, assigned_at, started_at, completed_at
         ) VALUES (
            $1, $2, $3,
            $4::payment_mode, $5, $6, $7,
            $8::order_status, $9, $10, $11, $12
         ) RETURNING id"
    )
    .bind(client_id)
    .bind(service_id)
    .bind(plan_id)
    .bind(payment_mode)
    .bind(base_price_cents)
    .bind(discount_percent)
    .bind(final_price)
    .bind(status)
    .bind(assigned_employee_id)
    .bind(assigned_at)
    .bind(started_at)
    .bind(completed_at)
    .fetch_one(pool)
    .await
    .expect("Error al crear orden")
}

/* Crea fases basadas en el template del plan, ajustando estados según el modo de pago
 * y el estado de la orden. Respeta la regla de 064A-21: full → fases pagadas siempre. */
async fn create_phases_from_template(
    pool: &PgPool,
    order_id: Uuid,
    plan_id: Uuid,
    payment_mode: &str,
    order_status: &str,
) {
    let templates = sqlx::query_as::<_, PhaseTemplate>(
        "SELECT phase_number, title, description, percentage_of_total, estimated_days, max_revisions
         FROM service_plan_phases WHERE plan_id = $1 ORDER BY phase_number"
    )
    .bind(plan_id)
    .fetch_all(pool)
    .await
    .expect("Error al obtener fases del template");

    let order_price: i32 = sqlx::query_scalar("SELECT final_price_cents FROM orders WHERE id = $1")
        .bind(order_id)
        .fetch_one(pool)
        .await
        .expect("Error al obtener precio");

    for (idx, t) in templates.iter().enumerate() {
        let phase_price = order_price * t.percentage_of_total / 100;

        let phase_status = determine_phase_status(payment_mode, order_status, idx, templates.len());

        sqlx::query(
            "INSERT INTO order_phases (
                order_id, phase_number, title, description, price_cents,
                status, max_revisions, estimated_days
             ) VALUES ($1, $2, $3, $4, $5, $6::phase_status, $7, $8)"
        )
        .bind(order_id)
        .bind(t.phase_number)
        .bind(&t.title)
        .bind(&t.description)
        .bind(phase_price)
        .bind(phase_status)
        .bind(t.max_revisions)
        .bind(t.estimated_days)
        .execute(pool)
        .await
        .expect("Error al crear fase");
    }
}

/* [064A-21] Determina el estado de cada fase según el modo de pago y estado de la orden.
 * Regla principal: payment_mode=full → TODAS las fases están pagadas (nunca pending_payment). */
#[allow(clippy::too_many_lines)]
fn determine_phase_status(payment_mode: &str, order_status: &str, phase_idx: usize, total_phases: usize) -> &'static str {
    match (payment_mode, order_status) {
        /* Todos los modos: completed → approved */
        (_, "completed") => "approved",

        /* Full: todo pagado de entrada */
        ("full", "under_review") => {
            if phase_idx < total_phases - 1 { "approved" } else { "delivered" }
        }
        ("full" | "half_half", "in_progress") => {
            if phase_idx == 0 { "in_progress" } else { "paid" }
        }
        ("full", _) => "paid",

        /* Half-half: primeras fases pagadas, últimas pending */
        ("half_half", "awaiting_assignment") => {
            let half = total_phases / 2;
            if phase_idx < half.max(1) { "paid" } else { "pending_payment" }
        }

        /* Phased: solo la fase actual está activa, el resto locked o pending */
        ("phased", "in_progress") => {
            if phase_idx == 0 { "in_progress" } else if phase_idx == 1 { "pending_payment" } else { "locked" }
        }
        ("phased", "pending_payment") => {
            if phase_idx == 0 { "pending_payment" } else { "locked" }
        }

        _ => "locked",
    }
}

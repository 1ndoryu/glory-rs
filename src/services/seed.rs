/* sentinel-disable-file sqlx-query-sin-macro sqlx-query-as-sin-macro: seed usa runtime queries
 * porque ejecuta SQL dinámico con datos de prueba y tipos genéricos. */
/* [064A-62] Servicio de seed: recrear/borrar datos de prueba desde el panel admin.
 * Lógica extraída de examples/seed_test_data.rs. Crea usuarios de test (cliente, empleado)
 * y órdenes en estados variados para testear flujos del marketplace.
 * [064A-51] Extendido con suscripciones de hosting en diferentes estados.
 * [074A-2] Enriquecido con notificaciones, reviews, chat, activity log.
 * Solo accesible por admin. */

use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use sqlx::PgPool;
use uuid::Uuid;

/* [114A-1] Incluye usuarios legacy de test anteriores para limpiarlos */
const TEST_EMAILS: [&str; 5] = [
    "cliente@test.com",
    "empleado@test.com",
    "test@test.com",
    "testfase2@test.com",
    "employee1@test.com",
];

/* plan, status, client_name, domain, price_cents, storage_mb */
type HostingSeedEntry<'a> = (&'a str, &'a str, &'a str, Option<&'a str>, i32, i32);

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

        /* Borrar en orden por FKs: refunds → payments → deliverables → reviews → fases → órdenes → chat → notificaciones → hosting → activity → usuarios */
        sqlx::query(
            "DELETE FROM order_refunds WHERE order_id IN (SELECT id FROM orders WHERE client_id = ANY($1))",
        )
        .bind(&ids)
        .execute(pool)
        .await?;

        sqlx::query(
            "DELETE FROM order_payments WHERE order_id IN (SELECT id FROM orders WHERE client_id = ANY($1))",
        )
        .bind(&ids)
        .execute(pool)
        .await?;

        sqlx::query(
            "DELETE FROM phase_deliverables WHERE phase_id IN (SELECT id FROM order_phases WHERE order_id IN (SELECT id FROM orders WHERE client_id = ANY($1)))",
        )
        .bind(&ids)
        .execute(pool)
        .await?;

        sqlx::query(
            "DELETE FROM order_reviews WHERE order_id IN (SELECT id FROM orders WHERE client_id = ANY($1))",
        )
        .bind(&ids)
        .execute(pool)
        .await?;

        sqlx::query(
            "DELETE FROM order_delegations WHERE order_id IN (SELECT id FROM orders WHERE client_id = ANY($1))",
        )
        .bind(&ids)
        .execute(pool)
        .await?;

        sqlx::query(
            "DELETE FROM order_phases WHERE order_id IN (SELECT id FROM orders WHERE client_id = ANY($1))",
        )
        .bind(&ids)
        .execute(pool)
        .await?;

        /* Limpiar chat_session_id de orders antes de borrar chat_sessions (FK) */
        sqlx::query("UPDATE orders SET chat_session_id = NULL WHERE client_id = ANY($1)")
            .bind(&ids)
            .execute(pool)
            .await?;

        /* Chat depende de orders (order_id FK) y users (user_id FK) — borrar antes de orders */
        sqlx::query("DELETE FROM chat_messages WHERE session_id IN (SELECT id FROM chat_sessions WHERE user_id = ANY($1) OR order_id IN (SELECT id FROM orders WHERE client_id = ANY($1)))")
            .bind(&ids)
            .execute(pool)
            .await?;

        sqlx::query("DELETE FROM chat_sessions WHERE user_id = ANY($1) OR order_id IN (SELECT id FROM orders WHERE client_id = ANY($1))")
            .bind(&ids)
            .execute(pool)
            .await?;

        sqlx::query("DELETE FROM orders WHERE client_id = ANY($1)")
            .bind(&ids)
            .execute(pool)
            .await?;

        /* Hosting: borrar eventos primero, luego suscripciones */
        sqlx::query(
            "DELETE FROM hosting_events WHERE subscription_id IN (SELECT id FROM hosting_subscriptions WHERE user_id = ANY($1))",
        )
        .bind(&ids)
        .execute(pool)
        .await?;

        sqlx::query("DELETE FROM hosting_subscriptions WHERE user_id = ANY($1)")
            .bind(&ids)
            .execute(pool)
            .await?;

        sqlx::query("DELETE FROM notifications WHERE user_id = ANY($1)")
            .bind(&ids)
            .execute(pool)
            .await?;

        sqlx::query("DELETE FROM activity_log WHERE user_id = ANY($1)")
            .bind(&ids)
            .execute(pool)
            .await?;

        let result = sqlx::query("DELETE FROM users WHERE id = ANY($1)")
            .bind(&ids)
            .execute(pool)
            .await?;

        /* [074A-12] Limpiar proyectos de seed (slug conocidos) */
        sqlx::query(
            "DELETE FROM projects WHERE slug = ANY($1)",
        )
        .bind(["kamples", "mabuhay", "guillermochatbot", "task", "material-de-padel"])
        .execute(pool)
        .await?;

        /* [074A-13] Limpiar miembros del equipo de seed */
        sqlx::query(
            "DELETE FROM team_members WHERE slug = ANY($1)",
        )
        .bind(["wan", "anthony", "misael"])
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
        /* Orden 1: full + in_progress — proyecto activo entre cliente y empleado */
        if let Some(p) = plans.iter().find(|p| p.slug == "basico" && p.service_title.contains("Sitios Web")) {
            Self::create_seed_order(pool, client_id, p, "full", 20, "in_progress", Some(employee_id)).await?;
            created += 1;
        }
        /* Orden 2: phased + pending_payment — esperando pago del cliente */
        if let Some(p) = plans.iter().find(|p| p.slug == "avanzado" && p.service_title.contains("Aplicaciones")) {
            Self::create_seed_order(pool, client_id, p, "phased", 0, "pending_payment", None).await?;
            created += 1;
        }
        /* Orden 3: full + completed — proyecto ya entregado */
        if let Some(p) = plans.iter().find(|p| p.slug == "basico" && p.service_title.contains("IA")) {
            Self::create_seed_order(pool, client_id, p, "full", 20, "completed", Some(employee_id)).await?;
            created += 1;
        }
        /* [084A-9] Orden 4: full + under_review — admin revisa trabajo del empleado */
        if let Some(p) = plans.iter().find(|p| p.slug == "avanzado" && p.service_title.contains("Marca")) {
            Self::create_seed_order(pool, client_id, p, "full", 10, "under_review", Some(employee_id)).await?;
            created += 1;
        }

        /* Suscripciones de hosting de prueba */
        let hosting_count = Self::create_seed_hosting(pool, client_id).await?;

        /* [074A-2] Datos enriquecidos: notificaciones, reviews, chat, activity log */
        let notif_count = Self::create_seed_notifications(pool, client_id, employee_id).await?;
        let review_count = Self::create_seed_reviews(pool, client_id, employee_id).await?;
        let chat_count = Self::create_seed_chat(pool, client_id, employee_id).await?;
        let activity_count = Self::create_seed_activity(pool, client_id, employee_id).await?;

        /* [074A-12] Proyectos de showcase */
        let projects_count = Self::create_seed_projects(pool).await?;

        /* [074A-13] Miembros del equipo */
        let team_count = Self::create_seed_team_members(pool).await?;

        Ok(format!(
            "Seed completado: 2 usuarios + {created} órdenes + {hosting_count} suscripciones hosting + {notif_count} notificaciones + {review_count} reviews + {chat_count} mensajes chat + {activity_count} activity log + {projects_count} proyectos + {team_count} miembros equipo. Credenciales: cliente@test.com/cliente, empleado@test.com/empleado"
        ))
    }

    /* [064A-51] Crea suscripciones de hosting variadas para el usuario test.
     * [084A-10] Precios actualizados: Básico $5, Pro $10, E-commerce $15. */
    async fn create_seed_hosting(pool: &PgPool, client_id: Uuid) -> Result<u32, sqlx::Error> {
        let subs: &[HostingSeedEntry<'_>] = &[
            ("basico", "active", "Cliente Test", Some("mitienda-test.com"), 500, 5120),
            ("pro", "provisioning", "Cliente Test", Some("app-demo.nakomi.dev"), 1000, 20480),
            ("ecommerce", "suspended", "Cliente Test", None, 1500, 51200),
        ];

        let mut count = 0u32;
        for (plan, status, name, domain, price, storage) in subs {
            let sub_id: Uuid = sqlx::query_scalar(
                "INSERT INTO hosting_subscriptions
                 (user_id, client_name, client_email, plan, domain, status, monthly_price_cents, storage_limit_mb)
                 VALUES ($1, $2, 'cliente@test.com', $3, $4, $5, $6, $7)
                 RETURNING id",
            )
            .bind(client_id).bind(name).bind(plan).bind(domain).bind(status)
            .bind(price).bind(storage)
            .fetch_one(pool)
            .await?;

            /* Evento de creación */
            sqlx::query(
                "INSERT INTO hosting_events (subscription_id, event_type, details)
                 VALUES ($1, 'created', $2::jsonb)",
            )
            .bind(sub_id)
            .bind(serde_json::json!({"source": "seed", "plan": plan}))
            .execute(pool)
            .await?;

            /* Evento adicional según status */
            match *status {
                "active" => {
                    sqlx::query(
                        "INSERT INTO hosting_events (subscription_id, event_type, details)
                         VALUES ($1, 'status_changed', $2::jsonb)",
                    )
                    .bind(sub_id)
                    .bind(serde_json::json!({"from": "pending", "to": "active", "source": "seed"}))
                    .execute(pool)
                    .await?;
                }
                "provisioning" => {
                    sqlx::query(
                        "INSERT INTO hosting_events (subscription_id, event_type, details)
                         VALUES ($1, 'status_changed', $2::jsonb)",
                    )
                    .bind(sub_id)
                    .bind(serde_json::json!({"from": "pending", "to": "provisioning", "source": "seed"}))
                    .execute(pool)
                    .await?;
                }
                "suspended" => {
                    sqlx::query(
                        "INSERT INTO hosting_events (subscription_id, event_type, details)
                         VALUES ($1, 'status_changed', $2::jsonb)",
                    )
                    .bind(sub_id)
                    .bind(serde_json::json!({"from": "active", "to": "suspended", "reason": "Impago simulado (seed)", "source": "seed"}))
                    .execute(pool)
                    .await?;
                }
                _ => {}
            }

            count += 1;
        }

        Ok(count)
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

    /* [074A-2] Notificaciones de prueba para que la campanita muestre algo */
    async fn create_seed_notifications(pool: &PgPool, client_id: Uuid, employee_id: Uuid) -> Result<u32, sqlx::Error> {
        let notifs: &[(&str, Uuid, &str, &str, Option<&str>)] = &[
            ("order_assigned", employee_id, "Nueva orden asignada", "Se te asignó la orden #1 de Diseño Web Básico.", Some("/panel?tab=ordenes")),
            ("payment_received", client_id, "Pago confirmado", "Tu pago de $100.00 fue procesado exitosamente.", Some("/panel?tab=ordenes")),
            ("phase_delivered", client_id, "Entrega lista para revisión", "El freelancer entregó la Fase 1 de tu orden.", Some("/panel?tab=ordenes")),
            ("message_received", client_id, "Nuevo mensaje", "Empleado te envió un mensaje en el chat de la orden.", Some("/panel?tab=mensajes")),
            ("order_completed", client_id, "Orden completada", "Tu orden de Agentes IA ha sido completada. ¡Déjanos una reseña!", Some("/panel?tab=ordenes")),
            ("order_completed", employee_id, "Orden completada", "La orden #3 de Agentes IA fue marcada como completada.", Some("/panel?tab=ordenes")),
        ];

        for (ntype, user_id, title, body, link) in notifs {
            sqlx::query(
                "INSERT INTO notifications (user_id, notification_type, title, body, link, read)
                 VALUES ($1, $2, $3, $4, $5, $6)",
            )
            .bind(user_id).bind(ntype).bind(title).bind(body).bind(link)
            .bind(ntype == &"order_completed") /* completada = ya leída */
            .execute(pool)
            .await?;
        }

        #[allow(clippy::cast_possible_truncation)]
        Ok(notifs.len() as u32)
    }

    /* [074A-2] Review en la orden completada */
    async fn create_seed_reviews(pool: &PgPool, client_id: Uuid, employee_id: Uuid) -> Result<u32, sqlx::Error> {
        let completed_order: Option<Uuid> = sqlx::query_scalar(
            "SELECT id FROM orders WHERE client_id = $1 AND status = 'completed' LIMIT 1",
        )
        .bind(client_id)
        .fetch_optional(pool)
        .await?;

        let Some(order_id) = completed_order else { return Ok(0) };

        sqlx::query(
            "INSERT INTO order_reviews (order_id, client_id, employee_id, rating, comment, employee_response, employee_responded_at)
             VALUES ($1, $2, $3, 5, 'Excelente trabajo, muy profesional. Entregas puntuales y comunicación clara.', 'Gracias por confiar en nosotros, fue un placer trabajar contigo.', NOW() - INTERVAL '1 day')
             ON CONFLICT (order_id) DO NOTHING",
        )
        .bind(order_id).bind(client_id).bind(employee_id)
        .execute(pool)
        .await?;

        Ok(1)
    }

    /* [074A-2] Chat con mensajes entre cliente y empleado en la orden in_progress */
    async fn create_seed_chat(pool: &PgPool, client_id: Uuid, employee_id: Uuid) -> Result<u32, sqlx::Error> {
        let in_progress_order: Option<Uuid> = sqlx::query_scalar(
            "SELECT id FROM orders WHERE client_id = $1 AND status = 'in_progress' LIMIT 1",
        )
        .bind(client_id)
        .fetch_optional(pool)
        .await?;

        let Some(order_id) = in_progress_order else { return Ok(0) };

        let session_id: Uuid = sqlx::query_scalar(
            "INSERT INTO chat_sessions (user_id, order_id, status, assigned_staff_id, ai_enabled)
             VALUES ($1, $2, 'active', $3, false)
             RETURNING id",
        )
        .bind(client_id).bind(order_id).bind(employee_id)
        .fetch_one(pool)
        .await?;

        /* Vincular sesión a la orden */
        sqlx::query("UPDATE orders SET chat_session_id = $1 WHERE id = $2")
            .bind(session_id).bind(order_id)
            .execute(pool)
            .await?;

        let messages: &[(&str, &str, &str)] = &[
            ("client", &client_id.to_string(), "Hola, quería consultar sobre el avance de mi sitio web."),
            ("employee", &employee_id.to_string(), "¡Hola! Estoy trabajando en el diseño. Te comparto un boceto en las próximas horas."),
            ("client", &client_id.to_string(), "Perfecto, ¿podrías incluir tonos verdes como en el logo?"),
            ("employee", &employee_id.to_string(), "Sí, ya lo tengo en cuenta. La paleta principal será verde oscuro + blanco."),
            ("client", &client_id.to_string(), "Genial, quedo atento. ¡Gracias!"),
        ];

        let mut count = 0u32;
        for (idx, (sender_type, sender_id, content)) in messages.iter().enumerate() {
            sqlx::query(
                "INSERT INTO chat_messages (session_id, sender_type, sender_id, content, created_at)
                 VALUES ($1, $2, $3, $4, NOW() - ($5 || ' hours')::INTERVAL)",
            )
            .bind(session_id).bind(sender_type).bind(sender_id).bind(content)
            .bind((messages.len() - idx).to_string())
            .execute(pool)
            .await?;
            count += 1;
        }

        Ok(count)
    }

    /* [074A-2] Activity log para dashboard admin */
    async fn create_seed_activity(pool: &PgPool, client_id: Uuid, employee_id: Uuid) -> Result<u32, sqlx::Error> {
        let orders: Vec<Uuid> = sqlx::query_scalar(
            "SELECT id FROM orders WHERE client_id = $1 ORDER BY created_at LIMIT 4",
        )
        .bind(client_id)
        .fetch_all(pool)
        .await?;

        if orders.is_empty() { return Ok(0) }

        let mut count = 0u32;
        for (idx, order_id) in orders.iter().enumerate() {
            sqlx::query(
                "INSERT INTO activity_log (user_id, action, entity_type, entity_id, details, created_at)
                 VALUES ($1, 'order_created', 'order', $2, $3::jsonb, NOW() - ($4 || ' days')::INTERVAL)",
            )
            .bind(client_id).bind(order_id)
            .bind(serde_json::json!({"source": "seed"}))
            .bind((14 - i32::try_from(idx).unwrap_or(0)).to_string())
            .execute(pool)
            .await?;
            count += 1;

            if idx == 0 {
                sqlx::query(
                    "INSERT INTO activity_log (user_id, action, entity_type, entity_id, details, created_at)
                     VALUES ($1, 'order_assigned', 'order', $2, $3::jsonb, NOW() - '12 days'::INTERVAL)",
                )
                .bind(employee_id).bind(order_id)
                .bind(serde_json::json!({"source": "seed", "employee": "empleado@test.com"}))
                .execute(pool)
                .await?;
                count += 1;
            }
        }

        Ok(count)
    }

    /* [074A-12] Proyectos de showcase para el CMS y la página pública */
    async fn create_seed_projects(pool: &PgPool) -> Result<u32, sqlx::Error> {
        #[allow(clippy::needless_pass_by_value)]
        fn je(e: serde_json::Error) -> sqlx::Error { sqlx::Error::Protocol(e.to_string()) }
        type ProjectSeed<'a> = (&'a str, &'a str, &'a str, &'a str, &'a str, &'a str, &'a [&'a str], &'a [&'a str], &'a str);
        let projects: &[ProjectSeed<'_>] = &[
            (
                "KAMPLES", "kamples", "Open Source Platform",
                "Plataforma de samples musicales con algoritmo de recomendación, DAW integrado y funcionalidades de red social.",
                "/assets/Proyectos portadas/Kamples portada.jpg",
                "published",
                &["web", "software"],
                &["React", "Node.js", "PostgreSQL", "Web Audio API", "Redis"],
                "Full-Stack Development|Arquitectura completa: API, frontend SPA y procesamiento de audio.;Algoritmo de Recomendación|Motor de descubrimiento basado en samples, géneros y uso.;DAW Integrado|Workstation de audio embebida para previsualizar y mezclar samples."
            ),
            (
                "MABUHAY", "mabuhay", "Agencia de Viajes",
                "Web y branding para agencia de viajes en España especializada en destinos asiáticos.",
                "/assets/Proyectos portadas/Mabuhay.jpg",
                "published",
                &["web", "branding"],
                &["WordPress", "PHP", "Figma", "Illustrator"],
                "Web Design|Sitio web responsive con catálogo de destinos y reservas.;Branding|Identidad visual inspirada en la hospitalidad filipina."
            ),
            (
                "Guillermo Chatbot", "guillermochatbot", "Proyecto Interno",
                "Chatbot IA conversacional con personalidad, contexto persistente y streaming de respuestas.",
                "/assets/Proyectos portadas/GuillermoPortada.jpg",
                "published",
                &["software", "ia"],
                &["Rust", "Axum", "OpenAI", "WebSocket", "React"],
                "IA Conversacional|Motor de chat con memoria contextual y personalidad configurable.;Streaming|Respuestas en tiempo real con Server-Sent Events."
            ),
            (
                "Task Manager", "task", "Herramienta Interna",
                "Gestor de tareas con tablero Kanban, delegación y seguimiento de tiempo.",
                "/assets/Proyectos portadas/TaskPortada.jpg",
                "published",
                &["software", "web"],
                &["React", "TypeScript", "Node.js", "PostgreSQL"],
                "Kanban Board|Tablero drag-and-drop con estados personalizables.;Time Tracking|Seguimiento de horas por tarea con reportes automáticos."
            ),
            (
                "Material de Pádel", "material-de-padel", "E-commerce",
                "Tienda online especializada en equipamiento de pádel con comparador de productos.",
                "/assets/Proyectos portadas/PadelPortada.jpg",
                "draft",
                &["web", "ecommerce"],
                &["WordPress", "WooCommerce", "PHP", "JavaScript"],
                "E-commerce|Catálogo de productos con filtros avanzados y comparador.;SEO|Optimización para búsquedas de equipamiento deportivo."
            ),
        ];

        let mut count = 0u32;
        for (idx, (title, slug, client, desc, image, status, cats, techs, skills_str)) in projects.iter().enumerate() {
            let categories = serde_json::to_value(cats).map_err(je)?;
            let technologies = serde_json::to_value(techs).map_err(je)?;
            let gallery = serde_json::to_value::<Vec<String>>(vec![]).map_err(je)?;
            let links = serde_json::to_value::<Vec<String>>(vec![]).map_err(je)?;

            /* Parsear skills desde string "titulo|desc;titulo|desc" */
            let skills_parsed: Vec<serde_json::Value> = skills_str.split(';')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let parts: Vec<&str> = s.splitn(2, '|').collect();
                    serde_json::json!({
                        "titulo": parts.first().unwrap_or(&""),
                        "descripcion": parts.get(1).unwrap_or(&"")
                    })
                })
                .collect();
            let skills = serde_json::to_value(&skills_parsed).map_err(je)?;

            sqlx::query(
                "INSERT INTO projects (title, slug, client, description, featured_image, gallery, categories, technologies, links, skills, status, sort_order)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                 ON CONFLICT (slug) DO UPDATE SET title = $1, client = $3, description = $4, featured_image = $5,
                 gallery = $6, categories = $7, technologies = $8, links = $9, skills = $10, status = $11, sort_order = $12"
            )
            .bind(title).bind(slug).bind(client).bind(desc).bind(image)
            .bind(&gallery).bind(&categories).bind(&technologies).bind(&links).bind(&skills)
            .bind(status).bind(i32::try_from(idx).unwrap_or(0))
            .execute(pool)
            .await?;

            count += 1;
        }

        Ok(count)
    }

    /* [074A-13] Miembros del equipo para el CMS y la página pública */
    async fn create_seed_team_members(pool: &PgPool) -> Result<u32, sqlx::Error> {
        let members: &[(&str, &str, &str, &str, &str, &str)] = &[
            (
                "Wan", "wan", "CEO & Founder",
                "Fundadora y directora creativa con visión estratégica para soluciones digitales de alto impacto.",
                "/assets/equipo/wan.jpg",
                "published"
            ),
            (
                "Anthony", "anthony", "Lead Developer",
                "Ingeniero de software principal, especializado en arquitecturas escalables y rendimiento.",
                "/assets/equipo/anthony.jpg",
                "published"
            ),
            (
                "Misael", "misael", "DevOps Engineer",
                "Ingeniero DevOps enfocado en la automatización, despliegue continuo y estabilidad de infraestructura.",
                "/assets/equipo/misael.jpg",
                "published"
            ),
        ];

        let mut count = 0u32;
        for (idx, (name, slug, role, bio, avatar, status)) in members.iter().enumerate() {
            sqlx::query(
                "INSERT INTO team_members (name, slug, role, bio, avatar, status, sort_order)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)
                 ON CONFLICT (slug) DO UPDATE SET name = $1, role = $3, bio = $4, avatar = $5, status = $6, sort_order = $7"
            )
            .bind(name).bind(slug).bind(role).bind(bio).bind(avatar)
            .bind(status).bind(i32::try_from(idx).unwrap_or(0))
            .execute(pool)
            .await?;

            count += 1;
        }

        Ok(count)
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

/* [054A-10] Seed local idempotente para chats de prueba.
 * Evita depender de psql en Windows y deja un hilo visible en el panel admin. */

use glory_backend::config::AppConfig;
use glory_backend::repositories::ChatRepository;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use uuid::Uuid;

const TEST_VISITOR_ID: &str = "test-visitor-001";
const TEST_VISITOR_NAME: &str = "Maria Garcia";

struct SeedMessage<'a> {
    sender_type: &'a str,
    sender_id: &'a str,
    content: &'a str,
}

const SEED_MESSAGES: &[SeedMessage<'_>] = &[
    SeedMessage {
        sender_type: "visitor",
        sender_id: TEST_VISITOR_ID,
        content: "Hola. Me interesa el servicio de diseno web. Quiero entender alcance, tiempos y mantenimiento.",
    },
    SeedMessage {
        sender_type: "ai",
        sender_id: "ai",
        content: "Hola, Maria. Tenemos planes basico y avanzado. Ambos incluyen responsive, SEO tecnico y acompanamiento durante la entrega.",
    },
    SeedMessage {
        sender_type: "visitor",
        sender_id: TEST_VISITOR_ID,
        content: "Me interesa el plan avanzado. Necesito integraciones y una zona privada para clientes.",
    },
    SeedMessage {
        sender_type: "ai",
        sender_id: "ai",
        content: "Perfecto. El plan avanzado suele tardar entre 4 y 6 semanas. Si quieres, un miembro del equipo puede tomar la conversacion.",
    },
    SeedMessage {
        sender_type: "visitor",
        sender_id: TEST_VISITOR_ID,
        content: "Si, por favor. Quiero revisar referencias y un rango de presupuesto esta misma semana.",
    },
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env()?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let session_id = ensure_test_chat(&pool).await?;
    println!("seed: chat de prueba listo en session_id={session_id}");

    Ok(())
}

async fn ensure_test_chat(pool: &PgPool) -> Result<Uuid, sqlx::Error> {
    let session = if let Some(existing) =
        ChatRepository::find_session_by_visitor(pool, TEST_VISITOR_ID).await?
    {
        existing
    } else {
        ChatRepository::create_session(
            pool,
            Some(TEST_VISITOR_ID),
            Some(TEST_VISITOR_NAME),
            None,
            None,
        )
        .await?
    };

    let session_id = session.id;
    let existing_messages = ChatRepository::list_messages(pool, session_id, 1, 0).await?;

    if existing_messages.is_empty() {
        for message in SEED_MESSAGES {
            ChatRepository::save_message(
                pool,
                session_id,
                message.sender_type,
                Some(message.sender_id),
                message.content,
            )
            .await?;
        }
    }

    Ok(session_id)
}

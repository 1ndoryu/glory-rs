/* [044A-38 Fase 5] Repositorio de chat: CRUD sesiones y mensajes.
 * Queries con prepared statements. Soporta sesiones anónimas y autenticadas. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{ChatMessage, ChatSession};

pub struct ChatRepository;

impl ChatRepository {
    /* ============================================================
       SESIONES
       ============================================================ */

    /// Crear sesión de chat (pre-venta o vinculada a orden)
    pub async fn create_session(
        pool: &PgPool,
        visitor_id: Option<&str>,
        visitor_name: Option<&str>,
        user_id: Option<Uuid>,
        order_id: Option<Uuid>,
    ) -> Result<ChatSession, sqlx::Error> {
        sqlx::query_as::<_, ChatSession>(
            "INSERT INTO chat_sessions (visitor_id, visitor_name, user_id, order_id) \
             VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(visitor_id)
        .bind(visitor_name)
        .bind(user_id)
        .bind(order_id)
        .fetch_one(pool)
        .await
    }

    pub async fn find_session_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<ChatSession>, sqlx::Error> {
        sqlx::query_as::<_, ChatSession>(
            "SELECT * FROM chat_sessions WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    /// Sesiones activas para un usuario autenticado
    pub async fn list_sessions_for_user(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<ChatSession>, sqlx::Error> {
        sqlx::query_as::<_, ChatSession>(
            "SELECT * FROM chat_sessions \
             WHERE (user_id = $1 OR assigned_staff_id = $1) \
             AND status != 'closed' \
             ORDER BY updated_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// Sesión activa por orden (para evitar duplicados)
    pub async fn find_session_by_order(
        pool: &PgPool,
        order_id: Uuid,
    ) -> Result<Option<ChatSession>, sqlx::Error> {
        sqlx::query_as::<_, ChatSession>(
            "SELECT * FROM chat_sessions \
             WHERE order_id = $1 AND status != 'closed' \
             ORDER BY created_at DESC LIMIT 1",
        )
        .bind(order_id)
        .fetch_optional(pool)
        .await
    }

    /// Sesión activa por `visitor_id` (anónimo)
    pub async fn find_session_by_visitor(
        pool: &PgPool,
        visitor_id: &str,
    ) -> Result<Option<ChatSession>, sqlx::Error> {
        sqlx::query_as::<_, ChatSession>(
            "SELECT * FROM chat_sessions \
             WHERE visitor_id = $1 AND status != 'closed' \
             ORDER BY created_at DESC LIMIT 1",
        )
        .bind(visitor_id)
        .fetch_optional(pool)
        .await
    }

    /// Todas las sesiones activas (panel staff)
    pub async fn list_active_sessions(
        pool: &PgPool,
    ) -> Result<Vec<ChatSession>, sqlx::Error> {
        sqlx::query_as::<_, ChatSession>(
            "SELECT * FROM chat_sessions \
             WHERE status != 'closed' \
             ORDER BY updated_at DESC",
        )
        .fetch_all(pool)
        .await
    }

    /// Staff toma una sesión
    pub async fn assign_staff(
        pool: &PgPool,
        session_id: Uuid,
        staff_id: Uuid,
    ) -> Result<ChatSession, sqlx::Error> {
        sqlx::query_as::<_, ChatSession>(
            "UPDATE chat_sessions SET assigned_staff_id = $2, \
             status = 'staff_handling', ai_enabled = false, \
             updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(session_id)
        .bind(staff_id)
        .fetch_one(pool)
        .await
    }

    /// Toggle IA en una sesión
    pub async fn toggle_ai(
        pool: &PgPool,
        session_id: Uuid,
        enabled: bool,
    ) -> Result<ChatSession, sqlx::Error> {
        let new_status = if enabled { "ai_handling" } else { "staff_handling" };
        sqlx::query_as::<_, ChatSession>(
            "UPDATE chat_sessions SET ai_enabled = $2, status = $3, \
             updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(session_id)
        .bind(enabled)
        .bind(new_status)
        .fetch_one(pool)
        .await
    }

    /// Cerrar sesión
    pub async fn close_session(
        pool: &PgPool,
        session_id: Uuid,
    ) -> Result<ChatSession, sqlx::Error> {
        sqlx::query_as::<_, ChatSession>(
            "UPDATE chat_sessions SET status = 'closed', \
             updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(session_id)
        .fetch_one(pool)
        .await
    }

    /* ============================================================
       MENSAJES
       ============================================================ */

    /// Guardar mensaje en BD
    pub async fn save_message(
        pool: &PgPool,
        session_id: Uuid,
        sender_type: &str,
        sender_id: Option<&str>,
        content: &str,
    ) -> Result<ChatMessage, sqlx::Error> {
        /* Actualizar timestamp de sesión al recibir mensaje */
        let _ = sqlx::query(
            "UPDATE chat_sessions SET updated_at = NOW() WHERE id = $1",
        )
        .bind(session_id)
        .execute(pool)
        .await;

        sqlx::query_as::<_, ChatMessage>(
            "INSERT INTO chat_messages (session_id, sender_type, sender_id, content) \
             VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(session_id)
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .fetch_one(pool)
        .await
    }

    /// Historial de mensajes de una sesión (paginado)
    pub async fn list_messages(
        pool: &PgPool,
        session_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ChatMessage>, sqlx::Error> {
        sqlx::query_as::<_, ChatMessage>(
            "SELECT * FROM chat_messages WHERE session_id = $1 \
             ORDER BY created_at ASC LIMIT $2 OFFSET $3",
        )
        .bind(session_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    /// Último mensaje de múltiples sesiones (para preview en lista)
    pub async fn last_messages_for_sessions(
        pool: &PgPool,
        session_ids: &[Uuid],
    ) -> Result<Vec<ChatMessage>, sqlx::Error> {
        sqlx::query_as::<_, ChatMessage>(
            "SELECT DISTINCT ON (session_id) * FROM chat_messages \
             WHERE session_id = ANY($1) \
             ORDER BY session_id, created_at DESC",
        )
        .bind(session_ids)
        .fetch_all(pool)
        .await
    }

    /* ============================================================
       TYPING
       ============================================================ */

    /// Actualizar typing preview
    pub async fn update_typing(
        pool: &PgPool,
        session_id: Uuid,
        content: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO chat_typing (session_id, content, updated_at) \
             VALUES ($1, $2, NOW()) \
             ON CONFLICT (session_id) DO UPDATE SET content = $2, updated_at = NOW()",
        )
        .bind(session_id)
        .bind(content)
        .execute(pool)
        .await?;
        Ok(())
    }
}

/* [044A-38 Fase 5] ChatHub: gestión de conexiones WebSocket y routing de mensajes.
 * DashMap para sesiones activas en memoria. Cada sesión tiene múltiples suscriptores
 * (visitante/cliente + staff). Los mensajes se persisten en BD y se broadcastean. */

use std::sync::Arc;

use dashmap::DashMap;
use sqlx::PgPool;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{
    ChatMessage, ChatSession, ChatSessionResponse, WsServerMessage,
};
use crate::repositories::ChatRepository;

/* Canal de broadcast por sesión: cada conexión WS suscribe un receiver */
const CHANNEL_CAPACITY: usize = 64;

pub type SessionSender = broadcast::Sender<WsServerMessage>;

/// Hub central de chat: estado en memoria de sesiones activas + broadcasters
#[derive(Clone)]
pub struct ChatHub {
    pool: PgPool,
    /* session_id → broadcast::Sender para ese chat */
    channels: Arc<DashMap<Uuid, SessionSender>>,
}

impl ChatHub {
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            channels: Arc::new(DashMap::new()),
        }
    }

    /// Obtiene o crea el canal broadcast para una sesión
    #[must_use]
    pub fn get_or_create_channel(&self, session_id: Uuid) -> SessionSender {
        self.channels
            .entry(session_id)
            .or_insert_with(|| broadcast::channel(CHANNEL_CAPACITY).0)
            .clone()
    }

    /// Suscribirse al canal de una sesión (para recibir mensajes)
    #[must_use]
    pub fn subscribe(&self, session_id: Uuid) -> broadcast::Receiver<WsServerMessage> {
        self.get_or_create_channel(session_id).subscribe()
    }

    /// Eliminar canal cuando la sesión se cierra
    pub fn remove_channel(&self, session_id: Uuid) {
        self.channels.remove(&session_id);
    }

    /// Broadcast de un mensaje a todos los suscriptores de una sesión
    pub fn broadcast(&self, session_id: Uuid, msg: WsServerMessage) {
        if let Some(sender) = self.channels.get(&session_id) {
            /* Ignorar error (no hay receivers conectados) */
            let _ = sender.send(msg);
        }
    }

    /* ============================================================
       OPERACIONES DE CHAT
       ============================================================ */

    /// Crear o recuperar sesión para un visitante anónimo
    pub async fn get_or_create_visitor_session(
        &self,
        visitor_id: &str,
        visitor_name: Option<&str>,
    ) -> Result<ChatSession, AppError> {
        if let Some(existing) =
            ChatRepository::find_session_by_visitor(&self.pool, visitor_id).await?
        {
            return Ok(existing);
        }
        let session = ChatRepository::create_session(
            &self.pool,
            Some(visitor_id),
            visitor_name,
            None,
            None,
        )
        .await?;
        Ok(session)
    }

    /// Crear o recuperar sesión vinculada a una orden
    pub async fn get_or_create_order_session(
        &self,
        order_id: Uuid,
        user_id: Uuid,
    ) -> Result<ChatSession, AppError> {
        if let Some(existing) =
            ChatRepository::find_session_by_order(&self.pool, order_id).await?
        {
            return Ok(existing);
        }
        let session = ChatRepository::create_session(
            &self.pool,
            None,
            None,
            Some(user_id),
            Some(order_id),
        )
        .await?;
        Ok(session)
    }

    /// Enviar mensaje y persistir + broadcast
    pub async fn send_message(
        &self,
        session_id: Uuid,
        sender_type: &str,
        sender_id: Option<&str>,
        content: &str,
    ) -> Result<ChatMessage, AppError> {
        let msg = ChatRepository::save_message(
            &self.pool,
            session_id,
            sender_type,
            sender_id,
            content,
        )
        .await?;

        let ws_msg = WsServerMessage::Message {
            id: msg.id,
            session_id: msg.session_id,
            sender: msg.sender_type.clone(),
            sender_id: msg.sender_id.clone(),
            content: msg.content.clone(),
            created_at: msg.created_at,
        };
        self.broadcast(session_id, ws_msg);

        Ok(msg)
    }

    /// Broadcast de typing sin persistir (alta frecuencia)
    pub fn send_typing(&self, session_id: Uuid, sender: &str, content: &str) {
        self.broadcast(
            session_id,
            WsServerMessage::Typing {
                session_id,
                sender: sender.to_string(),
                content: content.to_string(),
            },
        );
    }

    /// Staff toma sesión
    pub async fn staff_join_session(
        &self,
        session_id: Uuid,
        staff_id: Uuid,
    ) -> Result<ChatSession, AppError> {
        let session = ChatRepository::assign_staff(&self.pool, session_id, staff_id).await?;
        self.broadcast(
            session_id,
            WsServerMessage::Status {
                session_id,
                value: "staff_handling".to_string(),
            },
        );
        Ok(session)
    }

    /// Toggle AI en sesión
    pub async fn toggle_ai(
        &self,
        session_id: Uuid,
        enabled: bool,
    ) -> Result<ChatSession, AppError> {
        let session = ChatRepository::toggle_ai(&self.pool, session_id, enabled).await?;
        let status = if enabled { "ai_handling" } else { "staff_handling" };
        self.broadcast(
            session_id,
            WsServerMessage::Status {
                session_id,
                value: status.to_string(),
            },
        );
        Ok(session)
    }

    /// Cerrar sesión
    pub async fn close_session(&self, session_id: Uuid) -> Result<(), AppError> {
        ChatRepository::close_session(&self.pool, session_id).await?;
        self.broadcast(session_id, WsServerMessage::SessionClosed { session_id });
        self.remove_channel(session_id);
        Ok(())
    }

    /// Listar sesiones activas como responses con `last_message` preview
    pub async fn list_sessions_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ChatSessionResponse>, AppError> {
        let sessions = ChatRepository::list_sessions_for_user(&self.pool, user_id).await?;
        self.enrich_sessions(sessions).await
    }

    /// Listar todas las sesiones activas (staff/admin)
    pub async fn list_all_active_sessions(
        &self,
    ) -> Result<Vec<ChatSessionResponse>, AppError> {
        let sessions = ChatRepository::list_active_sessions(&self.pool).await?;
        self.enrich_sessions(sessions).await
    }

    /// Enriquecer sesiones con `last_message` preview
    async fn enrich_sessions(
        &self,
        sessions: Vec<ChatSession>,
    ) -> Result<Vec<ChatSessionResponse>, AppError> {
        let ids: Vec<Uuid> = sessions.iter().map(|s| s.id).collect();
        let last_msgs = ChatRepository::last_messages_for_sessions(&self.pool, &ids).await?;

        Ok(sessions
            .into_iter()
            .map(|s| {
                let last = last_msgs.iter().find(|m| m.session_id == s.id);
                ChatSessionResponse {
                    id: s.id,
                    order_id: s.order_id,
                    status: s.status,
                    ai_enabled: s.ai_enabled,
                    assigned_staff_id: s.assigned_staff_id,
                    last_message: last.map(|m| m.content.clone()),
                    last_message_at: last.map(|m| m.created_at),
                    created_at: s.created_at,
                }
            })
            .collect())
    }

    /// Acceso al pool (para AI service u otros)
    #[must_use]
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

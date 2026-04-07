/* [044A-38 Fase 5] Modelos de chat: sesiones, mensajes y tipos WebSocket.
 * chat_sessions vinculadas opcionalmente a orders (order_id).
 * sender_type: client|ai|employee|admin (roles marketplace). */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/* ============================================================
   MODELOS DE BD
   ============================================================ */

#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct ChatSession {
    pub id: Uuid,
    pub visitor_id: Option<String>,
    pub visitor_name: Option<String>,
    pub user_id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub status: String,
    pub assigned_staff_id: Option<Uuid>,
    pub ai_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /* [064A-72] Metadata del visitante capturada en la conexión WS.
     * default: los queries que no seleccionan estas columnas las reciben como None. */
    #[sqlx(default)]
    pub visitor_ip: Option<String>,
    #[sqlx(default)]
    pub visitor_user_agent: Option<String>,
}

#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct ChatMessage {
    pub id: Uuid,
    pub session_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<String>,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

/* [064A-70] Respuesta enriquecida con datos del sender (avatar + nombre) */
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ChatMessageResponse {
    pub id: Uuid,
    pub session_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<String>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub sender_avatar_url: Option<String>,
    pub sender_display_name: Option<String>,
}

/* ============================================================
   REQUESTS / RESPONSES
   ============================================================ */

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateChatSessionRequest {
    pub visitor_id: Option<String>,
    pub visitor_name: Option<String>,
    pub order_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SendMessageRequest {
    pub content: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChatSessionResponse {
    pub id: Uuid,
    pub order_id: Option<Uuid>,
    /// [064A-31] Número de orden legible (si la sesión está vinculada a una orden)
    pub order_number: Option<i32>,
    pub status: String,
    pub ai_enabled: bool,
    pub assigned_staff_id: Option<Uuid>,
    pub last_message: Option<String>,
    pub last_message_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    /* [064A-72] Info del visitante para panel lateral */
    pub visitor_name: Option<String>,
    pub visitor_ip: Option<String>,
    pub visitor_user_agent: Option<String>,
}

/* [064A-72] Modelo de notas de sesión de chat */
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct ChatSessionNote {
    pub id: Uuid,
    pub session_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSessionNoteRequest {
    pub content: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateVisitorNameRequest {
    pub name: String,
}

/* ============================================================
   MENSAJES WEBSOCKET (protocolo JSON)
   ============================================================ */

/// Mensaje entrante del cliente WebSocket
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum WsClientMessage {
    #[serde(rename = "message")]
    Message { content: String },
    #[serde(rename = "typing")]
    Typing { content: String },
    #[serde(rename = "join")]
    Join { session_id: Uuid },
    #[serde(rename = "close")]
    Close,
    #[serde(rename = "toggle_ai")]
    ToggleAi { session_id: Uuid, enabled: bool },
}

/// Mensaje saliente del servidor WebSocket
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum WsServerMessage {
    #[serde(rename = "message")]
    Message {
        id: Uuid,
        session_id: Uuid,
        sender: String,
        sender_id: Option<String>,
        content: String,
        created_at: DateTime<Utc>,
    },
    #[serde(rename = "typing")]
    Typing {
        session_id: Uuid,
        sender: String,
        content: String,
    },
    #[serde(rename = "status")]
    Status {
        session_id: Uuid,
        value: String,
    },
    #[serde(rename = "session_new")]
    SessionNew {
        session: ChatSession,
    },
    #[serde(rename = "session_closed")]
    SessionClosed {
        session_id: Uuid,
    },
    #[serde(rename = "error")]
    Error {
        message: String,
    },
}

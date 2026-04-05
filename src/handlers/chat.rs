/* [044A-38 Fase 5] Handlers de chat: WebSocket + REST.
 * WS: /ws/chat/visitor (anónimo), /ws/chat/staff (autenticado).
 * REST: CRUD sesiones y mensajes bajo /api/chat/. */

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Path, Query, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::AuthUser;
use crate::models::{
    ChatMessage, ChatSessionResponse, CreateChatSessionRequest,
    SendMessageRequest, WsClientMessage, WsServerMessage,
};
use crate::services::AiChatService;
use crate::AppState;

/* ============================================================
   QUERY PARAMS
   ============================================================ */

#[derive(Deserialize)]
pub struct VisitorWsParams {
    pub visitor_id: String,
    pub visitor_name: Option<String>,
}

#[derive(Deserialize)]
pub struct MessagesQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/* ============================================================
   WEBSOCKET: VISITOR (anónimo o cliente autenticado)
   ============================================================ */

async fn ws_visitor(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(params): Query<VisitorWsParams>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_visitor_ws(socket, state, params))
}

async fn handle_visitor_ws(socket: WebSocket, state: AppState, params: VisitorWsParams) {
    let session = match state
        .chat_hub
        .get_or_create_visitor_session(&params.visitor_id, params.visitor_name.as_deref())
        .await
    {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Error creando sesión visitor: {e}");
            return;
        }
    };

    let session_id = session.id;
    let mut rx = state.chat_hub.subscribe(session_id);
    let (mut sender, mut receiver) = socket.split();

    /* Notificar a staff de nueva sesión */
    state.chat_hub.broadcast(
        session_id,
        WsServerMessage::SessionNew {
            session: session.clone(),
        },
    );

    /* Task: enviar mensajes del broadcast al WS del visitante */
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    /* Recibir mensajes del visitante */
    let hub = state.chat_hub.clone();
    let ai_config = state.ai_config.clone();
    let pool = state.pool.clone();
    let vid = params.visitor_id.clone();

    while let Some(Ok(msg)) = receiver.next().await {
        let Message::Text(text) = msg else {
            continue;
        };
        let Ok(ws_msg) = serde_json::from_str::<WsClientMessage>(&text) else {
            continue;
        };

        match ws_msg {
            WsClientMessage::Message { content } => {
                /* Guardar mensaje del visitante */
                let _ = hub
                    .send_message(session_id, "client", Some(&vid), &content)
                    .await;

                /* Si IA habilitada y no hay staff, generar respuesta */
                if let Ok(Some(s)) =
                    crate::repositories::ChatRepository::find_session_by_id(&pool, session_id)
                        .await
                {
                    if s.ai_enabled && s.assigned_staff_id.is_none() {
                        let ai_response = AiChatService::generate_response(
                            &pool,
                            &ai_config,
                            session_id,
                            &content,
                        )
                        .await
                        .unwrap_or_else(|e| format!("Error IA: {e}"));

                        let _ = hub
                            .send_message(session_id, "ai", Some("ai"), &ai_response)
                            .await;
                    }
                }
            }
            WsClientMessage::Typing { content } => {
                hub.send_typing(session_id, "client", &content);
            }
            WsClientMessage::Close => {
                let _ = hub.close_session(session_id).await;
                break;
            }
            _ => {} /* join/toggle_ai son solo para staff */
        }
    }

    send_task.abort();
}

/* ============================================================
   WEBSOCKET: STAFF (autenticado con JWT)
   ============================================================ */

#[derive(Deserialize)]
pub struct StaffWsParams {
    pub token: String,
}

async fn ws_staff(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(params): Query<StaffWsParams>,
) -> impl IntoResponse {
    /* Verificar JWT desde query param (WS no soporta headers custom) */
    let Ok(claims) = crate::services::AuthService::verify_token(
        &params.token,
        &state.jwt_secret,
    ) else {
        return (StatusCode::UNAUTHORIZED, "Token inválido").into_response();
    };

    ws.on_upgrade(move |socket| handle_staff_ws(socket, state, claims.sub))
        .into_response()
}

async fn handle_staff_ws(socket: WebSocket, state: AppState, staff_id: Uuid) {
    let (mut ws_sender, mut receiver) = socket.split();

    let hub = state.chat_hub.clone();
    let pool = state.pool.clone();

    /* Canal interno: las suscripciones a sesiones envían aquí, un task central escribe al WS */
    let (tx, mut rx) = tokio::sync::mpsc::channel::<WsServerMessage>(128);

    /* Enviar lista de sesiones activas al conectar */
    if let Ok(sessions) = hub.list_all_active_sessions().await {
        let init_msg = serde_json::json!({
            "type": "init",
            "sessions": sessions
        });
        if let Ok(json) = serde_json::to_string(&init_msg) {
            let _ = ws_sender.send(Message::Text(json)).await;
        }
    }

    /* Task: leer del mpsc y enviar al WS */
    let send_task = tokio::spawn(async move {
        while let Some(server_msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&server_msg) {
                if ws_sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    /* Track de suscripciones */
    let mut subscriptions = Vec::new();

    /* Recibir mensajes del staff */
    while let Some(Ok(msg)) = receiver.next().await {
        let Message::Text(text) = msg else {
            continue;
        };
        let Ok(ws_msg) = serde_json::from_str::<WsClientMessage>(&text) else {
            continue;
        };

        match ws_msg {
            WsClientMessage::Join { session_id } => {
                let _ = hub.staff_join_session(session_id, staff_id).await;

                /* Suscribirse al canal de esta sesión → reenviar al mpsc */
                let mut session_rx = hub.subscribe(session_id);
                let tx_clone = tx.clone();
                let handle = tokio::spawn(async move {
                    while let Ok(server_msg) = session_rx.recv().await {
                        if tx_clone.send(server_msg).await.is_err() {
                            break;
                        }
                    }
                });
                subscriptions.push(handle);
            }
            WsClientMessage::Message { content } => {
                tracing::debug!("Staff message (sin session_id en WsClientMessage::Message): {content}");
            }
            WsClientMessage::Typing { content } => {
                tracing::debug!("Staff typing: {content}");
            }
            WsClientMessage::ToggleAi {
                session_id,
                enabled,
            } => {
                let _ = hub.toggle_ai(session_id, enabled).await;
            }
            WsClientMessage::Close => {
                break;
            }
        }
    }

    /* Cleanup */
    for handle in subscriptions {
        handle.abort();
    }
    send_task.abort();

    let _ = pool;
}

/* ============================================================
   REST API ENDPOINTS
   ============================================================ */

/// Listar sesiones de chat del usuario
#[utoipa::path(
    get,
    path = "/api/chat/sessions",
    responses(
        (status = 200, description = "Sesiones activas", body = Vec<ChatSessionResponse>),
        (status = 401, description = "No autorizado"),
    ),
    security(("bearer_auth" = [])),
    tag = "chat"
)]
pub async fn list_sessions(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<ChatSessionResponse>>, AppError> {
    let sessions = if auth.effective_role == crate::models::UserRole::Admin
        || auth.effective_role == crate::models::UserRole::Employee
    {
        state.chat_hub.list_all_active_sessions().await?
    } else {
        state.chat_hub.list_sessions_for_user(auth.user_id).await?
    };
    Ok(Json(sessions))
}

/// Obtener mensajes de una sesión
#[utoipa::path(
    get,
    path = "/api/chat/sessions/{session_id}/messages",
    params(
        ("session_id" = Uuid, Path, description = "ID de la sesión"),
        ("limit" = Option<i64>, Query, description = "Límite de mensajes"),
        ("offset" = Option<i64>, Query, description = "Offset para paginación"),
    ),
    responses(
        (status = 200, description = "Mensajes", body = Vec<ChatMessage>),
        (status = 401, description = "No autorizado"),
    ),
    security(("bearer_auth" = [])),
    tag = "chat"
)]
pub async fn get_messages(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(session_id): Path<Uuid>,
    Query(params): Query<MessagesQuery>,
) -> Result<Json<Vec<ChatMessage>>, AppError> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    let messages =
        crate::repositories::ChatRepository::list_messages(&state.pool, session_id, limit, offset)
            .await?;
    Ok(Json(messages))
}

/// Crear sesión de chat (para órdenes desde frontend)
#[utoipa::path(
    post,
    path = "/api/chat/sessions",
    request_body = CreateChatSessionRequest,
    responses(
        (status = 201, description = "Sesión creada", body = ChatSessionResponse),
        (status = 401, description = "No autorizado"),
    ),
    security(("bearer_auth" = [])),
    tag = "chat"
)]
pub async fn create_session(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateChatSessionRequest>,
) -> Result<(StatusCode, Json<ChatSessionResponse>), AppError> {
    let session = if let Some(order_id) = req.order_id {
        state
            .chat_hub
            .get_or_create_order_session(order_id, auth.user_id)
            .await?
    } else {
        let vid = req
            .visitor_id
            .unwrap_or_else(|| auth.user_id.to_string());
        state
            .chat_hub
            .get_or_create_visitor_session(&vid, req.visitor_name.as_deref())
            .await?
    };

    let response = ChatSessionResponse {
        id: session.id,
        order_id: session.order_id,
        status: session.status,
        ai_enabled: session.ai_enabled,
        assigned_staff_id: session.assigned_staff_id,
        last_message: None,
        last_message_at: None,
        created_at: session.created_at,
    };
    Ok((StatusCode::CREATED, Json(response)))
}

/// Enviar mensaje REST (alternativa a WebSocket)
#[utoipa::path(
    post,
    path = "/api/chat/sessions/{session_id}/messages",
    params(("session_id" = Uuid, Path, description = "ID de la sesión")),
    request_body = SendMessageRequest,
    responses(
        (status = 201, description = "Mensaje enviado", body = ChatMessage),
        (status = 401, description = "No autorizado"),
    ),
    security(("bearer_auth" = [])),
    tag = "chat"
)]
pub async fn send_message(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(session_id): Path<Uuid>,
    Json(req): Json<SendMessageRequest>,
) -> Result<(StatusCode, Json<ChatMessage>), AppError> {
    let sender_type = match auth.effective_role {
        crate::models::UserRole::Admin => "admin",
        crate::models::UserRole::Employee => "employee",
        crate::models::UserRole::Client => "client",
    };

    let msg = state
        .chat_hub
        .send_message(session_id, sender_type, Some(&auth.user_id.to_string()), &req.content)
        .await?;

    /* Si IA habilitada y sender es client, generar respuesta */
    if sender_type == "client" {
        if let Ok(Some(s)) =
            crate::repositories::ChatRepository::find_session_by_id(&state.pool, session_id).await
        {
            if s.ai_enabled && s.assigned_staff_id.is_none() {
                let ai_response = AiChatService::generate_response(
                    &state.pool,
                    &state.ai_config,
                    session_id,
                    &req.content,
                )
                .await
                .unwrap_or_else(|e| format!("Error IA: {e}"));

                let _ = state
                    .chat_hub
                    .send_message(session_id, "ai", Some("ai"), &ai_response)
                    .await;
            }
        }
    }

    Ok((StatusCode::CREATED, Json(msg)))
}

/* ============================================================
   ROUTES
   ============================================================ */

/// WebSocket routes (montadas en root, no bajo /api)
pub fn ws_routes() -> Router<AppState> {
    Router::new()
        .route("/ws/chat/visitor", get(ws_visitor))
        .route("/ws/chat/staff", get(ws_staff))
}

/// REST routes (montadas bajo /api)
pub fn rest_routes() -> Router<AppState> {
    Router::new()
        .route("/chat/sessions", get(list_sessions).post(create_session))
        .route("/chat/sessions/:session_id/messages", get(get_messages).post(send_message))
}

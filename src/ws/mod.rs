use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use glory_rs::websocket::{WebSocketEnvelope, WebSocketHub};
use serde::Serialize;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use crate::errors::AppError;

/* [174A-70/174A-72] Runtime websocket del backend principal sobre el hub reusable.
 * El socket gestiona handshake, registro/unregister y ping/pong; la proyección de
 * eventos de dominio queda en helpers para que cada handler emita sin acoplarse al
 * formato interno del hub. */

pub fn emit_event<T: Serialize>(
    hub: &WebSocketHub,
    user_id: i32,
    name: &str,
    payload: &T,
) -> Result<usize, AppError> {
    let payload = serde_json::to_value(payload)
        .map_err(|error| AppError::Internal(format!("serializar payload websocket: {error}")))?;
    hub.broadcast_user(
        user_id,
        &WebSocketEnvelope::Event {
            name: name.to_owned(),
            payload,
        },
    )
    .map_err(|error| AppError::Internal(format!("emitir evento websocket: {error}")))
}

pub async fn serve_socket(socket: WebSocket, hub: Arc<WebSocketHub>, user_id: i32) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = unbounded_channel::<Message>();
    let connection = match hub.register(user_id, tx.clone()) {
        Ok(connection) => connection,
        Err(error) => {
            let envelope = WebSocketEnvelope::Error {
                code: "too_many_connections".into(),
                message: error.to_string(),
            };
            if let Ok(message) = envelope.to_message() {
                let _ = sender.send(message).await;
            }
            let _ = sender.send(Message::Close(None)).await;
            return;
        }
    };

    let authenticated = WebSocketEnvelope::Authenticated {
        user_id,
        connection_id: connection.connection_id,
    };
    if let Err(error) = authenticated.to_message() {
        tracing::error!(user_id, error = %error, "no se pudo serializar mensaje authenticated websocket");
        hub.unregister(connection);
        return;
    }
    if sender
        .send(authenticated.to_message().expect("validated above"))
        .await
        .is_err()
    {
        hub.unregister(connection);
        return;
    }

    loop {
        tokio::select! {
            outbound = rx.recv() => {
                match outbound {
                    Some(message) => {
                        if sender.send(message).await.is_err() {
                            break;
                        }
                    }
                    None => break,
                }
            }
            inbound = receiver.next() => {
                match inbound {
                    Some(Ok(message)) => {
                        if !handle_incoming_message(&tx, message) {
                            break;
                        }
                    }
                    Some(Err(error)) => {
                        tracing::warn!(user_id, error = %error, "falló la recepción websocket");
                        break;
                    }
                    None => break,
                }
            }
        }
    }

    hub.unregister(connection);
}

fn handle_incoming_message(tx: &UnboundedSender<Message>, message: Message) -> bool {
    match message {
        Message::Text(text) => {
            handle_text_message(tx, &text);
            true
        }
        Message::Binary(_) => {
            send_envelope(
                tx,
                &WebSocketEnvelope::Error {
                    code: "unsupported_message".into(),
                    message: "Sólo se aceptan mensajes JSON tipo websocket envelope".into(),
                },
            );
            true
        }
        Message::Ping(payload) => tx.send(Message::Pong(payload)).is_ok(),
        Message::Pong(_) => true,
        Message::Close(_) => false,
    }
}

fn handle_text_message(tx: &UnboundedSender<Message>, text: &str) {
    match serde_json::from_str::<WebSocketEnvelope>(text) {
        Ok(WebSocketEnvelope::Ping) => {
            send_envelope(tx, &WebSocketEnvelope::Pong);
        }
        Ok(_) => {
            send_envelope(
                tx,
                &WebSocketEnvelope::Error {
                    code: "unsupported_message".into(),
                    message: "Este endpoint todavía no acepta eventos de cliente".into(),
                },
            );
        }
        Err(_) => {
            send_envelope(
                tx,
                &WebSocketEnvelope::Error {
                    code: "invalid_payload".into(),
                    message: "Payload websocket inválido".into(),
                },
            );
        }
    }
}

fn send_envelope(tx: &UnboundedSender<Message>, envelope: &WebSocketEnvelope) {
    match envelope.to_message() {
        Ok(message) => {
            let _ = tx.send(message);
        }
        Err(error) => {
            tracing::error!(error = %error, "no se pudo serializar envelope websocket");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::emit_event;
    use axum::extract::ws::Message;
    use glory_rs::websocket::{HubConfig, WebSocketHub};
    use serde::Serialize;
    use tokio::sync::mpsc::unbounded_channel;

    #[derive(Serialize)]
    struct TestPayload {
        id: i32,
    }

    #[test]
    fn emits_domain_event_to_registered_user() {
        let hub = WebSocketHub::new(HubConfig::default());
        let (tx, mut rx) = unbounded_channel::<Message>();
        let _connection = hub.register(17, tx).expect("register");

        let delivered =
            emit_event(&hub, 17, "mensaje_nuevo", &TestPayload { id: 42 }).expect("emit event");

        assert_eq!(delivered, 1);
        let Message::Text(payload) = rx.try_recv().expect("payload recibido") else {
            panic!("mensaje websocket no textual");
        };
        assert!(payload.contains("\"type\":\"event\""));
        assert!(payload.contains("\"name\":\"mensaje_nuevo\""));
        assert!(payload.contains("\"id\":42"));
    }
}

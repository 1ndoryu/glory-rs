use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use glory_rs::websocket::{WebSocketEnvelope, WebSocketHub};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

/* [174A-70] Runtime websocket del backend principal sobre el hub reusable.
 * Este corte sólo resuelve handshake, registro/unregister y ping/pong básico;
 * la emisión de eventos de dominio queda para 174A-72. */

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
	if sender.send(authenticated.to_message().expect("validated above")).await.is_err() {
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

# Glory Framework — WebSocket Hub + Ticket HMAC (174A-69) — 2026-04-18

## Alcance
- Subcrate afectado: `glory-rs/backend`
- Módulo nuevo: `src/websocket/`
- Objetivo: dejar el framework con un hub websocket reusable y tickets HMAC de vida corta, sin acoplar todavía handlers HTTP ni dominio de negocio.

## Módulos creados
| Archivo | Responsabilidad |
|---|---|
| `glory-rs/backend/src/websocket/mod.rs` | Hub en memoria por `user_id`, registro/unregister y fanout a todas las conexiones del usuario |
| `glory-rs/backend/src/websocket/ticket.rs` | Generación y verificación de tickets HMAC-SHA256 (`user_id + exp + nonce`) |
| `glory-rs/backend/src/websocket/types.rs` | Envelope websocket serializable (`Ping`, `Pong`, `Authenticated`, `Event`, `Error`) |

## API expuesta por el framework
- `WebSocketHub::new(HubConfig)`
- `WebSocketHub::register(user_id, tx)`
- `WebSocketHub::unregister(ConnectionKey)`
- `WebSocketHub::broadcast_user(user_id, &WebSocketEnvelope)`
- `generate_ticket(user_id, ttl_secs, secret)`
- `verify_ticket(token, secret)`

## Decisiones de implementación
- El hub usa `DashMap<i32, Vec<SocketEntry>>` porque 174A-69 solo exige fanout in-memory local; Redis pub/sub se deja para 174A-73.
- El ticket HMAC replica el patrón ya probado en `src/services/download_token.rs`, pero generalizado a websocket con `nonce` UUID para evitar tokens triviales repetidos.
- El envelope del framework es genérico: el campo `Event { name, payload }` evita acoplar `glory-rs` a eventos específicos de Kamples.

## Dependencias añadidas al subcrate
- `axum` feature `ws`
- `dashmap`
- `hmac`
- `base64`
- `hex`

## Validación
- `cargo check` en `glory-rs/backend`
- `cargo clippy --all-targets -- -D warnings` en `glory-rs/backend`
- `cargo test` en `glory-rs/backend` → 9/9 tests verdes

## Gotchas
- El framework usa su propio `AppError`; por eso el ticket reutiliza `Unauthorized`/`Forbidden` del subcrate en vez del error del app principal.
- `broadcast_user` poda conexiones rotas durante el fanout para no acumular senders muertos.
- El hub no hace upgrade HTTP ni parsea query params; eso queda explícitamente fuera para 174A-70.

## Pendiente / TODO
- `GET /ws/ticket` y `GET /ws` en el backend principal usando este módulo (174A-70).
- Bridge multi-instancia por Redis pub/sub (174A-73).
- Eventos de conversaciones/mensajes/notificaciones sobre `WebSocketEnvelope::Event` (174A-72).
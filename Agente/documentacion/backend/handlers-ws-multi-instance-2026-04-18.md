# Handlers — WebSocket multi-instancia (174A-73) — 2026-04-18

## Alcance
- Tarea: `174A-73 — Multi-instancia: Redis pub/sub`
- Backend afectado: `glory-rust-template`
- Reutilizable consumido: `glory-rs/backend::websocket`

## Qué cambió
- El fanout websocket ya no depende sólo del hub local en memoria.
- Cada evento publicado para un usuario ahora sigue dos caminos:
  - fanout local inmediato sobre `AppState.ws_hub`
  - publish Redis al canal `ws:user:{id}` para que otros nodos lo reinyecten en sus sockets locales
- Cada proceso del backend genera un `ws_node_id` único al arrancar y abre un subscriber `PSUBSCRIBE ws:user:*`.
- El subscriber ignora los mensajes originados por su propio `ws_node_id` para evitar duplicados en el nodo emisor.

## Flujo runtime
1. El handler emite un evento de dominio con `crate::ws::emit_event(state, user_id, name, payload)`.
2. `emit_event` serializa una sola vez el `payload` a `WebSocketEnvelope::Event`.
3. El envelope se entrega localmente con `broadcast_user`.
4. Si Redis está configurado, también se publica un `RedisBridgeMessage` JSON en `ws:user:{id}`.
5. Todos los nodos suscritos reciben ese mensaje.
6. Cada nodo descarta el mensaje si `origin_node_id == ws_node_id`; en caso contrario hace fanout local para ese usuario.

## Estructura del bridge
- Canal Redis: `ws:user:{id}`
- Pattern de suscripción: `ws:user:*`
- Payload Redis:
  - `origin_node_id`
  - `user_id`
  - `envelope`

## Implementación
- `src/ws/mod.rs`
  - `emit_event(...)` pasa a ser async y publica en Redis además del hub local
  - `spawn_pubsub_bridge(...)` arranca el subscriber en background
  - `run_pubsub_bridge(...)` abre `redis::Client::get_async_pubsub()`, hace `psubscribe` y reinyecta envelopes remotos
  - helpers internos para `broadcast_envelope`, parseo del canal y serialización del bridge
- `src/lib.rs`
  - `AppState` ahora guarda `ws_node_id`
- `src/handlers/mod.rs`
  - genera `ws_node_id` al construir el router
  - arranca el bridge sólo cuando Redis está configurado y disponible
- `src/handlers/messages/events.rs`
  - la emisión de `mensaje_nuevo` ahora espera el publish async del bridge, pero sigue siendo fire-and-forget respecto al HTTP: cualquier error se loguea y no rompe el `201`

## Gotchas
- El subscriber Redis usa un `redis::Client` dedicado en lugar del pool `deadpool-redis`, porque pub/sub necesita una conexión larga y stateful.
- El publish sí reutiliza el pool ya existente del app para no abrir una conexión extra por evento.
- El bridge vive en el backend principal, no en `glory-rs`, porque depende de `deadpool-redis`, del `AppState` y de la política concreta de fallback local sin Redis.
- No hay cambios de OpenAPI ni frontend codegen: el contrato sigue siendo el mismo y sólo cambia la entrega cross-node.

## Validación
- `cargo check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `npm --prefix frontend run type-check`
- `npm run self-check -- -TareaId 174A-73`
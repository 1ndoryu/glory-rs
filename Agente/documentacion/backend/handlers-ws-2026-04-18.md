# Handlers WS — 2026-04-18

## Alcance
- Tarea: `174A-70 — GET /ws` upgrade + `GET /ws/ticket``
- Backend afectado: `glory-rust-template`
- Dependencia reusable consumida: `glory-rs/backend::websocket`

## Endpoints nuevos

### `GET /api/ws/ticket`
- Requiere `Authorization: Bearer <jwt>`.
- Emite un ticket HMAC de corta vida usando `WS_SECRET`.
- Respuesta:
  - `ticket`: token firmado para el handshake.
  - `ttl_secs`: TTL efectivo ya normalizado.
  - `ws_url`: URL websocket pública derivada de `WS_PUBLIC_URL` o `PUBLIC_BASE_URL`.

### `GET /api/ws?ticket=...`
- No usa JWT directo; el único credential permitido es el ticket HMAC.
- Valida el ticket con `glory_rs::websocket::verify_ticket`.
- Si el ticket es válido, ejecuta el upgrade Axum y registra la conexión en `AppState.ws_hub`.
- Al conectar, el servidor envía un envelope inicial:
  - `Authenticated { user_id, connection_id }`

## Runtime websocket mínimo
- Implementado en `src/ws/mod.rs`.
- Responsabilidades de este corte:
  - `register/unregister` en el hub reusable.
  - `Ping -> Pong` para mantener handshake mínimo verificable.
  - rechazo de mensajes no soportados con envelope `Error`.
  - poda natural al cerrarse la conexión o fallar el sender.
- Fuera de alcance para 174A-70:
  - conversaciones y mensajes
  - emisión de eventos de dominio
  - multi-instancia/Redis pub-sub

## Configuración nueva
- `WS_SECRET`
  - secret HMAC para tickets websocket.
  - si no está definido, `AppConfig` cae a `JWT_SECRET`.
- `WS_PUBLIC_URL`
  - URL websocket absoluta opcional para clientes.
- `WS_TICKET_TTL_SECS`
  - TTL default de tickets; el handler lo acota a `300s` máximo.

## Archivos tocados
- `Cargo.toml`
- `Cargo.lock`
- `.env.example`
- `src/config/mod.rs`
- `src/lib.rs`
- `src/handlers/mod.rs`
- `src/handlers/ws.rs`
- `src/ws/mod.rs`
- `openapi.json`
- `frontend/src/api/generated.ts`

## Validación
- `cargo check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test` → `134/134` verdes
- `cargo run -- --emit-openapi openapi.json`
- `npm run codegen`
- `npm --prefix frontend run type-check`

## Gotchas
- El handshake HTTP quedó separado de la lógica reusable del framework para no acoplar `glory-rs` al `AppState` de Kamples.
- El TTL del ticket se normaliza tanto en config como en runtime; valores `<= 0` vuelven a `60s`.
- Si el cliente intenta hablar antes de 174A-72, sólo recibirá `Pong` o envelopes `Error`; eso es intencional.
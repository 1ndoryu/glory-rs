# Handlers — Mensajes realtime (174A-72) — 2026-04-18

## Alcance
- Tarea: `174A-72 — WS events emitidos`
- Backend afectado: `glory-rust-template`
- Dependencia reusable consumida: `glory-rs/backend::websocket`

## Qué cambió
- El envío exitoso de un mensaje directo ahora emite un evento websocket de dominio.
- El nombre del evento quedó fijado en `mensaje_nuevo` para mantener compatibilidad con el cliente legado.
- El transporte sigue usando el envelope reusable del hub (`type = event`, `name`, `payload`), pero el `payload` se proyecta al shape legacy esperado por el frontend viejo.

## Contrato realtime
- Destino: sólo el destinatario del mensaje, no el remitente.
- Motivo: los listeners legacy de notificaciones reaccionan a `mensaje_nuevo` sin filtrar autoenvíos; enviar al remitente generaría falsos avisos locales.
- Shape del payload:
  - `conversacionId`
  - `mensaje`
    - `id`
    - `conversacionId`
    - `remitenteId`
    - `contenido`
    - `tipo`
    - `mediaUrl`
    - `mediaMetadata`
    - `leido`
    - `creadoAt`

## Proyección de metadata
- `imagen/audio`
  - `formato`
  - `tamano`
  - `mimeType`
- `sample`
  - `sampleId`
  - `titulo`
  - `idCorto`
  - `slug`
  - `tipo`
  - `bpm`
  - `key`

## Implementación
- `src/ws/mod.rs`
  - añade `emit_event<T: Serialize>(...)` como helper local del app para emitir eventos de dominio sobre `WebSocketHub`
  - adapta el error reusable del framework al `AppError` local
- `src/services/notification_fanout.rs`
  - desde `174A-78` concentra la proyección legacy-compatible de `mensaje_nuevo`
  - dispara websocket + Web Push + FCM como side-effect unificado del envío de mensajes
- `src/handlers/messages.rs`
  - tras persistir y normalizar el mensaje, delega en `NotificationFanoutService::dispatch_new_message(...)`
  - mantiene el envío HTTP exitoso aunque falle algún canal secundario

## Gotchas
- El repo penaliza controladores grandes; por eso la serialización websocket terminó consolidada en `NotificationFanoutService` junto a push/FCM, en vez de volver a crecer `messages.rs`.
- El helper reusable del framework y el backend principal usan tipos `AppError` distintos; la frontera correcta sigue siendo `src/ws/mod.rs`.
- No hubo cambios de OpenAPI ni de codegen frontend: cambió sólo el side-effect del `POST /api/mensajes/{conversacionId}`.

## Validación
- `cargo check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `npm --prefix frontend run type-check`
- `npm run self-check -- -TareaId 174A-72`
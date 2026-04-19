# Service `notification_fanout` — Pipeline integrado (174A-78) — 2026-04-19

## Alcance
- Tarea: `174A-78 — Pipeline integrado notify(user, event)`.
- Se centralizó el fanout multicanal para eventos sociales y de mensajería sin volver a duplicar reglas legacy en cada handler.
- Canales conectados en este corte:
  - persistencia in-app (`notificaciones`)
  - websocket `notificacion`
  - websocket `mensaje_nuevo`
  - Web Push
  - FCM
- El branch de email quedó preparado pero explícitamente desactivado hasta que exista una preferencia backend persistida para notificaciones por correo.

## Contrato de compatibilidad
- Eventos sociales (`follow`, likes, comentarios, replies) siguen el flujo legacy de `ServicioNotificaciones::crear()`:
  1. persistir en `notificaciones`
  2. emitir websocket `notificacion`
  3. intentar Web Push
  4. intentar FCM
- Mensajes directos mantienen el contrato legacy separado:
  - websocket `mensaje_nuevo`
  - push/FCM con canal `mensajes`
  - sin insertar en `notificaciones`
- La deduplicación y el skip de auto-notificaciones siguen viviendo en `NotificationService`, no en handlers.

## Productores conectados
- `POST /api/follow/{userId}`
  - produce `follow`
- `POST /api/like`
  - `sample` → `like` / `encanta`
  - `publicacion` → `like`
- `POST /api/comentarios/{tipo}/{targetId}`
  - comentario raíz en sample/publicación → `comentario`
  - reply → `comentario` dirigido al autor del padre
- `POST /api/comentarios/{commentId}/like`
  - produce `like` al autor del comentario
- `POST /api/mensajes/{conversacionId}`
  - emite `mensaje_nuevo` y dispara push/FCM si los runtimes están configurados

## Metadata auxiliar
- `src/repositories/notification_target.rs` agrega lookups mínimos de metadata para no mezclar SQL ad-hoc en handlers:
  - sample: `creator_id`, `title`, `slug`
  - publicación: `author_id`, `content`

## Smoke test local
- Verificado con backend local (`cargo run`) y usuarios temporales:
  - `POST /api/follow/{userId}` → `200`, inserta notificación y dispara el fanout sin romper el request.
  - `POST /api/mensajes/nueva` + `POST /api/mensajes/{conversacionId}` → `201`, emite `mensaje_nuevo` y pasa por push/FCM en modo no-op si no hay configuración.
- Limitación observada durante la verificación:
  - `POST /api/publicaciones` devuelve `500` por un decode error preexistente en el handler de posts (`unexpected null` en columna 30), lo que impidió validar manualmente likes/comentarios sobre publicaciones en este corte.

## Validación ejecutada
- `cargo sqlx prepare --workspace`
- `cargo check`
- `cargo clippy -- -D warnings`
- `cargo test`
- smoke test local por HTTP: `health`, `follow`, `mensajes/nueva`, `mensajes/{id}`
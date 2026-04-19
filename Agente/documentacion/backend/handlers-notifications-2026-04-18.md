# 174A-74 — Notificaciones persistentes base

## Alcance
- Se portó la base del dominio in-app sobre la tabla `notificaciones` ya creada en migraciones.
- El backend expone los endpoints legacy del dropdown: `GET /api/notificaciones`, `POST /api/notificaciones/{id}/leer`, `POST /api/notificaciones/leer-todas` y `GET /api/notificaciones/conteo`.
- Se añadió `NotificationService` como punto único para crear notificaciones persistentes con exclusión de auto-notificaciones y deduplicación temporal compatible con el legado.

## Contrato HTTP
- Lista: `{ data: UserNotification[] }`.
- Conteo: `{ total }`.
- Item de notificación:
  - `id`, `tipo`, `titulo`, `mensaje`, `datos`, `leida`, `enlace`
  - `creadaAt`
  - `actor?: { username, nombreVisible, avatarUrl }`
- El shape mantiene `creadaAt` y `actor.nombreVisible/avatarUrl` para no romper consumidores legacy.

## Reglas de negocio portadas
- Auto-notificación: si `actor_id === destinatario_id`, `NotificationService::create()` no inserta nada.
- Deduplicación centralizada por tipo:
  - `like`, `encanta`, `follow`: 24h
  - `comentario`: 5 min
  - `venta`: 1h
- El dedup compara `usuario_id + tipo + actor_id + datos(JSON)` dentro de la ventana, evitando colisiones falsas entre entidades distintas.
- El listado filtra actores bloqueados y actores inactivos, igual que el backend PHP.

## Implementación
- `src/repositories/notification.rs`
  - listado con actor (`LEFT JOIN usuarios_ext`)
  - marcar leída / marcar todas
  - conteo no leído
  - inserción completa
  - verificación de dedup reciente
- `src/services/notification.rs`
  - `create()` con reglas de exclusión y dedup
  - `list_for_user()` con paginación legacy fija a 30
- `src/handlers/notifications.rs`
  - rutas HTTP + normalización de `avatarUrl` a URL pública

## Ajuste posterior — 174A-78
- `NotificationService` sigue siendo la puerta de persistencia/dedup, pero ya no vive aislado.
- `NotificationFanoutService` lo usa como primera etapa y después dispara:
  - websocket `notificacion`
  - Web Push
  - FCM
- Productores ya conectados:
  - follow
  - likes de sample/publicación
  - comentarios y replies
  - like a comentario
- Mensajes directos siguen fuera de `notificaciones` por compatibilidad legacy; sólo usan `mensaje_nuevo` + push/FCM.

## Validación ejecutada
- `cargo sqlx prepare`
- `cargo check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo run -- --emit-openapi openapi.json`
- `npm run codegen`
- `npm run self-check -- -TareaId 174A-74`
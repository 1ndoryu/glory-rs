# Handlers — Mensajes directos (174A-71) — 2026-04-18

## Endpoints
| Método | Path | Auth | Descripción |
|---|---|---|---|
| GET | `/api/mensajes/conversaciones` | sí | Lista mis conversaciones visibles, ya enriquecidas con participante y último mensaje |
| POST | `/api/mensajes/nueva` | sí | Crea o reutiliza la conversación 1:1 con otro usuario |
| GET | `/api/mensajes/{conversacionId}` | sí | Lista mensajes paginados de una conversación (`before_id`, `limit`) |
| POST | `/api/mensajes/{conversacionId}` | sí | Envía mensaje `texto`, `imagen`, `audio` o `sample` |
| POST | `/api/mensajes/{conversacionId}/leer` | sí | Marca como leídos los mensajes recibidos en esa conversación |
| POST | `/api/mensajes/leer-todas` | sí | Marca como leídos todos mis mensajes pendientes |

## Reglas de dominio
- Solo se permiten conversaciones directas 1:1.
- Crear conversación con uno mismo falla con `400`.
- El endpoint reutiliza la conversación existente si ya había una fila para el mismo par canónico de usuarios.
- Todas las rutas filtran bloqueos bidireccionales (`list_blocked + list_blockers`).
- Tanto el emisor como el destinatario deben seguir con `perfil.estado = 'activo'`.
- `GET /api/mensajes/{conversacionId}` solo devuelve la conversación si el viewer participa en ella.
- `POST /api/mensajes/{conversacionId}` acepta:
  - JSON `{ contenido, tipo, sample_id }`.
  - `multipart/form-data` con `media`, `contenido`, `tipo`, `sample_id`.
- `tipo` solo admite `texto`, `imagen`, `audio`, `sample`.
- Mensaje vacío solo se acepta si hay media adjunta o si el tipo es `sample`.
- El contenido textual se recorta a `5000` caracteres y se bloquean patrones obvios de spam antes de persistir.

## Multimedia y sample-share
- La media válida se limita a imágenes y audio.
- Límites actuales:
  - imagen: `10 MiB`
  - audio: `30 MiB`
- Los archivos se guardan bajo `messages/{user_id}/{yyyy}/{mm}/{uuid}.{ext}`.
- La respuesta pública normaliza `media_url` contra `PUBLIC_BASE_URL` o cae a `/uploads/...`.
- `media_metadata` guarda `content_type`, `size_bytes`, `original_filename`, `extension` y `media_kind`.
- Los mensajes `sample` serializan en `media_metadata` el snapshot mínimo del sample compartido (`sample_id`, `titulo`, `id_corto`, `slug`, `tipo`, `bpm`, `key`).

## Implementación
- `src/repositories/conversation.rs`
  - listado de conversaciones
  - lookup 1:1 por par canónico
  - creación idempotente con `ON CONFLICT`
  - verificación de participación y lookup del otro participante
- `src/repositories/message.rs`
  - listado paginado por conversación
  - create atómico: insert del mensaje + update de `ultimo_mensaje_at` y `aceptada`
  - `mark_read`, `mark_all_read_for_user`
  - lookup del sample compartible
- `src/handlers/messages/payload.rs`
  - parser unificado JSON/multipart
  - detección de tipo por MIME/extensión
  - validación de shape y construcción de storage key
- `src/handlers/messages.rs`
  - reglas de acceso
  - normalización de URLs públicas
  - wiring REST + OpenAPI

## Gotchas
- `SQLX_OFFLINE=true` obligó a regenerar `.sqlx/` con `cargo sqlx prepare` antes de poder cerrar la compilación.
- El create de mensajes borra el archivo recién subido si falla la persistencia SQL, para no dejar huérfanos en storage.
- El modelo deja preparado el dominio REST completo, pero no emite eventos websocket ni notificaciones; eso sigue en `174A-72` y `174A-74`.
- La conversación se marca `aceptada = TRUE` cuando entra el primer mensaje, igual que en el flujo legado.

## Validación
- `cargo sqlx prepare`
- `cargo check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test` → `137/137` verdes
- `cargo run -- --emit-openapi openapi.json`
- `npm run codegen`
- `npm --prefix frontend run type-check`
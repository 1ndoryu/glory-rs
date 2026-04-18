# Handlers — Comentarios polimórficos (174A-68) — 2026-04-18

## Endpoints
| Método | Path | Auth | Descripción |
|---|---|---|---|
| GET | `/api/comentarios/{tipo}/{targetId}` | opcional | Lista comentarios raíz visibles del target |
| GET | `/api/comentarios/{commentId}/respuestas` | opcional | Lista respuestas visibles de un comentario |
| POST | `/api/comentarios/{tipo}/{targetId}` | sí | Crear comentario texto o multimedia (JSON o multipart) |
| PUT | `/api/comentarios/{commentId}` | sí | Editar contenido de comentario propio |
| DELETE | `/api/comentarios/{commentId}` | sí | Borrar comentario propio y su subárbol |
| POST | `/api/comentarios/{commentId}/like` | sí | Crear o actualizar reacción sobre comentario |
| DELETE | `/api/comentarios/{commentId}/like` | sí | Quitar mi reacción del comentario |

## Targets soportados
- `sample`
- `publicacion`
- `cancion`
- `relacion`
- `articulo`

## Reglas de dominio
- `GET` admite viewer anónimo; si llega bearer se enriquece `mi_reaccion` y se filtran bloqueos bidireccionales.
- `POST` soporta dos contratos en el mismo path:
  - JSON `{ contenido, parent_id }` para texto.
  - `multipart/form-data` con `media` opcional, `contenido`, `parent_id`, `tipo_contenido`.
- `tipo_contenido` solo admite `texto`, `imagen`, `audio`.
- Comentario vacío solo se permite cuando existe `media` adjunta.
- `parent_id` debe existir dentro del mismo contexto `tipo + target_id`.
- `DELETE` borra en cascada el subárbol de respuestas y después recalcula `total_comentarios` del target.
- `PUT` no cambia ni reemplaza media; solo actualiza `contenido` y exige autoría.
- El listado filtra `moderacion_estado = 'rechazado'` y autores bloqueados en cualquier dirección.

## Multimedia
- Se aceptan imágenes y audio; otros MIME fallan con `415`.
- Límite actual: imágenes `8 MiB`, audio `24 MiB`.
- La media se guarda bajo `comments/{user}/{yyyy}/{mm}/{uuid}.{ext}` y la respuesta expone URL pública vía `public_base_url` / `/uploads/...`.
- `media_metadata` guarda `content_type`, `size_bytes`, `original_filename`, `extension`, `media_kind`.

## Respuesta enriquecida
- `CommentDetail` devuelve autor resumido, contadores (`total_likes`, `total_respuestas`), `liked`, `mi_reaccion`, `media_url` pública y `media_metadata` JSON.
- `GET /respuestas` mantiene orden cronológico ascendente; raíces ordenan por `total_likes DESC, created_at DESC`.

## Decisiones de implementación
- El parser de creación se separó a `src/handlers/comments/payload.rs` para mantener el controlador principal dentro del límite de líneas del repo.
- `CommentRepository` usa `sqlx::query!` / `query_as!` en todas las rutas críticas; el recount por target vive en ramas explícitas para no volver a SQL runtime.
- Se extendió `LikeKind::Comentario` para compartir infraestructura de likes sin duplicar tabla ni lógica de upsert.

## Gotchas
- El cleanup de media al borrar usa un CTE recursivo para cubrir también replies con archivos adjuntos; si no se hace así, quedan huérfanos en storage.
- `articulo` ya estaba contemplado por el schema social aunque la tarea del blog llegue más tarde; por eso se respetó el polimorfismo completo en vez de hardcodear solo samples/posts.
- El endpoint create documenta multipart para favorecer codegen, pero sigue aceptando JSON simple para paridad con el legado/desktop.

## Pendiente / TODO
- Notificaciones por comentario/reply cuando exista Fase 11.
- Moderación IA específica de comentarios cuando se porte la cola correspondiente.
- Rate limit específico de comentarios cuando exista RateLimiter global.
# Handlers — Publicaciones (174A-67) — 2026-04-18

## Endpoints
| Método | Path | Auth | Descripción |
|---|---|---|---|
| POST | `/api/publicaciones` | sí | Crear publicación social |
| GET | `/api/publicaciones` | sí | Listar publicaciones visibles |
| GET | `/api/publicaciones/:id` | sí | Obtener detalle de una publicación |
| PUT | `/api/publicaciones/:id` | sí | Editar publicación propia |
| DELETE | `/api/publicaciones/:id` | sí | Soft-delete de publicación propia |
| POST | `/api/publicaciones/:id/repost` | sí | Crear repost propio del post original |
| DELETE | `/api/publicaciones/:id/repost` | sí | Quitar mi repost del post original |

## Reglas de dominio
- Toda ruta requiere `CurrentUser`.
- `PUT` y `DELETE` solo aplican a publicaciones originales del autor. Si el registro es un repost, la mutación correcta es `DELETE /api/publicaciones/:id/repost`.
- `POST /repost` rechaza repostear la propia publicación y también repostear un repost.
- Listado y detalle filtran `eliminado_en IS NULL`, `moderacion_estado != 'rechazado'`, y autores bloqueados en cualquier dirección (`list_blocked` + `list_blockers`).
- `samples_adjuntos` se valida contra `samples` activos antes de crear o editar.
- Filtros soportados en `GET /api/publicaciones`: `todos`, `siguiendo`, `populares`.

## Respuesta enriquecida
- `PostDetail` devuelve autor, contadores sociales, reacción del viewer (`mi_reaccion`), si ya reposteó (`yo_ya_repostee`) y, cuando aplica, `repost_original` con snapshot del post origen.
- El listado y el detalle comparten el mismo shape SQL para evitar drift entre endpoints.

## Decisiones de implementación
- El dominio se separó en `src/handlers/posts.rs` y `src/repositories/post.rs` para no seguir inflando handlers sociales previos.
- `PostRepository::list` recibe `PostListParams` para encapsular filtros y mantener clippy verde sin romper el query único que hace joins y flags en una sola pasada.
- `PostRow` quedó plano a propósito: `sqlx::query_as!` no mapea bien una estructura anidada para flags calculadas (`siguiendo_autor`, `yo_ya_repostee`, verificados, etc.).

## Gotchas
- El repost se modela como fila nueva en `publicaciones` con `repost_id`, no como toggle sobre la publicación original.
- `LikeRepository` ya soportaba `tipo=publicacion`; en 174A-67 solo se endureció el target para excluir posts soft-deleted.
- Comentarios, likes sobre comentarios y multimedia conversacional quedan para 174A-68.

## Pendiente / TODO
- Notificaciones al autor original cuando exista Fase 11.
- Rate limit específico de publicaciones/reposts si reaparece abuso.
- Integrar conteo real de comentarios cuando se porte el dominio de comentarios.
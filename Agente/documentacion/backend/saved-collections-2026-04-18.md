# Saved Collections (174A-66) — 2026-04-18

## Endpoints
| Método | Path | Auth | Descripción |
|---|---|---|---|
| POST   | `/api/colecciones/:id/save`         | sí | Guarda (bookmark) coleccion. Idempotente. |
| DELETE | `/api/colecciones/:id/save`         | sí | Quita el bookmark. Idempotente. |
| GET    | `/api/me/colecciones-guardadas`     | sí | Lista paginada (`limit`, `offset`) de colecciones guardadas. |

## Tabla `colecciones_guardadas` (migración 20260418000031)
- `usuario_id INT FK → usuarios_ext`
- `coleccion_id BIGINT FK → colecciones`
- `created_at TIMESTAMPTZ DEFAULT NOW()`
- PK compuesta `(usuario_id, coleccion_id)`
- Índices: `(usuario_id, created_at DESC)` y `(coleccion_id)`.
- **DROP CASCADE** previo de la versión legacy (con `coleccion_id INT`) por
  incompatibilidad de tipos con la nueva `colecciones.id BIGINT`.

## Reglas
- Guardar requiere que la colección sea pública o pertenezca al usuario.
- Quitar bookmark es siempre permitido (no expone existencia).
- Listado solo devuelve colecciones no soft-deleted.

## Implementación
- Repo `SavedCollectionsRepository` con `save`/`unsave`/`is_saved`/`list_by_user`.
- `save` usa `ON CONFLICT DO NOTHING` (PK compuesta), idempotente.
- `list_by_user` devuelve `SavedColeccion` (campos planos del JOIN + `guardada_at`).

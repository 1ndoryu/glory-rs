# Handlers — Colecciones (174A-64) — 2026-04-18

## Endpoints
| Método | Path | Auth | Descripción |
|---|---|---|---|
| POST   | `/api/colecciones`                              | sí | Crear colección |
| GET    | `/api/colecciones`                              | sí | Listar colecciones del usuario |
| GET    | `/api/colecciones/:id`                          | sí | Obtener (owner o `publica=true`) |
| PUT    | `/api/colecciones/:id`                          | sí | Actualizar (owner) |
| DELETE | `/api/colecciones/:id`                          | sí | Soft-delete (owner) |
| POST   | `/api/colecciones/:id/samples`                  | sí | Agregar sample (orden = MAX+1) |
| DELETE | `/api/colecciones/:id/samples/:sample_id`       | sí | Quitar sample |
| GET    | `/api/colecciones/:id/samples`                  | sí | Listar samples (ORDER BY orden) |

## Esquema BD (`migrations/20260418000030_colecciones.up.sql`)
- `colecciones(id BIGSERIAL, usuario_id INT FK→usuarios_ext, nombre, descripcion, publica, parent_id BIGINT FK self, imagen_url, version, total_samples, created_at, updated_at, eliminado_en)`.
- `coleccion_samples(coleccion_id BIGINT, sample_id INT, orden INT, added_at, PK compuesta)`.
- Índice único parcial `idx_colecciones_nombre_unico (usuario_id, COALESCE(parent_id,0), nombre) WHERE eliminado_en IS NULL`.

## Reglas
- Profundidad máxima 2 niveles (parent.parent_id debe ser NULL).
- Una colección no puede ser su propio padre.
- Mutaciones requieren `is_owner`. Lectura permitida si `publica` o owner.
- Conflicto de nombre devuelve `409` (`AppError::Conflict`).
- Add sample idempotente (duplicado devuelve `ok:true`, no error).
- Soft-delete: marca `eliminado_en`; libera el unique constraint.

## DTOs
- `UpdateColeccionRequest` usa `Option<Option<T>>` con deserializer custom para distinguir
  "campo ausente" de "campo presente con null" (necesario para limpiar `descripcion`,
  `imagen_url`, `parent_id`). Lint `clippy::option_option` allowed con justificación.

## GOTCHAs encontrados
- **Tabla legacy `colecciones_guardadas`** existía con `coleccion_id integer` apuntando a
  una tabla `colecciones` legacy con `id integer`. Tras el `DROP TABLE colecciones CASCADE`
  para resetear la migración, la FK quedó huérfana. Se difiere su recreación a 174A-66
  (Saved collections), donde se recreará con `coleccion_id BIGINT`.
- **`IF NOT EXISTS` + tabla legacy** ⇒ los `CREATE INDEX ... WHERE col IS NULL` fallan si
  la tabla previa carecía de la columna. En dev: drop cascade + delete `_sqlx_migrations`.
- **`utoipa::path` params** requieren `description` aunque parezcan opcionales — sin él,
  el macro emite "expected ,".

## Pendiente / TODO
- Optimistic locking (`version`).
- Sync changelog para offline-first.
- Subir imagen (multipart).
- Eliminación con opciones (cascada de hijas, borrar samples).
- Endpoint admin override.

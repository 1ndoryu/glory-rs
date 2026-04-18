# Handler — Merge colecciones (174A-65) — 2026-04-18

## Endpoint
`POST /api/colecciones/:id/merge` con body `{ "source_id": <i64> }`.

- `:id` es el **target** (la que sobrevive).
- `source_id` es la que se vacía y soft-deletea.

## Reglas
- Ambas colecciones deben existir y pertenecer al usuario autenticado.
- `source_id != target_id` (devuelve 400).
- Inserta los samples de source que no estén en target, conservando su orden
  relativo y asignándoles un `orden` continuo después del último de target.
- Vacía `coleccion_samples` de source y soft-deletea source.
- Actualiza `total_samples` del target con los movidos efectivamente (sin contar
  duplicados que ya estaban en target).

## Implementación
- `ColeccionesRepository::merge(pool, target_id, source_id)` en transacción.
- Usa `ROW_NUMBER() OVER (ORDER BY orden, added_at)` y suma del `next_orden`
  base para asignar posiciones contiguas.
- `is_owner` valida que ambas pertenezcan al usuario y no estén soft-deleted.

## GOTCHAs
- `rows_affected()` devuelve `u64`; clippy bloquea cast directo a `i64`/`i32`.
  Usar `i64::try_from(...).unwrap_or(i64::MAX)`.

## TODO
- Soporte para parent (al mergear, decidir qué pasa con hijas del source).
- Sync changelog para clientes offline.

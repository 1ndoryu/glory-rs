# Sync changelog endpoint — 2026-04-19

## Contrato

`GET /api/sync/changelog?cursor={n}&limite={n}` (también acepta `since=` como alias por compat con la spec del roadmap).

- **Auth:** Bearer JWT obligatorio. El `usuarioId` se toma SIEMPRE del token (`CurrentUser.user_id`), nunca de query — diferencia frente al legado PHP que lo leía de la sesión.
- **Parámetros:**
  - `cursor` (opcional, `i64`): último ID de changelog que el cliente conoce. Si <=0 o ausente → respuesta `full_sync_required=true` con `cursor` apuntando al `MAX(id)` actual del usuario.
  - `since` (opcional): alias de `cursor`. Si ambos están presentes, `cursor` gana.
  - `limite` (opcional, default 100): clamp `[1, 500]`.
- **Respuesta `SyncChangelogDelta`:**
  - `cambios: SyncChangelogEntry[]` — orden ascendente por `id`.
  - `cursor: i64` — máximo `id` devuelto (o el cursor original si vacío). El cliente debe usarlo como `cursor` en la siguiente petición.
  - `hay_mas: bool` — `true` si la página llenó el límite y hay más entradas pendientes.
  - `full_sync_required: bool` — `true` cuando se detecta purga (cursor < `MIN(id)` actual) o conexión inicial. El cliente debe descartar caché local y rehidratar desde colecciones/samples.

## Tipos de evento (`SyncChangelogTipo`)

`sample_added`, `sample_removed`, `sample_updated`, `collection_created`, `collection_renamed`, `collection_deleted`, `collection_merged`. Mapeados 1:1 con la `CHECK` de la migración `20260417000011_indexes_planificador_changelog.up.sql`.

## Detección de purga

Replicado del legado: si la query devuelve 0 filas y existe `MIN(id) > cursor`, significa que el changelog fue purgado y el cliente está desincronizado → `full_sync_required=true`.

## Notas de implementación

- Repositorio: `SyncChangelogRepository::delta(pool, usuario_id, cursor, limite)` en `src/repositories/sync_changelog.rs`.
- Modelos: `src/models/sync.rs` (`SyncChangelogTipo`, `SyncChangelogEntry`, `SyncChangelogDelta`, `SyncChangelogQuery`).
- Handler: `src/handlers/sync.rs`. Tag OpenAPI `sync`.
- Fetch interno: `LIMIT limite + 1` para detectar `hay_mas` sin segunda query.
- `query!`/`query_scalar!` con SQLx offline → requiere `cargo sqlx prepare` en cambios futuros.

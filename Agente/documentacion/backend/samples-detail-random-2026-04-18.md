# 174A-45 — GET /samples/{slug} y GET /samples/random

## Objetivo

Completar la capa pública de lectura de samples con dos rutas faltantes:

- detalle por `slug` o `id_corto`
- sample aleatorio del catálogo público

La implementación también corrige un gap detectado al cerrar `174A-44`: el catálogo estaba devolviendo storage keys crudas en lugar de URLs públicas para previews, waveforms e imágenes.

## Qué quedó implementado

- `GET /api/samples/{slug}` en `src/handlers/sample_catalog.rs`.
- `GET /api/samples/random` en el mismo handler, registrado antes de `:slug` para evitar colisiones de routing.
- Nuevo DTO `SampleDetailResponse` con estado, metadata, tamaños, visibilidad y creator summary.
- Normalización centralizada de assets en `SampleCatalogService`:
  - storage key relativa → `/uploads/...`
  - URL absoluta existente → se preserva
  - `PUBLIC_BASE_URL` → se antepone cuando existe
- `ruta_original` y `ruta_optimizada` solo se exponen si el usuario autenticado es el creador del sample.
- Queries de detalle y random migradas a `sqlx::query_as!` y cacheadas en `.sqlx/` para respetar `SQLX_OFFLINE=true`.

## Decisiones

- El detalle mantiene la semántica legacy de lookup dual `slug`/`id_corto` y solo excluye `eliminado_en`, no estados intermedios. Esto conserva la capacidad de inspeccionar un sample en `procesando` o `en_supervision` desde el enlace directo.
- El random endpoint solo toma samples `activo` y `mostrar_en_comunidad = true`, eligiendo aleatoriamente dentro del top `1000` más recientes para evitar un `ORDER BY RANDOM()` sobre toda la tabla.
- La normalización de URLs se aplicó también al listado ya implementado en `174A-44`, porque dejar el catálogo con storage keys crudas habría roto el contrato público en cuanto aparecieran previews reales.

## Validación

- `cargo sqlx prepare`
- `cargo check`
- `cargo test sample_catalog`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `npm run codegen`
- `npm --prefix frontend run type-check`

## Pendiente relacionado

- `174A-46` añadirá mutaciones owner-only (`PATCH` y `DELETE`) sobre el detalle.
- `174A-56` reabrirá `GET /samples/random` para soportar `seed` y su integración con el feed algorítmico.
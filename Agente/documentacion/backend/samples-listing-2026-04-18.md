# 174A-44 — GET /samples con paginación y filtros

## Objetivo

Migrar el listado público de samples a Rust sin depender todavía de la búsqueda fuzzy ni de similitud. La tarea cubre el contrato base de catálogo para que frontend y desktop puedan pedir páginas filtradas por BPM, key, type, tags, premium y creator.

## Qué quedó implementado

- Nuevo handler público `GET /api/samples` en `src/handlers/sample_catalog.rs`.
- Query params documentados en OpenAPI: `page`, `per_page`, `bpm`, `key`, `type`, `tags`, `premium`, `creator`.
- Alias legacy soportados en parsing: `tipo`, `creador`, `es_premium`.
- Servicio `SampleCatalogService` para validar y normalizar filtros antes de tocar SQL.
- Repositorio separado `sample_catalog.rs` con `QueryBuilder` y binds preparados para evitar un árbol de combinaciones con `sqlx::query!`.
- Respuesta paginada con `data[]` + `pagination { page, per_page, total, pages }`.

## Decisiones

- El listado público queda limitado a samples `activo`, no eliminados y con `mostrar_en_comunidad = true`.
- El filtro por `creator` usa `usuarios_ext.username` porque es el lookup público estable del sistema; no expone IDs como API principal del catálogo.
- `tags` filtra contra `tags_enriquecidos` para acercarse al comportamiento discovery del legado sin adelantar todavía la búsqueda textual fuzzy de `174A-47`.
- El query se movió a `src/repositories/sample_catalog.rs` para no seguir inflando `src/repositories/sample.rs`, que ya concentra upload y pipeline.

## Validación

- `cargo check`
- `cargo test sample_catalog`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `npm run codegen`
- `npm --prefix frontend run type-check`

## Pendiente relacionado

- `174A-45` completará el detalle por slug y el random endpoint.
- `174A-47` añadirá búsqueda fuzzy sobre título/tags, hoy deliberadamente fuera de este scope.
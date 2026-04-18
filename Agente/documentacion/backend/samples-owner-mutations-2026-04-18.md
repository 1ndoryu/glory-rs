# 174A-46 — PATCH /samples/{slug} y DELETE /samples/{slug}

## Objetivo

Completar las mutaciones owner-only que faltaban sobre el detalle de samples:

- edición parcial por `slug` o `id_corto`
- borrado lógico con `soft-delete`

La tarea se resolvió sin arrastrar la rama legacy de hard-delete admin, porque el roadmap pedía explícitamente `owner check` más `soft-delete`.

## Qué quedó implementado

- `PATCH /api/samples/{slug}` en `src/handlers/sample_catalog.rs` con auth obligatoria.
- `DELETE /api/samples/{slug}` en el mismo handler, también autenticado.
- Nuevo payload `UpdateSampleRequest` y respuesta `DeleteSampleResponse` en `src/models/sample.rs`.
- Lookup mínimo owner-only en repositorio para resolver `slug`/`id_corto` sin cargar todo el detalle cuando solo se necesita ownership.
- `UpdateSamplePatch` interno con soporte tri-state para campos anulables:
  - `precio = Some(None)` para limpiar precio al desactivar premium
  - `imagen_url = Some(None)` para limpiar portada cuando llega vacío
- `soft_delete_owned_sample()` que marca `estado = 'eliminado'`, `eliminado_en = NOW()` y mantiene el asset físico intacto.

## Reglas de negocio aplicadas

- Solo el creador autenticado puede editar o enviar a papelera un sample.
- El lookup sigue aceptando `slug` o `id_corto`, igual que el detalle público.
- `titulo` se trimmea y no puede quedar vacío.
- `descripcion` se trimmea.
- `tags` se normalizan a lowercase + dedupe y exigen al menos `2` valores.
- `type` / `tipo` reutiliza la misma normalización del catálogo (`loop`, `oneshot`, `fx`, `vocal`, `stem`, `otro`).
- Si `es_premium = false`, el precio se limpia automáticamente.
- Si el mismo patch intenta enviar `precio` junto con `es_premium = false`, se rechaza como inconsistente.

## Decisiones

- La mutación quedó en `sample_catalog` en vez de abrir otro módulo paralelo porque este dominio ya concentra el lookup por `slug`/`id_corto`, la normalización pública y el DTO de detalle. Separarlo ahora solo habría duplicado plumbing.
- El update SQL usa `QueryBuilder<Postgres>` porque los campos son opcionales y combinables; forzar `query!` por cada combinación sería ruido y peor mantenibilidad. En cambio, el lookup owner-only y el soft-delete sí quedaron con macros `sqlx::query_as!` / `sqlx::query!` y su cache `.sqlx` correspondiente.

## Validación

- `cargo sqlx prepare`
- `cargo check`
- `cargo test sample_catalog`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `npm run codegen`
- `npm --prefix frontend run type-check`

## Pendiente relacionado

- `174A-47` sigue con búsqueda fuzzy trigram sobre el catálogo ya mutable.
# 174A-85 — Reportes (legales, contenido, errores)

## Alcance portado

- `POST /api/reportar`
- `POST /api/reportar-usuario/{userId}`
- `POST /api/reportar-error`
- `POST /api/reportar-legal`
- `GET /api/admin/reportes/legales`
- `POST /api/comentarios/{id}/reportar`
- `POST /api/publicaciones/{id}/reportar`

## Backend

- Se añadió `src/models/report.rs` con DTOs y enums para reportes genéricos, legales y admin.
- Se añadió `src/repositories/report.rs` para creación, rate limiting, duplicados, conteos pendientes y listado admin de legales.
- `src/handlers/reports.rs` concentra las rutas públicas/autenticadas y delega helpers en `src/handlers/reports/support.rs` para respetar el límite de tamaño de controladores.
- `migrations/20260419000033_reports_public_legal.up.sql` vuelve nullable `reportador_id` y agrega `ip_origen` para reclamaciones legales públicas.

## Paridad legacy relevante

- Duplicados idempotentes solo para `publicacion`, `comentario` y `sample`.
- Rate limit genérico: `10` reportes por hora por tipo/reportador.
- `reportar-error`: `5` por usuario cada `24h`.
- `reportar-legal`: `3` por IP cada `1h`.
- Auto-hide de samples y publicaciones con `3` reportes pendientes, preservando visibilidad para el creador.
- Auto-suspensión de usuario al acumular `4` reportes de tipo `usuario` en `2h`, por `48h`.

## Integraciones de visibilidad

- Catálogo público de samples: listado y similares filtran en SQL dinámico; detalle y random validan visibilidad post-query.
- Feed: se filtran samples ocultos después del ranking, manteniendo visibles los del propio creador.
- Publicaciones: `get` y `list` excluyen publicaciones y reposts auto-ocultados salvo para su autor.

## Validación ejecutada

- `cargo sqlx migrate run`
- `cargo sqlx prepare`
- `cargo check`
- `cargo run -- --emit-openapi openapi.json`
- `npm --prefix frontend run codegen`
- `npm --prefix frontend run type-check`

## Limitaciones conocidas

- No se añadió UI específica nueva en frontend; este corte deja el contrato OpenAPI/Orval listo para consumir los endpoints.
- No se ejecutó `cargo clippy` ni test suite completa en este cierre.
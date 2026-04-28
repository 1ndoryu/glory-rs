# Admin Experimentos, Embeddings y Benchmark — 2026-04-28

## Alcance

- `274A-49`: `POST /api/admin/embeddings/generar` genera embeddings faltantes para samples activos.
- `274A-50`: `POST /api/admin/embeddings/regenerar` limpia y recalcula todos los embeddings activos.
- `274A-51`: `POST /api/admin/experimentos/generar` crea datos sociales reales de prueba para el administrador.
- `274A-52`: `POST /api/admin/procesos/benchmark` ejecuta un benchmark Rust sobre consultas reales del algoritmo.

## Arquitectura

- `src/handlers/admin_experiments.rs` expone handlers Axum delgados y schemas OpenAPI.
- `src/services/admin_experiments/mod.rs` contiene la logica de negocio: batching de embeddings, acciones de experimento y formato del benchmark.
- `src/repositories/admin_experiments.rs` concentra SQL parametrizado contra `samples`, `usuarios_ext` y pgvector.
- `src/audio/embeddings.rs` sigue siendo la fuente de verdad del vector 128d.

## Embeddings

- El batch procesa candidatos en lotes de `200`, igual que la estrategia legacy.
- `generar` filtra samples activos sin `embedding`.
- `regenerar` primero pone `embedding = NULL` para samples no eliminados y luego ejecuta el mismo batch.
- La entrada usa metadata real de `samples`: `bpm`, `key`, `escala`, `duracion`, `tipo`, `es_premium` y `tags`.
- La persistencia usa `pgvector::Vector` y actualiza `updated_at`.

## Experimentos sociales

- El endpoint acepta `acciones?: ["usuario", "notificacion", "mensaje"]`; si no se envia, ejecuta todas.
- El usuario fijo `alice_test` se crea/actualiza con `ON CONFLICT (username)` y `es_seed = TRUE`.
- La notificacion usa `NotificationService::create`, por lo que respeta deduplicacion y shape real del sistema.
- El mensaje usa `ConversationRepository` y `MessageRepository`, creando conversacion si no existia.

## Benchmark

- No shell-ejecuta el PHP legacy; mide rutas reales del backend Rust.
- Reporta en texto compatible con la UI legacy: samples activos, samples con embedding, feed publico y similares por pgvector.
- `perPage` se limita a `1..100` para evitar consultas admin excesivas.

## Gotchas

- SQLx offline impide usar macros nuevas sin regenerar `.sqlx`; estos repositorios usan queries runtime con `.bind()` y `sentinel-disable-file` justificado.
- Orval falla si utoipa genera referencias con nombres locales duplicados o rutas `crate::...`; se renombraron schemas admin duplicados y se registraron schemas referenciados.
- El registro central `handlers/mod.rs` mantiene `sentinel-disable-file limite-lineas directory-size` porque es el punto unico de OpenAPI/rutas.

## Validacion

- `cargo fmt --check` OK.
- `cargo check --message-format=short` OK.
- `cargo clippy --all-targets --message-format=short -- -D warnings` OK.
- `cargo test --all-targets` OK: 181 tests.
- `cargo run -- --emit-openapi openapi.json` OK.
- `npm run codegen` OK.
- `npm --prefix frontend run type-check` OK.
- `npm run audit:api` queda en 3 faltantes esperados: duplicados backfill, automatizacion reactivar y samples todos.

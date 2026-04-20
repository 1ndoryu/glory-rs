# Dashboard creador — 2026-04-19

## Endpoints
- `GET /api/dashboard/stats?usuarioId={id}` → `CreatorDashboardStats` (ingresos total/mes/anterior, descargas total/mes, reproducciones total/mes, seguidores total/nuevos del mes, samples publicados).
- `GET /api/dashboard/top-samples?usuarioId={id}` → `Vec<CreatorDashboardSampleStat>` (top 10 por descargas con `total_descargas`, `total_reproducciones` y `ingresos_generados`).
- `GET /api/dashboard/transacciones?usuarioId={id}&pagina={n}` → `Vec<CreatorDashboardTransaction>` (paginado fijo 20/pág, tipos `descarga|venta|suscripcion`).
- `GET /api/dashboard/ingresos?usuarioId={id}&periodo={semana|mes|anio}` → `Vec<CreatorDashboardIncomePoint>` con `fecha` (YYYY-MM-DD) e `ingresos` agregados por día.

Auth: bearer obligatorio (`CurrentUser`). El handler valida que `usuarioId` exista vía `BillingRepository::find_stripe_user_profile` → `404 Not Found` si no.

## Origen legacy
- `App/dashboard/Controllers/DashboardController.php`, `DashboardRepository`, `SamplesRepository::topSamplesCreador`, `TransaccionesRepository::ingresosDashboard|listarDelCreador|ingresosGrafico`.
- Payout queda fuera del alcance: en legacy solo era stub frontend sin backend real.

## Capa Rust
- Modelos: `src/models/dashboard/mod.rs` (subdirectorio porque `models/` ya estaba al límite de 10 archivos del Sentinel).
- Repositorio: `src/repositories/creator_dashboard.rs` con 4 métodos `async`:
  - `stats`: 1 query agregada con subselects sobre `usuarios_ext`, `samples`, `transacciones`, `descargas`, `reproducciones`, `follows`. Acepta tanto estado `completada` como `completed`.
  - `top_samples`: JOIN `samples` + `descargas` + `reproducciones` + `transacciones`, GROUP BY sample, LIMIT 10.
  - `transactions`: JOIN `transacciones` + `samples` + `usuarios_ext`, ORDER BY `created_at DESC`, LIMIT 20 OFFSET (`pagina-1`)*20. Mapea estado legacy `compra_sample` → `venta`.
  - `income_series`: GROUP BY `DATE(t.created_at)` filtrado por `make_interval(days => $2)`. **Gotcha**: `ORDER BY fecha` con alias rompía SQLx prepare en Postgres; usar `ORDER BY DATE(t.created_at) ASC`.
- Handlers: `src/handlers/dashboard.rs` con `#[utoipa::path]` taggeado `payments`. `routes()` se mergea en `api_routes()` de `src/handlers/mod.rs`.

## Validaciones ejecutadas
- `cargo sqlx prepare --workspace`
- `cargo check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --lib` (181 passed)
- `cargo run -- --emit-openapi openapi.json`
- `npm run codegen` + `npm --prefix frontend run type-check`
- `npm run self-check -- -TareaId 174A-90`

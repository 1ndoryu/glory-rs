# Handler `payments::list_plans` — Catálogo público de planes (174A-80) — 2026-04-19

## Alcance
- Tarea: `174A-80 — GET /pagos/planes`.
- Se añadió `GET /api/pagos/planes` como endpoint público para que frontend, desktop y mobile puedan leer el catálogo real desde backend en lugar de seguir con datos estáticos duplicados.

## Contrato devuelto
- Top-level:
  - `stripeHabilitado`
  - `publishableKey`
  - `moneda`
  - `planes`
- Cada plan expone:
  - `id`
  - `nombre`
  - `precioMensualCents`
  - `precioMensual`
  - `precioAnualCents`
  - `precioAnual`
  - `ahorroAnualCents`
  - `ahorroAnual`
  - `descargasDia`
  - `subidasMes`
  - `maxSamples`
  - `transferenciaGb`
  - `revenueShareBps`
  - `revenueShareLabel`
  - `priceIdConfigurado`
  - `pruebaGratuitaDias`
  - `descargasPrueba`

## Decisiones de diseño
- El catálogo se construye desde `src/domain/payments.rs`, no desde constantes duplicadas en el handler.
- Se exponen centavos y string decimal al mismo tiempo:
  - centavos para fuente de verdad
  - string decimal para UX rápida sin floats en backend
- El pricing anual replica la regla legacy usada por UI:
  - anual = 10 meses
  - ahorro = 2 meses
- El endpoint responde aunque Stripe esté deshabilitado. Eso evita que la ausencia de secrets rompa el render del catálogo en local o staging.

## Orden y compatibilidad
- El orden de respuesta quedó fijado en:
  - `free`
  - `pro`
  - `premium`
- Esto desacopla la salida pública del orden interno del catálogo de dominio.

## Validación ejecutada
- `cargo check`
- `cargo clippy -- -D warnings`
- `cargo test`

## Pendiente inmediato
- `174A-81`: reutilizar `StripeRuntime` y este catálogo para exponer checkout de suscripción, checkout individual y portal de facturación.
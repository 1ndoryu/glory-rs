# Handlers `payments::{create_subscription_checkout, create_sample_checkout, create_billing_portal}` — Checkout y portal (174A-81) — 2026-04-19

## Alcance
- Tarea: `174A-81 — Checkout suscripción + sample + portal`.
- Se añadieron tres endpoints autenticados:
  - `POST /api/pagos/checkout`
  - `POST /api/pagos/checkout-sample`
  - `POST /api/pagos/portal`

## Contratos nuevos
- `CreateSubscriptionCheckoutRequest`
  - `plan`: `free | pro | premium`
  - `periodo`: `mensual | anual` opcional
- `CreateSampleCheckoutRequest`
  - `sampleId`
- `PaymentRedirectResponse`
  - `ok`
  - `url`

## Reglas portadas
- Suscripción:
  - sólo permite `pro` y `premium`
  - `free` devuelve validación porque no requiere Stripe
  - `anual` queda bloqueado por ahora porque el runtime no tiene price ids separados por periodo y el contrato legacy efectivo todavía opera sobre price ids mensuales
- Compra individual:
  - el sample debe existir y estar activo
  - debe ser premium y tener `precio > 0`
  - el comprador no puede ser el creador
  - no se permite recomprar un sample ya comprado
- Portal:
  - exige `stripe_customer_id` persistido; si no existe, responde conflicto

## Decisiones de implementación
- Las URLs de éxito/cancelación se construyen desde `public_base_url` con fallback local.
- Se corrigió una sutileza en Stripe Checkout: si la URL de éxito ya contiene query string, `session_id` se concatena con `&`, no con `?`.
- El repositorio de billing ahora expone:
  - lookup mínimo del sample para checkout
  - verificación de compra individual completada

## Compatibilidad legacy
- Las rutas destino imitan la semántica PHP:
  - suscripciones vuelven a `/planes/`
  - compra individual vuelve a `/descargas/` o al detalle del sample
- El shape del flujo sigue siendo redirección a Stripe, no JSON expandido del recurso remoto.

## Validación ejecutada
- `cargo sqlx prepare --workspace`
- `cargo check`
- `cargo clippy -- -D warnings`
- `cargo test`

## Pendiente inmediato
- `174A-82`: cerrar el circuito con webhook firmado e idempotente para que checkout y portal actualicen estado local de forma persistente.
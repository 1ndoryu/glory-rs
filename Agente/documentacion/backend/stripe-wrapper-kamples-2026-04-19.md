# Service `stripe_service` — Wrapper Stripe Kamples (174A-79) — 2026-04-19

## Alcance
- Tarea: `174A-79 — Wrapper Stripe + planes Kamples`.
- Este corte no expone endpoints todavía. Deja lista la base para `174A-80` a `174A-84`.
- La integración queda modelada como runtime opcional, igual que push, FCM y SMTP: si no existe `GLORY_STRIPE_SECRET_KEY` o `STRIPE_SECRET_KEY`, Stripe queda deshabilitado sin romper el arranque.

## Configuración soportada
- Secret key:
  - `GLORY_STRIPE_SECRET_KEY`
  - `STRIPE_SECRET_KEY`
- Publishable key:
  - `GLORY_STRIPE_PUBLISHABLE_KEY`
  - `STRIPE_PUBLISHABLE_KEY`
- Webhook secret:
  - `GLORY_STRIPE_WEBHOOK_SECRET`
  - `STRIPE_WEBHOOK_SECRET`
- Connect webhook secret:
  - `GLORY_STRIPE_CONNECT_WEBHOOK_SECRET`
  - `STRIPE_CONNECT_WEBHOOK_SECRET`
- Price ids por plan:
  - `GLORY_STRIPE_PRICE_FREE` / `STRIPE_PRICE_FREE`
  - `GLORY_STRIPE_PRICE_PRO` / `STRIPE_PRICE_PRO`
  - `GLORY_STRIPE_PRICE_PREMIUM` / `STRIPE_PRICE_PREMIUM`

## Componentes introducidos
- `src/domain/payments.rs`
  - define `KamplesPlanId`, `KamplesPlanConfig` y `RevenueShareBreakdown`
  - centraliza el catálogo legacy:
    - `free`
    - `pro`
    - `premium`
  - expone `calculate_sample_revenue_share()` y `format_price_cents()`
- `src/repositories/billing.rs`
  - encapsula lecturas y escrituras de `stripe_customer_id`, `stripe_connect_id` y suscripciones activas
  - deja el acceso SQL listo para checkout, portal y webhooks sin repetir queries en handlers
- `src/services/stripe_service.rs`
  - crea `StripeRuntime` a partir de config
  - expone operaciones de alto nivel:
    - get or create customer
    - checkout de suscripción
    - checkout de compra individual
    - billing portal
    - Connect account, onboarding link y login link
    - verificación de webhooks por secret

## Decisiones de implementación
- Se usó `async-stripe 1.0.0-rc.5` y subcrates tipados en lugar de HTTP manual.
- El runtime usa rustls y requiere habilitar un crypto provider explícito. En este repo quedó fijado `rustls-aws-lc-rs`.
- Los flags de estado de cuentas Connect se modelan como `Option<bool>` porque Stripe puede omitirlos según el estado del recurso.
- El formateo monetario se hace desde centavos enteros. No se usan conversiones a `f64` para evitar pérdida de precisión y warnings de Clippy.

## Integración con la app
- `AppConfig` ahora agrupa Stripe bajo `config.stripe`.
- `AppState` ahora transporta `stripe_runtime` como `Option<Arc<_>>`.
- `src/handlers/mod.rs` introduce `AppRuntimes` para no seguir inflando la firma de `create_router()` con runtimes opcionales.
- `src/main.rs` inicializa Stripe, registra si hay publishable key y price ids configurados, y sigue arrancando aunque Stripe esté deshabilitado.

## SQLx offline
- Se añadieron queries nuevas en `.sqlx/` para:
  - cargar perfil Stripe del usuario
  - persistir `stripe_customer_id`
  - persistir `stripe_connect_id`
  - leer suscripción activa
  - upsert de suscripción por `stripe_subscription_id`
- Recordatorio operativo: cualquier cambio futuro en estas queries exige volver a correr `cargo sqlx prepare --workspace`.

## Validación ejecutada
- `cargo sqlx prepare --workspace`
- `cargo check`
- `cargo clippy -- -D warnings`
- `cargo test`

## Pendiente para siguientes tareas
- `174A-80`: exponer catálogo de planes vía handler
- `174A-81`: conectar checkout suscripción/sample/portal a endpoints
- `174A-82`: persistencia idempotente de webhooks y auditoría
- `174A-83`: persistencia completa de Connect y revenue share operativo
- `174A-84`: códigos gratis y su interacción con límites/descargas
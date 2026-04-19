# Handler `payments::webhook::payment_webhook` — Webhook Stripe con idempotencia (174A-82) — 2026-04-19

## Alcance
- Tarea: `174A-82 — Webhook con HMAC + idempotencia`.
- Se añadió `POST /api/pagos/webhook` como endpoint sin auth para Stripe Payments.

## Eventos manejados
- `checkout.session.completed`
  - distingue suscripción vs compra de sample por `metadata.tipo`
- `customer.subscription.updated`
  - sincroniza plan del usuario y estado en `suscripciones`
- `customer.subscription.deleted`
  - degrada a `free` y marca la suscripción como cancelada

## Verificación e idempotencia
- La firma se valida con `StripeRuntime::verify_webhook(...)` y `StripeWebhookSecretKind::Payments`.
- La respuesta del evento queda cacheada por `event.id` usando `IdempotencyStore`:
  - namespace: `stripe-webhook`
  - TTL: 7 días
- Para compras individuales también se aplica idempotencia de persistencia:
  - `stripe_payment_id` sigue protegido por índice único en `transacciones`
  - además se persiste `idempotency_key = stripe-webhook-{event_id}`

## Persistencia introducida
- `BillingRepository` ahora soporta:
  - lookup de usuario por `stripe_customer_id`
  - actualización directa de `usuarios_ext.plan`
  - inserción idempotente de compra individual completada
- La compra individual valida contra BD antes de persistir:
  - sample existente
  - creador correcto
  - precio metadata = precio real del sample

## Impacto en descargas
- `handlers/downloads.rs` dejó de tratar todos los samples de pago como inaccesibles para usuarios free.
- Si existe una compra completada en `transacciones`, la descarga:
  - se permite
  - no consume crédito
  - mantiene el resto del flujo intacto

## Decisiones de diseño
- La lógica del webhook vive en `src/handlers/payments/webhook.rs` porque `src/handlers/payments.rs` superó el límite interno de 500 líneas para controladores.
- Para evitar activar soporte `bigdecimal` sólo por SQLx, la inserción de montos `NUMERIC` convierte centavos a string decimal antes del `INSERT`.
- Los eventos desconocidos responden `200` con `procesado=false`, evitando ruido y reintentos innecesarios.

## Validación ejecutada
- `cargo sqlx prepare --workspace`
- `cargo check`
- `cargo clippy -- -D warnings`
- `cargo test`

## Pendiente inmediato
- `174A-83`: usar la misma base Stripe para cerrar onboarding Connect y exponer revenue share operativo al creador.
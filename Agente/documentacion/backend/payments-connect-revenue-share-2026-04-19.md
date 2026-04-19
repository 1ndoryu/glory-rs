# Handlers `connect::*` + revenue share por descarga (174A-83) — 2026-04-19

## Alcance
- Tarea: `174A-83 — Connect onboarding + revenue share`.
- Se portó la superficie legacy de Stripe Connect bajo `/api/connect/*`.
- Se completó también el registro de revenue share por descarga de suscripción en `transacciones`.

## Endpoints añadidos
- `POST /api/connect/onboarding`
  - crea o reutiliza cuenta Express
  - promueve `usuarios_ext.rol` de `usuario` a `creador`
  - devuelve URL de onboarding con `return_url` / `refresh_url` legacy
- `GET /api/connect/estado`
  - responde el shape esperado por el frontend legado:
    - `estado`
    - `connectId`
    - `cargosActivos`
    - `payoutsActivos`
    - `detalle`
    - `requerimientosPendientes`
- `POST /api/connect/dashboard`
  - genera login link al Express Dashboard
- `GET /api/connect/balance`
  - consulta fondos de la cuenta conectada con header `Stripe-Account`

## Diseño técnico
- Se creó `src/handlers/connect.rs` para no seguir creciendo `payments.rs`.
- `StripeService` ahora expone:
  - `retrieve_connect_balance(...)`
  - `StripeConnectBalanceSummary`
  - más contexto en `StripeConnectAccountSummary` (`currently_due`, `disabled_reason`)
- El estado semántico se deriva así:
  - `activo`: charges + payouts habilitados
  - `pendiente`: hay `currently_due` o `details_submitted=false`
  - `restringido`: no está activo y Stripe ya no reporta onboarding pendiente
  - `error`: fallo consultando Stripe o runtime ausente

## Revenue share portado
- `domain::payments` añade `calculate_subscription_download_revenue_share(plan)`.
- Modelo legacy replicado:
  - base = `precio_mensual / 200`
  - luego se reparte según `revenue_share_bps`
- Persistencia nueva en `BillingRepository`:
  - `insert_completed_download_revenue_share(...)`
- Flujos conectados:
  - `POST /api/samples/:id/descargar`
  - `POST /api/colecciones/:id/descargar-zip`

## Notas de compatibilidad
- Para balance Connect hubo que habilitar el feature `balance` de `async-stripe-core`.
- El request usa `RetrieveForMyAccountBalance::new().customize().account_id(...)`.
- Los montos siguen yendo a `NUMERIC` vía centavos convertidos a string decimal, manteniendo la estrategia adoptada en 174A-82 para evitar introducir `bigdecimal`.

## Validación ejecutada
- `cargo sqlx prepare --workspace`
- `cargo check`
- `cargo clippy -- -D warnings`
- `cargo test`

## Pendiente inmediato
- `174A-84`: CRUD + uso de códigos gratis sobre el flujo de descargas ya portado.
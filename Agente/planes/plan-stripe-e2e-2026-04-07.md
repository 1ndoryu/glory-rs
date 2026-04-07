# Plan: Stripe End-to-End — 064A-59

Fecha: 2026-04-07
Estado: COMPLETADO (plan creado + email fix implementado)

## Resumen del estado actual

El flujo de pago Stripe está funcional pero incompleto:
- ✅ PaymentIntent creation con capture manual (escrow)
- ✅ Webhook verification (HMAC-SHA256) + deduplicación (064A-73)
- ✅ 3 modos de pago definidos (full, half_half, phased) con descuentos
- ✅ State machine post-pago (avance de fases según modo)
- ✅ Email pre-llenado en Stripe via receipt_email (064A-59)
- ❌ Payment mode hardcodeado a "full" en frontend
- ❌ Sin UI para pagos por fase (phased mode)
- ❌ Sin flujo de captura manual (webhooks de captura no implementados)
- ❌ Sin refund automation completa

## Pendientes clasificados por prioridad

### P1 — Selección de modo de pago (064A-60)
**Problema:** `useModalCompra.ts` línea ~36 tiene `payment_mode: 'full'` hardcodeado.
**Fix:**
1. Modal de compra debe mostrar 3 opciones con sus descuentos:
   - Pago completo (20% desc)
   - 50/50 (10% desc)
   - Por fases (sin desc)
2. Conectar la selección al `apiCreateOrder({ payment_mode: selected })`.
3. `OrdenDetalle.tsx` debe adaptar UI según modo:
   - Full: botón "Pagar todo" en header
   - HalfHalf: botón "Pagar primera mitad", luego "Pagar segunda mitad"
   - Phased: cada FaseCard tiene su propio botón "Pagar fase"
**Archivos:** `frontend/src/hooks/useModalCompra.ts`, `frontend/src/components/panel/OrdenDetalle.tsx`, `frontend/src/components/panel/FaseCard.tsx`

### P2 — CheckoutModal para pagos parciales (fases)
**Problema:** `CheckoutModal` se abre solo para orden completa. Phased mode necesita pago por fase individual.
**Fix:**
1. `OrdenDetalle` ya pasa `phaseNumber` como prop al modal. Verificar que `FaseCard` lo use.
2. El backend ya soporta `phase_number` en `initiate_payment`. Solo falta conectar el frontend.
**Archivos:** `frontend/src/components/panel/FaseCard.tsx`, `frontend/src/components/panel/CheckoutModal.tsx`

### P3 — Captura de fondos (escrow → release)
**Problema:** Los PaymentIntents se crean con `capture_method: manual` pero no hay flujo de captura.
**Estado actual:** `capture_stripe_intent()` existe en el backend pero solo se llama desde `process_held_payments()`, que se ejecuta en el webhook de `payment_intent.succeeded`.
**Verificar:**
- ¿La captura se dispara al completar la orden o al entregar?
- ¿Qué pasa si el admin cancela una orden con fondos held?
- ¿Los refunds manejan correctamente fondos held vs captured?
**Archivos:** `src/services/payment.rs` (líneas 360-430)

### P4 — Testing end-to-end con Stripe CLI
**Pendiente:** Configurar `stripe listen --forward-to localhost:PORT/api/webhooks/stripe` para testing local con webhooks reales.
**Checks necesarios:**
1. Crear orden → pagar → webhook received → status updated
2. Pago phased → pagar fase 1 → webhook → fase avanza
3. Refund → webhook → fondos devueltos → estado actualizado
4. Doble webhook (dedup funciona)
5. Webhook con firma inválida → rejected

### P5 — Stripe Dashboard config
**Pendiente:** Verificar que el Stripe Dashboard NO tenga habilitada la opción de pedir email en PaymentElement (ahora que se envía receipt_email desde backend).

## Cambios implementados en esta tarea

1. **Backend:** `receipt_email` enviado a Stripe al crear PaymentIntent (del user autenticado)
2. **Archivos:** `src/handlers/payments.rs`, `src/services/payment.rs`
3. **Resultado:** El checkout modal de Stripe ya no pide email — usa el del usuario logueado

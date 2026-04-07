# Plan de Auditoría de Seguridad — 064A-73

Fecha: 2026-04-07
Estado: COMPLETADO

## Resumen

Auditoría completa del backend Axum + frontend React. Hallazgos clasificados por severidad.

---

## Etapa 1: CRÍTICOS (bloquean producción)

### 1.1 CORS abierto a cualquier origen
- **Archivo:** `src/handlers/mod.rs`
- **Fix:** CORS condicional con `GLORY_ALLOWED_ORIGINS` env var + allow_credentials
- **Estado:** ✅ Completado

### 1.2 Sin rate limiting en ningún endpoint
- **Archivos:** `src/handlers/mod.rs`, `Cargo.toml`
- **Fix:** `tower_governor` — auth: 5 req/min por IP, API general: 120 req/min por IP
- **Estado:** ✅ Completado

### 1.3 Webhook Stripe: sin deduplicación por event_id
- **Archivo:** `src/handlers/payments.rs`, migración `20260407002000_stripe_event_dedup`
- **Fix:** Tabla `stripe_processed_events` + check antes de procesar + tolerancia reducida a 120s
- **Estado:** ✅ Completado

### 1.4 WS staff: token en query param
- **Nota:** Limitación de la API WebSocket del navegador — no soporta headers custom.
- **Estado:** ⏭ Diferido (requiere rediseño de auth WS a cookie-based o first-message)

## Etapa 2: ALTOS

### 2.1 File uploads: sin validación de magic bytes
- **Archivo:** `src/handlers/deliverables.rs`
- **Fix:** `validate_magic_bytes()` verifica firma binaria vs MIME declarado para 13 tipos
- **Estado:** ✅ Completado

### 2.2 Path traversal en descarga de archivos
- **Archivo:** `src/handlers/deliverables.rs`
- **Fix:** `canonicalize()` + `starts_with(base_dir)` antes de servir archivo
- **Estado:** ✅ Completado

### 2.3 Sin HTTPS/HSTS enforcement
- **Archivo:** `src/handlers/mod.rs`
- **Fix:** Headers HSTS + X-Content-Type-Options + X-Frame-Options
- **Estado:** ✅ Completado (fusionado con 4.1)

### 2.4 Staff WS ve TODAS las sesiones
- **Estado:** ☐ No aplica (equipo pequeño, intencional)

## Etapa 3: MEDIOS

### 3.1 Sin audit logging de eventos de seguridad
- **Archivos:** `src/services/audit.rs`, `src/handlers/auth.rs`, `src/handlers/admin_users.rs`, `src/handlers/payments.rs`, migración `20260407003000_audit_log`
- **Fix:** Tabla `audit_log` + `AuditService::log()` fire-and-forget + instrumentado en login, role change, webhook
- **Estado:** ✅ Completado

### 3.2 Sin CSRF tokens en mutaciones
- **Estado:** ☐ No aplica (JWT stateless, no cookies)

### 3.3 AI service config sin whitelist de modelos
- **Archivo:** `src/services/ai_chat.rs`
- **Fix:** Whitelist de 5 modelos Groq permitidos, fallback a default si no está en lista
- **Estado:** ✅ Completado

### 3.4 Password change no forzado para quick-register
- **Estado:** ⏭ Diferido (requiere flag en users + middleware + flujo frontend)

## Etapa 4: BAJOS

### 4.1 OWASP security headers faltantes
- **Fix:** Fusionado con 2.3 — HSTS, nosniff, frame deny
- **Estado:** ✅ Completado

### 4.2 JWT sin refresh tokens
- **Estado:** ⏭ Diferido (requiere rediseño de flujo auth frontend completo)

### 4.3 Logging de uploads
- **Archivo:** `src/handlers/deliverables.rs`
- **Fix:** `tracing::info!` con user_id, filename, mime, size
- **Estado:** ✅ Completado

### 4.4 Stripe response body silenciado en errores
- **Archivo:** `src/services/payment.rs`
- **Fix:** `tracing::error!` en create_payment_intent, capture, cancel y refund
- **Estado:** ✅ Completado

---

## Resumen de ejecución

| Item | Estado |
|------|--------|
| 1.1 CORS | ✅ |
| 1.2 Rate limiting | ✅ |
| 1.3 Webhook dedup | ✅ |
| 1.4 WS token | ⏭ Diferido |
| 2.1 Magic bytes | ✅ |
| 2.2 Path traversal | ✅ |
| 2.3 HSTS | ✅ |
| 3.1 Audit log | ✅ |
| 3.3 AI whitelist | ✅ |
| 3.4 Password change | ⏭ Diferido |
| 4.1 Headers | ✅ |
| 4.2 JWT refresh | ⏭ Diferido |
| 4.3 Upload logging | ✅ |
| 4.4 Stripe logging | ✅ |

**11/14 implementados, 3 diferidos (requieren rediseño complejo de flujo)**

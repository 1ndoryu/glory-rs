# Plan: Revisión Profunda del Sistema de Marketing — 2026-03-30

## Contexto

El sistema de marketing tiene 3 flujos de comunicación: campañas (email/SMS/WhatsApp), plantillas WhatsApp (aprobación Meta), y recordatorios automáticos. La investigación revela bugs críticos, stubs sin implementar, y problemas de datos que impedirían el funcionamiento real en producción.

## Bugs Críticos Encontrados

### BUG-1: Meta WhatsApp usa endpoint INCORRECTO (CRÍTICO)
- **Archivo**: `src/services/meta_whatsapp.rs`
- **Problema**: Usa `https://graph.facebook.com/v21.0/{waba_id}/messages` pero la API real de Meta usa `POST /{Version}/{Phone-Number-ID}/messages`. El WABA ID sirve para crear templates, NO para enviar mensajes. Se necesita un `phone_number_id` separado.
- **Fix**: Agregar campo `meta_phone_number_id` a `integraciones_marketing` y usarlo en `enviar_mensaje()`.

### BUG-2: Plantilla WhatsApp enviar_a_meta es STUB (CRÍTICO)
- **Archivo**: `src/services/plantilla_whatsapp.rs`
- **Problema**: `enviar_a_meta()` genera un ID falso (`meta_stub_{uuid}`) en lugar de hacer POST real a `POST /{WABA_ID}/message_templates`.
- **Fix**: Implementar llamada real a Meta Business API usando `meta_waba_id` + `meta_access_token`.

### BUG-3: Contadores de campaña nunca se actualizan (MEDIO)
- **Archivo**: `src/services/campana.rs` + `src/repositories/campana.rs`
- **Problema**: `enviar_por_canales()` cuenta `enviados` y `errores` pero no devuelve nada. `set_estado()` solo actualiza `total_destinatarios`, no `total_enviados`/`total_fallidos`.
- **Fix**: Hacer que `enviar_por_canales` retorne `(u32, u32)` y actualizar `set_estado` para incluir ambos contadores.

### BUG-4: Estado de destinatarios nunca se actualiza (MEDIO)
- **Archivo**: `src/repositories/campana.rs`
- **Problema**: Los registros en `campana_destinatarios` quedan siempre como "pendiente". No hay método para marcarlos como "enviado"/"fallido".
- **Fix**: Crear `actualizar_estado_destinatario()` y llamarlo dentro de `enviar_por_canales`.

### BUG-5: Recordatorio sin prefijo telefónico (MEDIO)
- **Archivo**: `src/repositories/recordatorio.rs` + `src/services/recordatorio.rs`
- **Problema**: La query trae `r.telefono` de reservas (sin prefijo). SMS/WhatsApp requieren E.164 (`+34612345678`). En campañas sí se compone `prefijo + telefono`.
- **Fix**: Unir con `clientes` para obtener `prefijo_telefono` y componer el número completo.

### BUG-6: API de Meta versión desactualizada (BAJO)
- Usamos v21.0, actual es v23.0. Actualizar en ambos servicios.

## Mejoras Necesarias

### MEJORA-1: Validación E.164 en números de teléfono
- Twilio y Meta rechazan números mal formados. Agregar validación básica.

### MEJORA-2: .env.example incompleto
- Falta documentar SMTP_*, ERROR_REPORT_EMAIL en `.env.example`.

### MEJORA-3: Validación de email en smtp_from_email
- Un email malformado crashea `lettre` en runtime.

### MEJORA-4: Campo meta_phone_number_id en integraciones
- Nueva migración + campo en modelo + campo en formulario frontend.

## Fases de Implementación

### Fase 1 — Migración BD + Modelo (meta_phone_number_id)
1. Crear migración para agregar `meta_phone_number_id` a `integraciones_marketing`
2. Actualizar struct `IntegracionMarketing`
3. Actualizar `ActualizarIntegracionesRequest`
4. Actualizar `IntegracionMarketingPublica`
5. Actualizar repositorio `actualizar()`

### Fase 2 — Fix Meta WhatsApp Service (BUG-1 + BUG-6)
1. Cambiar endpoint de `{waba_id}/messages` a `{phone_number_id}/messages`
2. Actualizar versión de v21.0 a v23.0
3. Actualizar `meta_configurado()` para requerir `meta_phone_number_id`

### Fase 3 — Implementar enviar_a_meta real (BUG-2)
1. Reemplazar stub por POST real a Meta Business API
2. Usar `meta_waba_id` para crear templates (correcto)
3. Parsear respuesta y guardar `meta_template_id` real

### Fase 4 — Fix contadores de campaña (BUG-3 + BUG-4)
1. Modificar `enviar_por_canales` para retornar `(u32, u32)`
2. Actualizar `set_estado` para recibir `total_enviados` y `total_fallidos`
3. Crear método `actualizar_estado_destinatario` en repositorio
4. Llamar actualización por destinatario dentro del loop

### Fase 5 — Fix recordatorio prefijo telefónico (BUG-5)
1. Agregar `prefijo_telefono` a struct `ReservaPendienteRecordatorio`
2. Modificar query SQL para traer `c.prefijo_telefono`
3. Componer número en `enviar_recordatorio()`

### Fase 6 — Validaciones y .env.example (MEJORA-1,2,3)
1. Agregar validación E.164 helper
2. Validar `smtp_from_email` como email válido
3. Actualizar `.env.example`

### Fase 7 — Frontend (meta_phone_number_id)
1. Agregar campo en IntegracionesMarketing.tsx
2. Actualizar hook useIntegraciones

### Fase 8 — Validar todo compila, tests, deploy

## Archivos Afectados
- `migrations/` — nueva migración
- `src/models/integracion_marketing.rs`
- `src/repositories/integracion_marketing.rs`
- `src/repositories/campana.rs`
- `src/repositories/recordatorio.rs`
- `src/services/meta_whatsapp.rs`
- `src/services/plantilla_whatsapp.rs`
- `src/services/campana.rs`
- `src/services/recordatorio.rs`
- `frontend/src/componentes/IntegracionesMarketing.tsx`
- `.env.example`

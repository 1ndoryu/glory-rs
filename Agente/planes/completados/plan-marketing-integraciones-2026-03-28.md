# Plan: Marketing — Completar integraciones

## Contexto
Auditoría 283A-21 detectó que el sistema está ~40-50% funcional: CRUD y scheduling funcionan, pero envío real es stub.

## Fases

### Fase 1: BD + Backend config (integraciones_marketing)
- Migración: tabla `integraciones_marketing` (user_id, smtp_*, twilio_*, meta_*, timestamps)
- Model + repo + service + handler CRUD
- Endpoint GET/PUT `/api/configuracion/integraciones`

### Fase 2: Email de campañas
- `services/campana.rs` → `enviar_campana()` ya genera destinatarios. Conectar al servicio SMTP existente para canal `email`.
- Necesita: subject, body HTML de la plantilla, destinatarios con email.

### Fase 3: Recordatorios activos
- `services/recordatorio.rs` → `enviar_recordatorio()` debe llamar SMTP real para enviar email con el texto de la plantilla.
- Respetar el canal de la regla (email only por ahora, SMS cuando Twilio esté listo).

### Fase 4: Twilio SMS
- Crear `services/twilio.rs` con HTTP client para `api.twilio.com/2010-04-01/Accounts/{SID}/Messages.json`
- Integrar en `enviar_campana()` para canal SMS y en `enviar_recordatorio()` para reglas tipo SMS.

### Fase 5: Meta WhatsApp API
- Crear `services/meta_whatsapp.rs` para POST a `graph.facebook.com/v21.0/{WABA_ID}/messages`
- Integrar en `enviar_a_meta()` real y en `enviar_campana()` para canal WhatsApp.
- Webhook opcional (fase futura).

### Fase 6: UI Configuración
- Nueva sección en Configuración.tsx: formulario de credentials (SMTP, Twilio, Meta)
- Usar endpoint de Fase 1.

## Estado actual
- [x] Fase 1 — BD + Backend config + endpoints
- [x] Fase 2 — Email campañas via SMTP real
- [x] Fase 3 — Recordatorios email via SMTP real
- [x] Fase 4 — Twilio SMS (servicio + integrado en campañas y recordatorios)
- [x] Fase 5 — Meta WhatsApp (servicio + integrado en campañas y recordatorios)
- [x] Fase 6 — UI Configuración (tabs General + Integraciones con formularios SMTP/Twilio/Meta)

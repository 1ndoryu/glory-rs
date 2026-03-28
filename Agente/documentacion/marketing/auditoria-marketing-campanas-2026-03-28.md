# Auditoría: Marketing y Campañas — 2026-03-28

## Estado general: ~40-50% funcional

El sistema de marketing está bien arquitecturado pero depende de integraciones de terceros que son stubs.

## Qué funciona end-to-end

| Componente | Estado |
|---|---|
| CRUD campañas + segmentación + preview | ✅ Completo |
| CRUD plantillas WhatsApp | ✅ Completo |
| CRUD reglas recordatorios | ✅ Completo |
| Scheduler recordatorios (cada 60s en main.rs) | ✅ Funcionando |
| Historial de envíos | ✅ Se registra |
| Consentimiento GDPR (email/sms flags) | ✅ Implementado |
| Generación de destinatarios por segmento | ✅ Completo |
| Email password reset via SMTP | ✅ Completo (si SMTP configurado) |

## Qué es STUB (no envía realmente)

### 1. Envío de campañas (`services/campana.rs`)
- `enviar_campana()` genera destinatarios pero solo loguea "envío pendiente de integración"
- No llama a ningún proveedor de SMS, email ni WhatsApp

### 2. Envío a Meta (`services/plantilla_whatsapp.rs`)
- `enviar_a_meta()` genera ID falso `meta_stub_{uuid}` y marca como "enviada"
- No hace POST a `graph.facebook.com/v21.0/{WABA_ID}/message_templates`

### 3. Recordatorios (`services/recordatorio.rs`)
- `enviar_recordatorio()` loguea y retorna `Ok(())` sin enviar nada
- El historial registra como "enviado" aunque sea simulado

## Configuración requerida para producción

### Variables de entorno (.env)

```env
# SMTP (para emails de campaña + recordatorios + recovery)
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=tu-email@gmail.com
SMTP_PASSWORD=tu-app-password
SMTP_FROM_EMAIL=noreply@restaurante.com
SMTP_FROM_NAME=Restaurante

# Twilio (para SMS de campañas + recordatorios)
TWILIO_ACCOUNT_SID=ACxxxxxx
TWILIO_AUTH_TOKEN=xxxxxxxxx
TWILIO_FROM_NUMBER=+12125551234

# Meta WhatsApp Business API
META_WABA_ID=1234567890
META_BUSINESS_APP_ID=xxxx
META_PERMANENT_ACCESS_TOKEN=EAABs...
```

### BD: tabla `integraciones_marketing` (por crear)

Almacenar credentials de terceros por usuario para que cada instalación tenga sus propias keys:
- `twilio_account_sid`, `twilio_auth_token`, `twilio_from_number`
- `meta_waba_id`, `meta_business_app_id`, `meta_access_token`
- `sms_gateway_provider`, `sms_gateway_api_key`

### Frontend: falta sección en Configuracion.tsx

- Campo para credentials de Twilio
- Campo para credentials de Meta WhatsApp
- Campo para SMS Gateway

## Tareas necesarias para completar el sistema

1. **Email de campañas** (~2h): Conectar `services/email.rs` al flujo de `enviar_campana()` para el canal email. El servicio SMTP ya existe, solo falta el wiring.
2. **SMS via Twilio** (~8h): Crear cliente HTTP para Twilio REST API, handler de envío, config en BD.
3. **WhatsApp via Meta** (~16h): Implementar POST real a Meta API, webhook para cambios de estado, polling.
4. **Recordatorios activos** (~6h): Conectar `enviar_recordatorio()` al servicio de email/SMS real según el canal de la regla.
5. **UI de credentials** (~3h): Nueva sección en Configuracion.tsx + migración + endpoint.

## Otros huecos detectados (fuera de marketing)

- **Sin `.env.example`**: No hay archivo de referencia para las variables de entorno necesarias.
- **Sin webhook de Meta**: Si un template es rechazado, no hay forma de saberlo automáticamente.
- **Sin métricas de envío real**: Los destinatarios se generan pero no se actualiza su estado a "entregado"/"fallido" con data real del proveedor.

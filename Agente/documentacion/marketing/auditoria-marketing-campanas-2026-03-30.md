# Auditoría: Marketing y Campañas — 2026-03-30

## Estado general: ~85% funcional (actualizado 303A-1)

El sistema de marketing está completamente cableado a proveedores reales.
Solo requiere configurar credenciales de terceros (SMTP, Twilio, Meta) para funcionar en producción.

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
| **Envío campañas Email** (SMTP) | ✅ Funciona si SMTP configurado |
| **Envío campañas SMS** (Twilio) | ✅ Funciona si Twilio configurado |
| **Envío campañas WhatsApp** (Meta) | ✅ Funciona si Meta configurado |
| **Recordatorios Email/SMS/WhatsApp** | ✅ Funciona según canal y proveedor configurado |
| **Enviar plantilla a Meta** | ✅ POST real a Meta Business API v23.0 |
| **Contadores de campaña** (enviados/fallidos) | ✅ 303A-1: actualizados en tiempo real |
| **Estado por destinatario** | ✅ 303A-1: marcado como enviado/fallido |
| **Números E.164 en recordatorios** | ✅ 303A-1: prefijo + teléfono |
| **UI de credenciales** | ✅ Configuración > Integraciones (con Phone Number ID) |

## Configuración requerida para producción

### Credenciales por usuario (Configuración > Integraciones en la UI)

Cada restaurante/usuario configura sus propias credenciales desde la interfaz web:
- **SMTP**: host, port, user, password, from_email, from_name
- **Twilio**: account_sid, auth_token, from_number
- **Meta WhatsApp**: waba_id, business_app_id, access_token, **phone_number_id** (requerido para enviar mensajes)

### Variables de entorno del servidor (.env)

Solo se necesitan para el SMTP del servidor (reset password, errores admin):
```env
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=tu-email@gmail.com
SMTP_PASSWORD=tu-app-password
SMTP_FROM_EMAIL=noreply@restaurante.com
SMTP_FROM_NAME=Restaurante
ERROR_REPORT_EMAIL=admin@restaurante.com
```

## Pendiente / Futuro

| Feature | Estado | Notas |
|---------|--------|-------|
| Meta Webhook de estado de templates | No implementado | Las plantillas cambian de estado (APPROVED/REJECTED) vía webhook |
| Twilio Delivery Reports (DLR) | No implementado | Twilio puede enviar callbacks de estado |
| Rate limiting / queue para envíos masivos | No implementado | Riesgo de saturar SMTP/Twilio con muchos envíos simultáneos |
| Métricas de marketing (conversión) | No implementado | Fase 4e del roadmap |
| A/B Testing campañas | No implementado | Beta |
| Retry automático de envíos fallidos | No implementado | Los fallidos se registran pero no se reintentan |

- **Sin `.env.example`**: No hay archivo de referencia para las variables de entorno necesarias.
- **Sin webhook de Meta**: Si un template es rechazado, no hay forma de saberlo automáticamente.
- **Sin métricas de envío real**: Los destinatarios se generan pero no se actualiza su estado a "entregado"/"fallido" con data real del proveedor.

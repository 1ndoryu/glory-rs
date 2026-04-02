/* [283A-23] Servicio de WhatsApp via Meta Cloud API (Graph API v23.0).
 * Envía mensajes de texto libre usando las credenciales de integraciones_marketing.
 * Retorna Ok(false) si Meta no está configurado en la integración.
 * [303A-1] Corregido: el endpoint usa Phone-Number-ID, no WABA ID.
 * POST /{Version}/{Phone-Number-ID}/messages */

use crate::models::IntegracionMarketing;

pub struct MetaWhatsappService;

#[derive(Debug, thiserror::Error)]
pub enum MetaWhatsappError {
    #[error("Error HTTP Meta: {0}")]
    Http(String),
    #[error("Meta respondió con error: {status} — {body}")]
    Api { status: u16, body: String },
}

impl MetaWhatsappService {
    /// Envía un mensaje de texto por `WhatsApp` via Meta Cloud API.
    /// Retorna `Ok(false)` si no está configurado.
    pub async fn enviar_mensaje(
        integ: &IntegracionMarketing,
        destinatario: &str,
        cuerpo: &str,
    ) -> Result<bool, MetaWhatsappError> {
        if !integ.meta_configurado() {
            tracing::warn!("Meta WhatsApp no configurado — mensaje a {} omitido", destinatario);
            return Ok(false);
        }

        /* [303A-1] La API de mensajes usa Phone-Number-ID, no WABA ID.
         * WABA ID se usa para gestión de templates (crear/listar/eliminar). */
        let phone_number_id = integ.meta_phone_number_id.as_deref().unwrap_or_default();
        let access_token = integ.meta_access_token.as_deref().unwrap_or_default();

        let url = format!("https://graph.facebook.com/v23.0/{phone_number_id}/messages");
        let payload = serde_json::json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "to": destinatario,
            "type": "text",
            "text": { "body": cuerpo }
        });

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .bearer_auth(access_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| MetaWhatsappError::Http(e.to_string()))?;

        let status = resp.status().as_u16();
        if status >= 400 {
            let body = resp.text().await.unwrap_or_default();
            tracing::error!("Meta WhatsApp error {status}: {body}");
            return Err(MetaWhatsappError::Api { status, body });
        }

        tracing::info!("WhatsApp enviado a {} via Meta", destinatario);
        Ok(true)
    }

    /* [024A-1] Envía un mensaje usando una plantilla aprobada por Meta (Template Message API).
     * Las plantillas aprobadas se envían con type=template + nombre + idioma.
     * Meta renderiza el contenido de la plantilla registrada, no necesitamos enviar el body. */
    pub async fn enviar_template(
        integ: &IntegracionMarketing,
        destinatario: &str,
        template_nombre: &str,
        template_idioma: &str,
    ) -> Result<bool, MetaWhatsappError> {
        if !integ.meta_configurado() {
            tracing::warn!("Meta WhatsApp no configurado — template a {} omitido", destinatario);
            return Ok(false);
        }

        let phone_number_id = integ.meta_phone_number_id.as_deref().unwrap_or_default();
        let access_token = integ.meta_access_token.as_deref().unwrap_or_default();

        let url = format!("https://graph.facebook.com/v23.0/{phone_number_id}/messages");
        let payload = serde_json::json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "to": destinatario,
            "type": "template",
            "template": {
                "name": template_nombre,
                "language": { "code": template_idioma }
            }
        });

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .bearer_auth(access_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| MetaWhatsappError::Http(e.to_string()))?;

        let status = resp.status().as_u16();
        if status >= 400 {
            let body = resp.text().await.unwrap_or_default();
            tracing::error!("Meta WhatsApp template error {status}: {body}");
            return Err(MetaWhatsappError::Api { status, body });
        }

        tracing::info!("WhatsApp template '{}' enviado a {} via Meta", template_nombre, destinatario);
        Ok(true)
    }
}

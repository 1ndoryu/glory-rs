/* [283A-23] Servicio de WhatsApp via Meta Cloud API (Graph API v21.0).
 * Envía mensajes de texto libre usando las credenciales de integraciones_marketing.
 * Retorna Ok(false) si Meta no está configurado en la integración. */

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

        let waba_id = integ.meta_waba_id.as_deref().unwrap_or_default();
        let access_token = integ.meta_access_token.as_deref().unwrap_or_default();

        /* Meta Cloud API: enviar mensaje de texto libre */
        let url = format!("https://graph.facebook.com/v21.0/{waba_id}/messages");
        let payload = serde_json::json!({
            "messaging_product": "whatsapp",
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
}

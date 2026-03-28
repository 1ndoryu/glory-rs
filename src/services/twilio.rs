/* [283A-23] Servicio de SMS via Twilio REST API.
 * Envía SMS usando las credenciales almacenadas en integraciones_marketing.
 * Retorna Ok(false) si Twilio no está configurado en la integración. */

use crate::models::IntegracionMarketing;

pub struct TwilioService;

#[derive(Debug, thiserror::Error)]
pub enum TwilioError {
    #[error("Error HTTP Twilio: {0}")]
    Http(String),
    #[error("Twilio respondió con error: {status} — {body}")]
    Api { status: u16, body: String },
}

impl TwilioService {
    /// Envía un SMS via Twilio. Retorna `Ok(false)` si no está configurado.
    pub async fn enviar_sms(
        integ: &IntegracionMarketing,
        destinatario: &str,
        cuerpo: &str,
    ) -> Result<bool, TwilioError> {
        if !integ.twilio_configurado() {
            tracing::warn!("Twilio no configurado — SMS a {} omitido", destinatario);
            return Ok(false);
        }

        let account_sid = integ.twilio_account_sid.as_deref().unwrap_or_default();
        let auth_token = integ.twilio_auth_token.as_deref().unwrap_or_default();
        let from = integ.twilio_from_number.as_deref().unwrap_or_default();

        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{account_sid}/Messages.json"
        );

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .basic_auth(account_sid, Some(auth_token))
            .form(&[("To", destinatario), ("From", from), ("Body", cuerpo)])
            .send()
            .await
            .map_err(|e| TwilioError::Http(e.to_string()))?;

        let status = resp.status().as_u16();
        if status >= 400 {
            let body = resp.text().await.unwrap_or_default();
            tracing::error!("Twilio error {status}: {body}");
            return Err(TwilioError::Api { status, body });
        }

        tracing::info!("SMS enviado a {} via Twilio", destinatario);
        Ok(true)
    }
}

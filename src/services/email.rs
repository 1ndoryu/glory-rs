/* [263A-15] Servicio de email para envío de correos transaccionales.
 * Usa lettre con SMTP. Si SMTP no está configurado, loguea el enlace
 * en la consola para facilitar desarrollo local. */

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use crate::config::SmtpConfig;

pub struct EmailService;

impl EmailService {
    /// Envía el email de recuperación de contraseña.
    /// Si SMTP no está configurado, loguea el enlace y retorna Ok.
    pub async fn enviar_reset_password(
        smtp: Option<&SmtpConfig>,
        destinatario: &str,
        enlace: &str,
    ) -> Result<(), EmailError> {
        let Some(cfg) = smtp else {
            tracing::warn!(
                "SMTP no configurado — enlace de recuperación para {}: {}",
                destinatario,
                enlace
            );
            return Ok(());
        };

        let email = Message::builder()
            .from(
                format!("{} <{}>", cfg.from_name, cfg.from_email)
                    .parse()
                    .map_err(|e| EmailError::Build(format!("From inválido: {e}")))?,
            )
            .to(destinatario
                .parse()
                .map_err(|e| EmailError::Build(format!("Destinatario inválido: {e}")))?)
            .subject("Recuperar contraseña")
            .header(ContentType::TEXT_HTML)
            .body(body_reset_password(enlace))
            .map_err(|e| EmailError::Build(e.to_string()))?;

        let creds = Credentials::new(cfg.user.clone(), cfg.password.clone());

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&cfg.host)
            .map_err(|e| EmailError::Transport(e.to_string()))?
            .port(cfg.port)
            .credentials(creds)
            .build();

        mailer
            .send(email)
            .await
            .map_err(|e| EmailError::Transport(e.to_string()))?;

        tracing::info!("Email de recuperación enviado a {}", destinatario);
        Ok(())
    }
}

fn body_reset_password(enlace: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html><body style="font-family:sans-serif;padding:2rem;">
<h2>Recuperar contraseña</h2>
<p>Has solicitado restablecer tu contraseña. Pulsa el botón para continuar:</p>
<a href="{enlace}" style="display:inline-block;padding:12px 24px;background:#2563eb;color:#fff;
text-decoration:none;border-radius:6px;margin:1rem 0;">Restablecer contraseña</a>
<p>Si no solicitaste esto, ignora este correo.</p>
<p style="color:#888;font-size:0.85rem;">Este enlace expira en 1 hora.</p>
</body></html>"#
    )
}

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Error construyendo email: {0}")]
    Build(String),
    #[error("Error de transporte SMTP: {0}")]
    Transport(String),
}

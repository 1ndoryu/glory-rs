/* [263A-15] Servicio de email para envío de correos transaccionales.
 * Usa lettre con SMTP. Si SMTP no está configurado, loguea el enlace
 * en la consola para facilitar desarrollo local.
 * [283A-23] Método genérico `enviar` para campañas y recordatorios.
 * Parámetros SMTP centralizados en `SmtpParams` para cumplir clippy. */

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use crate::config::SmtpConfig;
use crate::models::IntegracionMarketing;

/// Parámetros SMTP agrupados para evitar demasiados argumentos
struct SmtpParams<'a> {
    host: &'a str,
    port: u16,
    user: &'a str,
    password: &'a str,
    from_email: &'a str,
    from_name: &'a str,
}

pub struct EmailService;

impl EmailService {
    /// Envía el email de recuperación de contraseña.
    /// Si SMTP no está configurado, loguea el enlace y retorna `Ok`.
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

        let params = SmtpParams {
            host: &cfg.host,
            port: cfg.port,
            user: &cfg.user,
            password: &cfg.password,
            from_email: &cfg.from_email,
            from_name: &cfg.from_name,
        };
        enviar_smtp(&params, destinatario, "Recuperar contraseña", &body_reset_password(enlace)).await
    }

    /// Envía un email genérico usando credentials de integraciones de marketing.
    /// Retorna `Ok(false)` si SMTP no está configurado en la integración.
    pub async fn enviar_campana(
        integ: &IntegracionMarketing,
        destinatario: &str,
        asunto: &str,
        cuerpo_html: &str,
    ) -> Result<bool, EmailError> {
        if !integ.smtp_configurado() {
            tracing::warn!("SMTP marketing no configurado — email a {} no enviado", destinatario);
            return Ok(false);
        }

        let user = integ.smtp_user.as_deref().unwrap_or_default();
        let params = SmtpParams {
            host: integ.smtp_host.as_deref().unwrap_or_default(),
            port: u16::try_from(integ.smtp_port.unwrap_or(587)).unwrap_or(587),
            user,
            password: integ.smtp_password.as_deref().unwrap_or_default(),
            from_email: integ.smtp_from_email.as_deref().unwrap_or(user),
            from_name: integ.smtp_from_name.as_deref().unwrap_or("Restaurante"),
        };

        enviar_smtp(&params, destinatario, asunto, cuerpo_html).await?;
        Ok(true)
    }
}

/// Envío SMTP centralizado — función libre para evitar demasiadas líneas en `impl`
async fn enviar_smtp(
    p: &SmtpParams<'_>,
    destinatario: &str,
    asunto: &str,
    cuerpo_html: &str,
) -> Result<(), EmailError> {
    let email = Message::builder()
        .from(
            format!("{} <{}>", p.from_name, p.from_email)
                .parse()
                .map_err(|e| EmailError::Build(format!("From inválido: {e}")))?,
        )
        .to(destinatario
            .parse()
            .map_err(|e| EmailError::Build(format!("Destinatario inválido: {e}")))?)
        .subject(asunto)
        .header(ContentType::TEXT_HTML)
        .body(cuerpo_html.to_string())
        .map_err(|e| EmailError::Build(e.to_string()))?;

    let creds = Credentials::new(p.user.to_string(), p.password.to_string());
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(p.host)
        .map_err(|e| EmailError::Transport(e.to_string()))?
        .port(p.port)
        .credentials(creds)
        .build();

    mailer
        .send(email)
        .await
        .map_err(|e| EmailError::Transport(e.to_string()))?;

    tracing::info!("Email enviado a {}", destinatario);
    Ok(())
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

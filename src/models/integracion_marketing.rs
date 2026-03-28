/* [283A-23] Modelo de integraciones de marketing.
 * Almacena credentials de terceros (SMTP, Twilio, Meta WhatsApp) por usuario.
 * Las credenciales no se serializan en respuestas GET por seguridad —
 * solo se devuelve si están configuradas o no. */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Integraciones de marketing almacenadas por usuario
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct IntegracionMarketing {
    pub id: Uuid,
    pub user_id: Uuid,

    /* SMTP — se ocultan en respuesta JSON */
    #[serde(skip_serializing)]
    pub smtp_host: Option<String>,
    #[serde(skip_serializing)]
    pub smtp_port: Option<i32>,
    #[serde(skip_serializing)]
    pub smtp_user: Option<String>,
    #[serde(skip_serializing)]
    pub smtp_password: Option<String>,
    #[serde(skip_serializing)]
    pub smtp_from_email: Option<String>,
    #[serde(skip_serializing)]
    pub smtp_from_name: Option<String>,

    /* Twilio — se ocultan en respuesta JSON */
    #[serde(skip_serializing)]
    pub twilio_account_sid: Option<String>,
    #[serde(skip_serializing)]
    pub twilio_auth_token: Option<String>,
    #[serde(skip_serializing)]
    pub twilio_from_number: Option<String>,

    /* Meta WhatsApp — se ocultan en respuesta JSON */
    #[serde(skip_serializing)]
    pub meta_waba_id: Option<String>,
    #[serde(skip_serializing)]
    pub meta_business_app_id: Option<String>,
    #[serde(skip_serializing)]
    pub meta_access_token: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntegracionMarketing {
    /// Indica si SMTP está configurado (`host` + `user` + `password` presentes)
    #[must_use]
    pub fn smtp_configurado(&self) -> bool {
        self.smtp_host.is_some() && self.smtp_user.is_some() && self.smtp_password.is_some()
    }

    /// Indica si Twilio está configurado
    #[must_use]
    pub fn twilio_configurado(&self) -> bool {
        self.twilio_account_sid.is_some()
            && self.twilio_auth_token.is_some()
            && self.twilio_from_number.is_some()
    }

    /// Indica si Meta `WhatsApp` está configurado
    #[must_use]
    pub fn meta_configurado(&self) -> bool {
        self.meta_waba_id.is_some() && self.meta_access_token.is_some()
    }
}

/// Vista pública: muestra solo si cada integración está configurada, sin exponer credentials
#[derive(Debug, Serialize, ToSchema)]
pub struct IntegracionMarketingPublica {
    pub id: Uuid,
    pub smtp_configurado: bool,
    pub smtp_from_email: Option<String>,
    pub smtp_from_name: Option<String>,
    pub twilio_configurado: bool,
    pub twilio_from_number: Option<String>,
    pub meta_configurado: bool,
    pub meta_waba_id: Option<String>,
}

impl From<&IntegracionMarketing> for IntegracionMarketingPublica {
    fn from(i: &IntegracionMarketing) -> Self {
        Self {
            id: i.id,
            smtp_configurado: i.smtp_configurado(),
            smtp_from_email: i.smtp_from_email.clone(),
            smtp_from_name: i.smtp_from_name.clone(),
            twilio_configurado: i.twilio_configurado(),
            twilio_from_number: i.twilio_from_number.clone(),
            meta_configurado: i.meta_configurado(),
            meta_waba_id: i.meta_waba_id.clone(),
        }
    }
}

/// Request para actualizar integraciones de marketing
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ActualizarIntegracionesRequest {
    #[validate(length(max = 255))]
    pub smtp_host: Option<String>,
    pub smtp_port: Option<i32>,
    #[validate(length(max = 255))]
    pub smtp_user: Option<String>,
    #[validate(length(max = 255))]
    pub smtp_password: Option<String>,
    #[validate(length(max = 255))]
    pub smtp_from_email: Option<String>,
    #[validate(length(max = 100))]
    pub smtp_from_name: Option<String>,

    #[validate(length(max = 100))]
    pub twilio_account_sid: Option<String>,
    #[validate(length(max = 100))]
    pub twilio_auth_token: Option<String>,
    #[validate(length(max = 20))]
    pub twilio_from_number: Option<String>,

    #[validate(length(max = 100))]
    pub meta_waba_id: Option<String>,
    #[validate(length(max = 100))]
    pub meta_business_app_id: Option<String>,
    pub meta_access_token: Option<String>,
}

/* [094A-5] Modelos de reglas de inactividad de clientes.
 * Cada regla define: después de N días sin visitar, enviar mensaje por canal X. */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct ReglaInactividad {
    pub id: Uuid,
    pub user_id: Uuid,
    pub nombre: String,
    pub dias_inactividad: i32,
    pub canal: String,
    pub mensaje_plantilla: String,
    pub activa: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CrearReglaInactividadRequest {
    #[validate(length(min = 1, max = 100))]
    pub nombre: String,
    #[validate(range(min = 1))]
    pub dias_inactividad: i32,
    #[validate(custom(function = "validar_canal"))]
    pub canal: String,
    #[validate(length(min = 1))]
    pub mensaje_plantilla: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ActualizarReglaInactividadRequest {
    #[validate(length(min = 1, max = 100))]
    pub nombre: Option<String>,
    #[validate(range(min = 1))]
    pub dias_inactividad: Option<i32>,
    pub canal: Option<String>,
    pub mensaje_plantilla: Option<String>,
    pub activa: Option<bool>,
}

fn validar_canal(canal: &str) -> Result<(), validator::ValidationError> {
    match canal {
        "email" | "sms" | "whatsapp" => Ok(()),
        _ => {
            let mut e = validator::ValidationError::new("canal_invalido");
            e.message = Some("Canal debe ser email, sms o whatsapp".into());
            Err(e)
        }
    }
}

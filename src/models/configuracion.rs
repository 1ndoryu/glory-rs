/* [263A-17] Modelo de configuración del restaurante.
 * Campos obligatorios al reservar + IVA por defecto + nombre restaurante. */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Configuración almacenada del restaurante
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
#[allow(clippy::struct_excessive_bools)] /* 4 flags de campos obligatorios al reservar — intencional */
pub struct ConfiguracionRestaurante {
    pub id: Uuid,
    pub user_id: Uuid,
    pub reserva_email_obligatorio: bool,
    pub reserva_telefono_obligatorio: bool,
    pub reserva_nombre_obligatorio: bool,
    pub reserva_apellidos_obligatorio: bool,
    pub iva_por_defecto: rust_decimal::Decimal,
    pub nombre_restaurante: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request para actualizar la configuración
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ActualizarConfiguracionRequest {
    pub reserva_email_obligatorio: Option<bool>,
    pub reserva_telefono_obligatorio: Option<bool>,
    pub reserva_nombre_obligatorio: Option<bool>,
    pub reserva_apellidos_obligatorio: Option<bool>,
    #[validate(custom(function = "validar_iva"))]
    pub iva_por_defecto: Option<rust_decimal::Decimal>,
    #[validate(length(max = 255))]
    pub nombre_restaurante: Option<String>,
}

fn validar_iva(valor: &rust_decimal::Decimal) -> Result<(), validator::ValidationError> {
    let cero = rust_decimal::Decimal::ZERO;
    let cien = rust_decimal::Decimal::ONE_HUNDRED;
    if *valor < cero || *valor > cien {
        let mut err = validator::ValidationError::new("rango_iva");
        err.message = Some("IVA debe estar entre 0 y 100%".into());
        return Err(err);
    }
    Ok(())
}

/* [263A-25] Modelo de reglas de recordatorio automático.
 * Cada regla define: horas_antes de la reserva, canal (sms/email/whatsapp),
 * mensaje plantilla y si está activa. El scheduler revisa periódicamente
 * y envía según estas reglas. */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema, Clone)]
pub struct ReglaRecordatorio {
    pub id: Uuid,
    pub user_id: Uuid,
    pub nombre: String,
    pub horas_antes: i32,
    pub canal: String,
    pub mensaje_plantilla: String,
    pub activa: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct CrearReglaRequest {
    #[validate(length(min = 1, max = 255))]
    pub nombre: String,
    #[validate(range(min = 1))]
    pub horas_antes: i32,
    #[validate(length(min = 1, max = 20))]
    pub canal: String,
    pub mensaje_plantilla: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct ActualizarReglaRequest {
    pub nombre: Option<String>,
    pub horas_antes: Option<i32>,
    pub canal: Option<String>,
    pub mensaje_plantilla: Option<String>,
    pub activa: Option<bool>,
}

#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct RecordatorioEnviado {
    pub id: Uuid,
    pub regla_id: Uuid,
    pub reserva_id: Uuid,
    pub canal: String,
    pub estado: String,
    pub enviado_at: DateTime<Utc>,
    pub error_mensaje: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReglasPaginadas {
    pub items: Vec<ReglaRecordatorio>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ReglasQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HistorialRecordatorios {
    pub items: Vec<RecordatorioEnviadoDetalle>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

/* Detalle de un recordatorio enviado con datos de la reserva */
#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct RecordatorioEnviadoDetalle {
    pub id: Uuid,
    pub regla_id: Uuid,
    pub regla_nombre: String,
    pub reserva_id: Uuid,
    pub nombre_cliente: String,
    pub fecha_reserva: chrono::NaiveDate,
    pub hora_reserva: chrono::NaiveTime,
    pub canal: String,
    pub estado: String,
    pub enviado_at: DateTime<Utc>,
    pub error_mensaje: Option<String>,
}

pub const CANALES_RECORDATORIO: &[&str] = &["sms", "email", "whatsapp"];

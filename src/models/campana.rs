/* [263A-23] Modelo de campañas de marketing multi-canal.
 * Soporta SMS, email, WhatsApp con segmentación por actividad del cliente.
 * Referencia: cliente/Data III/mensajes.md + screenshot Cover Manager. */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

/* Campana de marketing — tabla master */
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct Campana {
    pub id: Uuid,
    pub user_id: Uuid,
    pub nombre: String,
    pub descripcion_interna: String,
    pub cuerpo_mensaje: String,
    pub canales: Vec<String>,
    pub segmento: String,
    pub incluir_baja: bool,
    pub telefono_baja: String,
    pub estado: String,
    pub total_destinatarios: i32,
    pub total_enviados: i32,
    pub total_fallidos: i32,
    /* [024A-1] Plantilla WhatsApp aprobada por Meta para envío por template API */
    pub plantilla_whatsapp_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/* Request para crear una campaña */
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CrearCampanaRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "El nombre es obligatorio y no debe exceder 255 caracteres"
    ))]
    pub nombre: String,
    pub descripcion_interna: Option<String>,
    pub cuerpo_mensaje: Option<String>,
    pub canales: Vec<String>,
    #[validate(length(max = 50))]
    pub segmento: Option<String>,
    pub incluir_baja: Option<bool>,
    pub telefono_baja: Option<String>,
    /* [024A-1] Plantilla aprobada requerida si canal incluye 'whatsapp' */
    pub plantilla_whatsapp_id: Option<Uuid>,
}

/* Request para actualizar una campaña */
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ActualizarCampanaRequest {
    #[validate(length(min = 1, max = 255))]
    pub nombre: Option<String>,
    pub descripcion_interna: Option<String>,
    pub cuerpo_mensaje: Option<String>,
    pub canales: Option<Vec<String>>,
    #[validate(length(max = 50))]
    pub segmento: Option<String>,
    pub incluir_baja: Option<bool>,
    pub telefono_baja: Option<String>,
    /* [024A-1] Plantilla aprobada requerida si canal incluye 'whatsapp' */
    pub plantilla_whatsapp_id: Option<Uuid>,
}

/* Respuesta paginada de campañas */
#[derive(Debug, Serialize, ToSchema)]
pub struct CampanasPaginadas {
    pub items: Vec<Campana>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

/* Query para listar campañas */
#[derive(Debug, Deserialize, IntoParams)]
pub struct CampanasQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
    pub estado: Option<String>,
}

fn default_page() -> i64 {
    1
}
fn default_per_page() -> i64 {
    20
}

/* Destinatario de una campaña */
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct CampanaDestinatario {
    pub id: Uuid,
    pub campana_id: Uuid,
    pub cliente_id: Uuid,
    pub canal: String,
    pub estado: String,
    pub enviado_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/* Preview de segmentación — cuántos clientes recibirán la campaña */
#[derive(Debug, Serialize, ToSchema)]
pub struct SegmentoPreview {
    pub segmento: String,
    pub total_clientes: i64,
    pub con_email: i64,
    pub con_telefono: i64,
    pub con_consentimiento_email: i64,
    pub con_consentimiento_sms: i64,
}

/* Query para preview de segmento */
#[derive(Debug, Deserialize, IntoParams)]
pub struct SegmentoPreviewQuery {
    pub segmento: String,
}

/* Constantes de segmentos válidos */
pub const SEGMENTOS_VALIDOS: &[&str] = &[
    "habitual",
    "sin_1m",
    "sin_3m",
    "sin_6m",
    "sin_9m",
    "sin_1a",
    "sin_mas_1a",
    "todos",
];

/* Constantes de canales válidos */
pub const CANALES_VALIDOS: &[&str] = &["sms", "email", "whatsapp"];

/* [263A-24] Modelo de plantillas WhatsApp para Meta Business API.
 * Las plantillas pasan por un flujo: borrador → enviada → aprobada/rechazada.
 * Meta requiere categoría (MARKETING, UTILITY, AUTHENTICATION) e idioma. */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

/* Categorías de plantilla permitidas por Meta */
pub const CATEGORIAS_PLANTILLA: &[&str] = &["MARKETING", "UTILITY", "AUTHENTICATION"];

/* Plantilla WhatsApp — registro principal */
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct PlantillaWhatsapp {
    pub id: Uuid,
    pub user_id: Uuid,
    pub nombre: String,
    pub categoria: String,
    pub idioma: String,
    pub cuerpo_mensaje: String,
    pub cabecera_texto: Option<String>,
    pub pie_texto: Option<String>,
    pub cabecera_media_url: Option<String>,
    pub cabecera_media_tipo: Option<String>,
    pub estado: String,
    pub meta_template_id: Option<String>,
    pub meta_razon_rechazo: Option<String>,
    pub meta_enviada_at: Option<DateTime<Utc>>,
    pub meta_respondida_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/* Request para crear una plantilla */
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CrearPlantillaRequest {
    #[validate(length(min = 1, max = 255, message = "nombre requerido (1-255 chars)"))]
    pub nombre: String,
    pub categoria: Option<String>,
    pub idioma: Option<String>,
    pub cuerpo_mensaje: Option<String>,
    pub cabecera_texto: Option<String>,
    pub pie_texto: Option<String>,
    pub cabecera_media_url: Option<String>,
    pub cabecera_media_tipo: Option<String>,
}

/* Request para actualizar una plantilla (solo borradores) */
#[derive(Debug, Deserialize, ToSchema)]
pub struct ActualizarPlantillaRequest {
    pub nombre: Option<String>,
    pub categoria: Option<String>,
    pub idioma: Option<String>,
    pub cuerpo_mensaje: Option<String>,
    pub cabecera_texto: Option<String>,
    pub pie_texto: Option<String>,
    pub cabecera_media_url: Option<String>,
    pub cabecera_media_tipo: Option<String>,
}

/* Respuesta paginada */
#[derive(Debug, Serialize, ToSchema)]
pub struct PlantillasPaginadas {
    pub items: Vec<PlantillaWhatsapp>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

/* Query params para listar plantillas */
#[derive(Debug, Deserialize, IntoParams)]
pub struct PlantillasQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub estado: Option<String>,
}

/* [283A-2] Modelos para API keys — autenticación de chatbots externos.
 * Las API keys se almacenan como hash SHA-256 en BD. La key completa
 * solo se muestra una vez al crearla. El prefijo (8 chars) sirve
 * para identificación visual sin exponer la key. */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub nombre: String,
    pub key_hash: String,
    pub key_prefix: String,
    pub permisos: serde_json::Value,
    pub activa: bool,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CrearApiKeyRequest {
    #[validate(length(min = 1, max = 100))]
    pub nombre: String,
}

/// Respuesta pública de API key (sin hash)
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiKeyResponse {
    pub id: Uuid,
    pub nombre: String,
    pub key_prefix: String,
    pub permisos: serde_json::Value,
    pub activa: bool,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Respuesta al crear una key — incluye la key completa (solo se muestra una vez)
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiKeyCreatedResponse {
    pub id: Uuid,
    pub nombre: String,
    pub key: String,
    pub key_prefix: String,
    pub permisos: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

/// Modelos para el endpoint de disponibilidad del chatbot
#[derive(Debug, Serialize, ToSchema)]
pub struct DisponibilidadResponse {
    pub fecha: chrono::NaiveDate,
    pub franjas: Vec<FranjaDisponibilidad>,
    pub capacidad_total: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FranjaDisponibilidad {
    pub hora: chrono::NaiveTime,
    pub personas_reservadas: i32,
    pub mesas_ocupadas: i32,
    pub mesas_disponibles: i32,
    pub capacidad_disponible: i32,
}

/// Info pública del restaurante para el chatbot
#[derive(Debug, Serialize, ToSchema)]
pub struct RestauranteInfoResponse {
    pub nombre: String,
    pub campos_obligatorios: CamposObligatorios,
    pub capacidad_total: i32,
    pub zonas: Vec<ZonaResumen>,
}

#[derive(Debug, Serialize, ToSchema)]
#[allow(clippy::struct_excessive_bools)]
pub struct CamposObligatorios {
    pub email: bool,
    pub telefono: bool,
    pub nombre: bool,
    pub apellidos: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ZonaResumen {
    pub nombre: String,
    pub mesas: i32,
    pub capacidad_min: i32,
    pub capacidad_max: i32,
}

/// Request del chatbot para crear reserva (campos simplificados)
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ChatbotCrearReservaRequest {
    pub fecha: chrono::NaiveDate,
    pub hora: chrono::NaiveTime,
    #[validate(length(min = 1, max = 255))]
    pub nombre_cliente: String,
    #[validate(range(min = 1))]
    pub num_personas: i32,
    #[validate(length(max = 20))]
    pub telefono: Option<String>,
    #[validate(length(max = 255))]
    pub apellidos_cliente: Option<String>,
    #[validate(length(max = 500))]
    pub notas: Option<String>,
    #[validate(length(max = 255))]
    pub email: Option<String>,
}

/// Request del chatbot para buscar reservas
#[derive(Debug, Deserialize, ToSchema)]
pub struct ChatbotBuscarReservasQuery {
    pub telefono: Option<String>,
    pub nombre: Option<String>,
    pub fecha: Option<chrono::NaiveDate>,
}

/// Reserva simplificada para el chatbot (sin IDs internos)
#[derive(Debug, Serialize, ToSchema)]
pub struct ChatbotReservaResponse {
    pub id: Uuid,
    pub fecha: chrono::NaiveDate,
    pub hora: chrono::NaiveTime,
    pub nombre_cliente: String,
    pub apellidos_cliente: String,
    pub num_personas: i32,
    pub estado: String,
    pub telefono: String,
    pub notas: String,
    pub mesa_numero: Option<i32>,
}

impl From<ApiKey> for ApiKeyResponse {
    fn from(k: ApiKey) -> Self {
        Self {
            id: k.id,
            nombre: k.nombre,
            key_prefix: k.key_prefix,
            permisos: k.permisos,
            activa: k.activa,
            last_used_at: k.last_used_at,
            created_at: k.created_at,
        }
    }
}

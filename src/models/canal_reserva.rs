/* 263A-9: Modelo de canal de reserva — por dónde llegan las reservas
(WhatsApp, Instagram, teléfono, web, etc.) */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Canal de reserva del restaurante
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct CanalReserva {
    pub id: Uuid,
    pub user_id: Uuid,
    pub nombre: String,
    pub created_at: DateTime<Utc>,
}

/// Request para crear un canal de reserva
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CrearCanalReservaRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "El nombre del canal es obligatorio y no debe exceder 100 caracteres"
    ))]
    pub nombre: String,
}

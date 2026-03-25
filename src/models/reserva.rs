/* 253A-5: Modelo de reserva para el restaurante.
Audio 7: conteo mes/día, listado clientes, horarios.
Roadmap sección 3: módulo de reservas. */

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

/// Estados posibles de una reserva
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[serde(rename_all = "snake_case")]
pub enum EstadoReserva {
    #[sqlx(rename = "pendiente")]
    Pendiente,
    #[sqlx(rename = "confirmada")]
    Confirmada,
    #[sqlx(rename = "cancelada")]
    Cancelada,
    #[sqlx(rename = "completada")]
    Completada,
}

/// Reserva del restaurante
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct Reserva {
    pub id: Uuid,
    pub user_id: Uuid,
    pub fecha: NaiveDate,
    pub hora: NaiveTime,
    pub nombre_cliente: String,
    pub num_personas: i32,
    pub estado: String,
    pub notas: String,
    pub telefono: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request para crear una reserva
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CrearReservaRequest {
    pub fecha: NaiveDate,
    pub hora: NaiveTime,
    #[validate(length(
        min = 1,
        max = 255,
        message = "El nombre del cliente es obligatorio y no debe exceder 255 caracteres"
    ))]
    pub nombre_cliente: String,
    #[validate(range(min = 1, message = "Debe haber al menos 1 persona"))]
    pub num_personas: i32,
    pub estado: Option<EstadoReserva>,
    #[validate(length(max = 500))]
    pub notas: Option<String>,
    #[validate(length(max = 20))]
    pub telefono: Option<String>,
}

/// Request para actualizar una reserva
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ActualizarReservaRequest {
    pub fecha: Option<NaiveDate>,
    pub hora: Option<NaiveTime>,
    #[validate(length(min = 1, max = 255))]
    pub nombre_cliente: Option<String>,
    #[validate(range(min = 1))]
    pub num_personas: Option<i32>,
    pub estado: Option<EstadoReserva>,
    #[validate(length(max = 500))]
    pub notas: Option<String>,
    #[validate(length(max = 20))]
    pub telefono: Option<String>,
}

/// Conteo de reservas para el Home — mes y día actual
#[derive(Debug, Serialize, ToSchema)]
pub struct ReservasConteo {
    pub total_mes: i64,
    pub total_hoy: i64,
}

/// Response paginada de reservas
#[derive(Debug, Serialize, ToSchema)]
pub struct ReservasPaginadas {
    pub items: Vec<Reserva>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

/// Query para listar reservas
#[derive(Debug, Deserialize, IntoParams)]
pub struct ReservasQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
    pub fecha: Option<NaiveDate>,
}

fn default_page() -> i64 {
    1
}
fn default_per_page() -> i64 {
    20
}

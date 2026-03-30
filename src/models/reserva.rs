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
    #[sqlx(rename = "no_show")]
    NoShow,
    #[sqlx(rename = "lista_espera")]
    ListaEspera,
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
    pub cliente_id: Option<Uuid>,
    pub canal_id: Option<Uuid>,
    pub no_show: bool,
    pub num_mesa: Option<i32>,
    pub apellidos_cliente: String,
    pub mesa_id: Option<Uuid>,
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
    pub num_mesa: Option<i32>,
    #[validate(length(max = 255))]
    pub apellidos_cliente: Option<String>,
    pub canal_id: Option<Uuid>,
    pub mesa_id: Option<Uuid>,
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
    pub num_mesa: Option<i32>,
    #[validate(length(max = 255))]
    pub apellidos_cliente: Option<String>,
    pub canal_id: Option<Uuid>,
    pub mesa_id: Option<Uuid>,
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
    /// Fecha exacta (mantiene compatibilidad). Si se envía junto con `fecha_desde`/`fecha_hasta`, se ignora.
    pub fecha: Option<NaiveDate>,
    /// [303A-15] Inicio del rango de fechas (inclusive)
    pub fecha_desde: Option<NaiveDate>,
    /// [303A-15] Fin del rango de fechas (inclusive)
    pub fecha_hasta: Option<NaiveDate>,
    pub estado: Option<String>,
    pub turno: Option<String>,
    /// Búsqueda por nombre o apellidos del cliente
    pub busqueda: Option<String>,
}

/// Resumen diario de reservas — para la vista mes
#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct ResumenDiario {
    pub fecha: NaiveDate,
    pub total_reservas: i64,
    pub total_personas: i64,
}

/// Query para resumen mensual
#[derive(Debug, Deserialize, IntoParams)]
pub struct ResumenMesQuery {
    pub anio: i32,
    pub mes: i32,
}

fn default_page() -> i64 {
    1
}
fn default_per_page() -> i64 {
    20
}

/// Estadísticas de no-shows — 263A-8
#[derive(Debug, Serialize, ToSchema)]
pub struct NoShowStats {
    pub total_reservas: i64,
    pub total_no_shows: i64,
    pub ratio_porcentaje: f64,
    pub por_canal: Vec<NoShowPorCanal>,
}

/// No-shows desglosados por canal
#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct NoShowPorCanal {
    pub canal_nombre: Option<String>,
    pub total_reservas: i64,
    pub no_shows: i64,
    pub ratio_porcentaje: f64,
}

/// Query para estadísticas de no-show
#[derive(Debug, Deserialize, IntoParams)]
pub struct NoShowQuery {
    pub fecha_desde: Option<NaiveDate>,
    pub fecha_hasta: Option<NaiveDate>,
}

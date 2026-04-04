/* 253A-5: Modelo de gasto para el restaurante.
Campos basados en roadmap sección 4 — Nuevo Gasto. */

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

use super::common::MetodoPago;

/// Tipos de documento de gasto
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[serde(rename_all = "snake_case")]
pub enum TipoDocumento {
    #[sqlx(rename = "factura")]
    Factura,
    #[sqlx(rename = "albaran")]
    Albaran,
    #[sqlx(rename = "ticket")]
    Ticket,
}

/// Categoría de gasto — precargada en BD
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct CategoriaGasto {
    pub id: Uuid,
    pub nombre: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
}

/// Gasto registrado en el restaurante
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct Gasto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub fecha: NaiveDate,
    pub proveedor: String,
    pub categoria_id: Option<Uuid>,
    pub tipo_documento: String,
    pub metodo_pago: String,
    pub numero_documento: String,
    pub recurrente: bool,
    pub importe_base: rust_decimal::Decimal,
    pub importe_iva: rust_decimal::Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request para crear un gasto
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CrearGastoRequest {
    pub fecha: NaiveDate,
    #[validate(length(max = 255, message = "El proveedor no debe exceder 255 caracteres"))]
    pub proveedor: Option<String>,
    pub categoria_id: Option<Uuid>,
    pub tipo_documento: TipoDocumento,
    /* 253A-21: metodo_pago es opcional segun la plataforma Haddock (video 7) */
    pub metodo_pago: Option<MetodoPago>,
    #[validate(length(
        max = 100,
        message = "El número de documento no debe exceder 100 caracteres"
    ))]
    pub numero_documento: Option<String>,
    pub recurrente: Option<bool>,
    pub importe_base: rust_decimal::Decimal,
    pub importe_iva: rust_decimal::Decimal,
}

/* [283A-22] Request para actualizar un gasto — todos los campos opcionales
 * para soportar actualizaciones parciales (PATCH semántico via PUT). */
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ActualizarGastoRequest {
    pub fecha: Option<NaiveDate>,
    #[validate(length(max = 255, message = "El proveedor no debe exceder 255 caracteres"))]
    pub proveedor: Option<String>,
    pub categoria_id: Option<Uuid>,
    pub tipo_documento: Option<TipoDocumento>,
    pub metodo_pago: Option<MetodoPago>,
    #[validate(length(
        max = 100,
        message = "El número de documento no debe exceder 100 caracteres"
    ))]
    pub numero_documento: Option<String>,
    pub recurrente: Option<bool>,
    pub importe_base: Option<rust_decimal::Decimal>,
    pub importe_iva: Option<rust_decimal::Decimal>,
}

/// Response paginada de gastos
#[derive(Debug, Serialize, ToSchema)]
pub struct GastosPaginados {
    pub items: Vec<Gasto>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

/// Query para listar gastos con filtros
#[derive(Debug, Deserialize, IntoParams)]
pub struct GastosQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
    pub desde: Option<NaiveDate>,
    pub hasta: Option<NaiveDate>,
    pub categoria_id: Option<Uuid>,
    /// Búsqueda por texto: proveedor, tipo documento, número documento
    pub busqueda: Option<String>,
    /// Campo de ordenamiento: `fecha`, `proveedor`, `importe_base`, `tipo_documento`, `metodo_pago`
    pub sort_by: Option<String>,
    /// Dirección de orden: asc o desc. Por defecto desc
    pub sort_order: Option<String>,
}

fn default_page() -> i64 {
    1
}
fn default_per_page() -> i64 {
    20
}

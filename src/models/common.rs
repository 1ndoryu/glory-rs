/* 253A-7: Enums compartidos entre ventas y gastos.
   Movidos aquí para evitar referencia cruzada super::venta:: que genera
   schema OpenAPI con nombre "super.venta.MetodoPago" inválido. */

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Métodos de pago soportados — compartido entre ventas y gastos
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[serde(rename_all = "snake_case")]
pub enum MetodoPago {
    #[sqlx(rename = "efectivo")]
    Efectivo,
    #[sqlx(rename = "tarjeta")]
    Tarjeta,
    #[sqlx(rename = "transferencia")]
    Transferencia,
    #[sqlx(rename = "otros")]
    Otros,
}

/* 253A-5: Servicio de dashboard — resumen económico.
Audio 9: ventas - gastos = margen. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::ResumenEconomico;
use crate::repositories::{GastoRepository, VentaRepository};

pub struct DashboardService;

impl DashboardService {
    /// Resumen económico de un mes: total ventas, total gastos, margen
    pub async fn resumen_mes(
        pool: &PgPool,
        user_id: Uuid,
        year: i32,
        month: u32,
    ) -> Result<ResumenEconomico, AppError> {
        let desde = chrono::NaiveDate::from_ymd_opt(year, month, 1)
            .ok_or_else(|| AppError::BadRequest("Fecha inválida".into()))?;
        let hasta = if month == 12 {
            chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            chrono::NaiveDate::from_ymd_opt(year, month + 1, 1)
        }
        .ok_or_else(|| AppError::BadRequest("Fecha inválida".into()))?
        .pred_opt()
        .ok_or_else(|| AppError::Internal("Error calculando fecha".into()))?;

        let total_ventas = VentaRepository::total_periodo(pool, user_id, desde, hasta).await?;
        let total_gastos = GastoRepository::total_periodo(pool, user_id, desde, hasta).await?;
        let margen = total_ventas - total_gastos;

        let mes = format!("{year}-{month:02}");

        Ok(ResumenEconomico {
            total_ventas,
            total_gastos,
            margen,
            mes,
        })
    }
}

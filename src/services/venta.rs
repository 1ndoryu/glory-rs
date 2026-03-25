/* 253A-5: Servicio de ventas — lógica de negocio */

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{CrearVentaRequest, Venta, VentasPaginadas};
use crate::repositories::venta::NuevaVenta;
use crate::repositories::VentaRepository;

pub struct VentaService;

impl VentaService {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        req: CrearVentaRequest,
    ) -> Result<Venta, AppError> {
        let turno = serde_json::to_value(&req.turno)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "manana".into());
        let canal = serde_json::to_value(&req.canal)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "comedor".into());
        let metodo = serde_json::to_value(&req.metodo_pago)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "efectivo".into());

        let descripcion = req.descripcion.as_deref().unwrap_or("");
        let data = NuevaVenta {
            user_id,
            fecha: req.fecha,
            comensales: req.comensales,
            descripcion,
            iva_porcentaje: req.iva_porcentaje,
            turno: &turno,
            canal: &canal,
            metodo_pago: &metodo,
            importe_base: req.importe_base,
            importe_iva: req.importe_iva,
        };

        let venta = VentaRepository::create(pool, &data).await?;
        Ok(venta)
    }

    pub async fn get(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Venta, AppError> {
        VentaRepository::find_by_id(pool, id, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Venta no encontrada".into()))
    }

    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        page: i64,
        per_page: i64,
        desde: Option<chrono::NaiveDate>,
        hasta: Option<chrono::NaiveDate>,
    ) -> Result<VentasPaginadas, AppError> {
        let (items, total) =
            VentaRepository::list(pool, user_id, page, per_page, desde, hasta).await?;
        Ok(VentasPaginadas {
            items,
            total,
            page,
            per_page,
        })
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        if !VentaRepository::delete(pool, id, user_id).await? {
            return Err(AppError::NotFound("Venta no encontrada".into()));
        }
        Ok(())
    }
}

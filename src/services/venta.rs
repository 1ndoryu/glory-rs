/* 253A-5: Servicio de ventas — lógica de negocio
 * [064A-5] Añadido hook post-create/update para sincronización con Haddock POS API */

use sqlx::PgPool;
use tracing::warn;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{ActualizarVentaRequest, CrearVentaRequest, Venta, VentasPaginadas};
use crate::repositories::venta::{ActualizarVentaData, NuevaVenta};
use crate::repositories::{ConfiguracionRepository, VentaRepository};

use super::HaddockService;

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
            /* [034A-5] Ventas manuales no tienen reserva ni cliente asociado */
            reserva_id: None,
            cliente_id: None,
        };

        let venta = VentaRepository::create(pool, &data).await?;

        /* [064A-5] Sincronizar con Haddock en background (no bloquea la respuesta) */
        Self::spawn_haddock_sync(pool.clone(), user_id, venta.clone());

        Ok(venta)
    }

    pub async fn get(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Venta, AppError> {
        VentaRepository::find_by_id(pool, id, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Venta no encontrada".into()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        page: i64,
        per_page: i64,
        desde: Option<chrono::NaiveDate>,
        hasta: Option<chrono::NaiveDate>,
        busqueda: Option<String>,
        turno: Option<String>,
        canal: Option<String>,
        metodo_pago: Option<String>,
        sort_by: Option<String>,
        sort_order: Option<String>,
    ) -> Result<VentasPaginadas, AppError> {
        let (items, total) =
            VentaRepository::list(
                pool, user_id, page, per_page, desde, hasta,
                busqueda.as_deref(), turno.as_deref(), canal.as_deref(), metodo_pago.as_deref(),
                sort_by.as_deref(), sort_order.as_deref(),
            ).await?;
        Ok(VentasPaginadas {
            items,
            total,
            page,
            per_page,
        })
    }

    /* [283A-22] Actualizar parcialmente una venta.
     * Convierte enums a string igual que en create para mantener consistencia. */
    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        req: ActualizarVentaRequest,
    ) -> Result<Venta, AppError> {
        let turno = req.turno.as_ref().and_then(|t| {
            serde_json::to_value(t)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
        });
        let canal = req.canal.as_ref().and_then(|c| {
            serde_json::to_value(c)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
        });
        let metodo = req.metodo_pago.as_ref().and_then(|m| {
            serde_json::to_value(m)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
        });

        let data = ActualizarVentaData {
            id,
            user_id,
            fecha: req.fecha,
            comensales: req.comensales,
            descripcion: req.descripcion.as_deref(),
            iva_porcentaje: req.iva_porcentaje,
            turno: turno.as_deref(),
            canal: canal.as_deref(),
            metodo_pago: metodo.as_deref(),
            importe_base: req.importe_base,
            importe_iva: req.importe_iva,
        };

        let venta = VentaRepository::update(pool, &data)
            .await?
            .ok_or_else(|| AppError::NotFound("Venta no encontrada".into()))?;

        /* [064A-5] Re-sincronizar con Haddock tras actualización */
        Self::spawn_haddock_sync(pool.clone(), user_id, venta.clone());

        Ok(venta)
    }

    /* [064A-5] Lanza sincronización con Haddock en un task independiente.
     * [064A-6] Ahora pasa pool para actualizar estado sync en BD.
     * No bloquea ni falla la operación principal. */
    fn spawn_haddock_sync(pool: PgPool, user_id: Uuid, venta: Venta) {
        tokio::spawn(async move {
            let config = match ConfiguracionRepository::obtener_o_crear(&pool, user_id).await {
                Ok(c) => c,
                Err(e) => {
                    warn!("[064A-5] Error obteniendo config para Haddock sync: {e}");
                    return;
                }
            };
            HaddockService::sync_order(&pool, &venta, &config).await;
        });
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        if !VentaRepository::delete(pool, id, user_id).await? {
            return Err(AppError::NotFound("Venta no encontrada".into()));
        }
        Ok(())
    }
}

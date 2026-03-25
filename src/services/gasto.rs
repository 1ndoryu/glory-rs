/* 253A-5: Servicio de gastos */

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{CategoriaGasto, CrearGastoRequest, Gasto, GastosPaginados};
use crate::repositories::gasto::NuevoGasto;
use crate::repositories::{CategoriaGastoRepository, GastoRepository};

pub struct GastoService;

impl GastoService {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        req: CrearGastoRequest,
    ) -> Result<Gasto, AppError> {
        let tipo_doc = serde_json::to_value(&req.tipo_documento)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "ticket".into());
        let metodo = serde_json::to_value(&req.metodo_pago)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "efectivo".into());

        let proveedor = req.proveedor.as_deref().unwrap_or("");
        let numero_documento = req.numero_documento.as_deref().unwrap_or("");
        let data = NuevoGasto {
            user_id,
            fecha: req.fecha,
            proveedor,
            categoria_id: req.categoria_id,
            tipo_documento: &tipo_doc,
            metodo_pago: &metodo,
            numero_documento,
            recurrente: req.recurrente.unwrap_or(false),
            importe_base: req.importe_base,
            importe_iva: req.importe_iva,
        };

        let gasto = GastoRepository::create(pool, &data).await?;
        Ok(gasto)
    }

    pub async fn get(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Gasto, AppError> {
        GastoRepository::find_by_id(pool, id, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Gasto no encontrado".into()))
    }

    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        page: i64,
        per_page: i64,
        desde: Option<chrono::NaiveDate>,
        hasta: Option<chrono::NaiveDate>,
        categoria_id: Option<Uuid>,
    ) -> Result<GastosPaginados, AppError> {
        let (items, total) =
            GastoRepository::list(pool, user_id, page, per_page, desde, hasta, categoria_id)
                .await?;
        Ok(GastosPaginados {
            items,
            total,
            page,
            per_page,
        })
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        if !GastoRepository::delete(pool, id, user_id).await? {
            return Err(AppError::NotFound("Gasto no encontrado".into()));
        }
        Ok(())
    }

    pub async fn categorias(pool: &PgPool) -> Result<Vec<CategoriaGasto>, AppError> {
        let cats = CategoriaGastoRepository::list_all(pool).await?;
        Ok(cats)
    }
}

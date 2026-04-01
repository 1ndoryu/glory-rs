/* 253A-5: Repositorio de gastos */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CategoriaGasto, Gasto};

/// Datos necesarios para insertar un gasto en BD
pub struct NuevoGasto<'a> {
    pub user_id: Uuid,
    pub fecha: chrono::NaiveDate,
    pub proveedor: &'a str,
    pub categoria_id: Option<Uuid>,
    pub tipo_documento: &'a str,
    pub metodo_pago: &'a str,
    pub numero_documento: &'a str,
    pub recurrente: bool,
    pub importe_base: rust_decimal::Decimal,
    pub importe_iva: rust_decimal::Decimal,
}

/* [283A-22] Datos para actualizar parcialmente un gasto.
 * Usa runtime queries (query_as con bind) para no requerir actualizar el cache .sqlx. */
pub struct ActualizarGastoData<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub fecha: Option<chrono::NaiveDate>,
    pub proveedor: Option<&'a str>,
    pub categoria_id: Option<Uuid>,
    pub tipo_documento: Option<&'a str>,
    pub metodo_pago: Option<&'a str>,
    pub numero_documento: Option<&'a str>,
    pub recurrente: Option<bool>,
    pub importe_base: Option<rust_decimal::Decimal>,
    pub importe_iva: Option<rust_decimal::Decimal>,
}

pub struct GastoRepository;

impl GastoRepository {
    pub async fn create(pool: &PgPool, data: &NuevoGasto<'_>) -> Result<Gasto, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as!(
            Gasto,
            "INSERT INTO gastos (id, user_id, fecha, proveedor, categoria_id, tipo_documento, \
             metodo_pago, numero_documento, recurrente, importe_base, importe_iva) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) \
             RETURNING *",
            id,
            data.user_id,
            data.fecha,
            data.proveedor,
            data.categoria_id,
            data.tipo_documento,
            data.metodo_pago,
            data.numero_documento,
            data.recurrente,
            data.importe_base,
            data.importe_iva
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Gasto>, sqlx::Error> {
        sqlx::query_as!(
            Gasto,
            "SELECT * FROM gastos WHERE id = $1 AND user_id = $2",
            id,
            user_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        page: i64,
        per_page: i64,
        desde: Option<chrono::NaiveDate>,
        hasta: Option<chrono::NaiveDate>,
        categoria_id: Option<Uuid>,
    ) -> Result<(Vec<Gasto>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;

        let items = sqlx::query_as!(
            Gasto,
            "SELECT * FROM gastos WHERE user_id = $1 \
             AND ($4::DATE IS NULL OR fecha >= $4) \
             AND ($5::DATE IS NULL OR fecha <= $5) \
             AND ($6::UUID IS NULL OR categoria_id = $6) \
             ORDER BY fecha DESC, created_at DESC LIMIT $2 OFFSET $3",
            user_id,
            per_page,
            offset,
            desde,
            hasta,
            categoria_id
        )
        .fetch_all(pool)
        .await?;

        let rec = sqlx::query!(
            "SELECT COUNT(*) as total FROM gastos WHERE user_id = $1 \
             AND ($2::DATE IS NULL OR fecha >= $2) \
             AND ($3::DATE IS NULL OR fecha <= $3) \
             AND ($4::UUID IS NULL OR categoria_id = $4)",
            user_id,
            desde,
            hasta,
            categoria_id
        )
        .fetch_one(pool)
        .await?;

        Ok((items, rec.total.unwrap_or(0)))
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM gastos WHERE id = $1 AND user_id = $2",
            id,
            user_id
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /* [283A-22] Actualizar parcialmente un gasto — COALESCE mantiene valores existentes
     * cuando el campo no se envía (None).
     * [014A-11] Convertido a query_as! para verificación SQL en compilación. */
    pub async fn update(
        pool: &PgPool,
        data: &ActualizarGastoData<'_>,
    ) -> Result<Option<Gasto>, sqlx::Error> {
        sqlx::query_as!(
            Gasto,
            "UPDATE gastos SET \
             fecha = COALESCE($3, fecha), \
             proveedor = COALESCE($4, proveedor), \
             categoria_id = COALESCE($5, categoria_id), \
             tipo_documento = COALESCE($6, tipo_documento), \
             metodo_pago = COALESCE($7, metodo_pago), \
             numero_documento = COALESCE($8, numero_documento), \
             recurrente = COALESCE($9, recurrente), \
             importe_base = COALESCE($10, importe_base), \
             importe_iva = COALESCE($11, importe_iva), \
             updated_at = NOW() \
             WHERE id = $1 AND user_id = $2 \
             RETURNING *",
            data.id,
            data.user_id,
            data.fecha,
            data.proveedor,
            data.categoria_id,
            data.tipo_documento,
            data.metodo_pago,
            data.numero_documento,
            data.recurrente,
            data.importe_base,
            data.importe_iva
        )
        .fetch_optional(pool)
        .await
    }

    /// Suma de `importe_base` de gastos en un período
    pub async fn total_periodo(
        pool: &PgPool,
        user_id: Uuid,
        desde: chrono::NaiveDate,
        hasta: chrono::NaiveDate,
    ) -> Result<rust_decimal::Decimal, sqlx::Error> {
        let rec = sqlx::query!(
            "SELECT COALESCE(SUM(importe_base), 0) as total FROM gastos \
             WHERE user_id = $1 AND fecha >= $2 AND fecha <= $3",
            user_id,
            desde,
            hasta
        )
        .fetch_one(pool)
        .await?;
        Ok(rec.total.unwrap_or_default())
    }
}

pub struct CategoriaGastoRepository;

impl CategoriaGastoRepository {
    pub async fn list_all(pool: &PgPool) -> Result<Vec<CategoriaGasto>, sqlx::Error> {
        sqlx::query_as!(
            CategoriaGasto,
            "SELECT * FROM categorias_gasto ORDER BY nombre ASC"
        )
        .fetch_all(pool)
        .await
    }
}

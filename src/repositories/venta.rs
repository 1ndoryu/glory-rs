/* 253A-5: Repositorio de ventas — queries SQL con parámetros */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Venta;

/// Datos necesarios para insertar una venta en BD
pub struct NuevaVenta<'a> {
    pub user_id: Uuid,
    pub fecha: chrono::NaiveDate,
    pub comensales: Option<i32>,
    pub descripcion: &'a str,
    pub iva_porcentaje: rust_decimal::Decimal,
    pub turno: &'a str,
    pub canal: &'a str,
    pub metodo_pago: &'a str,
    pub importe_base: rust_decimal::Decimal,
    pub importe_iva: rust_decimal::Decimal,
}

pub struct VentaRepository;

impl VentaRepository {
    pub async fn create(pool: &PgPool, data: &NuevaVenta<'_>) -> Result<Venta, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, Venta>(
            "INSERT INTO ventas (id, user_id, fecha, comensales, descripcion, iva_porcentaje, \
             turno, canal, metodo_pago, importe_base, importe_iva) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) \
             RETURNING *",
        )
        .bind(id)
        .bind(data.user_id)
        .bind(data.fecha)
        .bind(data.comensales)
        .bind(data.descripcion)
        .bind(data.iva_porcentaje)
        .bind(data.turno)
        .bind(data.canal)
        .bind(data.metodo_pago)
        .bind(data.importe_base)
        .bind(data.importe_iva)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Venta>, sqlx::Error> {
        sqlx::query_as::<_, Venta>("SELECT * FROM ventas WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
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
    ) -> Result<(Vec<Venta>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;

        let items = sqlx::query_as::<_, Venta>(
            "SELECT * FROM ventas WHERE user_id = $1 \
             AND ($4::DATE IS NULL OR fecha >= $4) \
             AND ($5::DATE IS NULL OR fecha <= $5) \
             ORDER BY fecha DESC, created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(per_page)
        .bind(offset)
        .bind(desde)
        .bind(hasta)
        .fetch_all(pool)
        .await?;

        let (total,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM ventas WHERE user_id = $1 \
             AND ($2::DATE IS NULL OR fecha >= $2) \
             AND ($3::DATE IS NULL OR fecha <= $3)",
        )
        .bind(user_id)
        .bind(desde)
        .bind(hasta)
        .fetch_one(pool)
        .await?;

        Ok((items, total))
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM ventas WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Suma de `importe_base` de ventas en un rango de fechas
    pub async fn total_periodo(
        pool: &PgPool,
        user_id: Uuid,
        desde: chrono::NaiveDate,
        hasta: chrono::NaiveDate,
    ) -> Result<rust_decimal::Decimal, sqlx::Error> {
        let (total,): (Option<rust_decimal::Decimal>,) = sqlx::query_as(
            "SELECT COALESCE(SUM(importe_base), 0) FROM ventas \
             WHERE user_id = $1 AND fecha >= $2 AND fecha <= $3",
        )
        .bind(user_id)
        .bind(desde)
        .bind(hasta)
        .fetch_one(pool)
        .await?;
        Ok(total.unwrap_or_default())
    }
}

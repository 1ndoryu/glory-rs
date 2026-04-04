/* 253A-5: Repositorio de ventas — queries SQL con parámetros */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{Venta, VentaConCliente};

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
    /* [034A-5] Relaciones opcionales */
    pub reserva_id: Option<Uuid>,
    pub cliente_id: Option<Uuid>,
}

/* [283A-22] Datos para actualizar parcialmente una venta. */
pub struct ActualizarVentaData<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub fecha: Option<chrono::NaiveDate>,
    pub comensales: Option<i32>,
    pub descripcion: Option<&'a str>,
    pub iva_porcentaje: Option<rust_decimal::Decimal>,
    pub turno: Option<&'a str>,
    pub canal: Option<&'a str>,
    pub metodo_pago: Option<&'a str>,
    pub importe_base: Option<rust_decimal::Decimal>,
    pub importe_iva: Option<rust_decimal::Decimal>,
}

pub struct VentaRepository;

impl VentaRepository {
    pub async fn create(pool: &PgPool, data: &NuevaVenta<'_>) -> Result<Venta, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as!(
            Venta,
            "INSERT INTO ventas (id, user_id, fecha, comensales, descripcion, iva_porcentaje, \
             turno, canal, metodo_pago, importe_base, importe_iva, reserva_id, cliente_id) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) \
             RETURNING *",
            id,
            data.user_id,
            data.fecha,
            data.comensales,
            data.descripcion,
            data.iva_porcentaje,
            data.turno,
            data.canal,
            data.metodo_pago,
            data.importe_base,
            data.importe_iva,
            data.reserva_id,
            data.cliente_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Venta>, sqlx::Error> {
        sqlx::query_as!(
            Venta,
            "SELECT * FROM ventas WHERE id = $1 AND user_id = $2",
            id,
            user_id
        )
        .fetch_optional(pool)
        .await
    }

    /* [044A-8+9] Whitelist de columnas — previene SQL injection */
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        page: i64,
        per_page: i64,
        desde: Option<chrono::NaiveDate>,
        hasta: Option<chrono::NaiveDate>,
        busqueda: Option<&str>,
        sort_by: Option<&str>,
        sort_order: Option<&str>,
    ) -> Result<(Vec<VentaConCliente>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;

        /* Whitelist de columnas — previene SQL injection */
        let order_col = match sort_by {
            Some("importe_base") => "v.importe_base",
            Some("turno") => "v.turno",
            Some("canal") => "v.canal",
            Some("metodo_pago") => "v.metodo_pago",
            Some("nombre_cliente") => "nombre_cliente",
            _ => "v.fecha",
        };
        let order_dir = if matches!(sort_order, Some("asc")) { "ASC" } else { "DESC" };

        let busqueda_pattern = busqueda
            .filter(|b| !b.is_empty())
            .map(|b| format!("%{b}%"));

        let query_str = format!(
            "SELECT v.id, v.user_id, v.fecha, v.comensales, v.descripcion, \
                    v.iva_porcentaje, v.turno, v.canal, v.metodo_pago, \
                    v.importe_base, v.importe_iva, v.reserva_id, v.cliente_id, \
                    CASE WHEN c.id IS NOT NULL \
                         THEN CONCAT(c.nombre, CASE WHEN c.apellidos != '' THEN CONCAT(' ', c.apellidos) ELSE '' END) \
                         ELSE NULL \
                    END AS nombre_cliente, \
                    v.created_at, v.updated_at \
             FROM ventas v \
             LEFT JOIN clientes c ON c.id = v.cliente_id \
             WHERE v.user_id = $1 \
             AND ($4::DATE IS NULL OR v.fecha >= $4) \
             AND ($5::DATE IS NULL OR v.fecha <= $5) \
             AND ($6::TEXT IS NULL \
                  OR v.descripcion ILIKE $6 \
                  OR v.turno ILIKE $6 \
                  OR v.canal ILIKE $6 \
                  OR c.nombre ILIKE $6 \
                  OR c.apellidos ILIKE $6) \
             ORDER BY {order_col} {order_dir}, v.created_at DESC \
             LIMIT $2 OFFSET $3"
        );

        let items = sqlx::query_as::<_, VentaConCliente>(&query_str)
            .bind(user_id)
            .bind(per_page)
            .bind(offset)
            .bind(desde)
            .bind(hasta)
            .bind(busqueda_pattern.as_deref())
            .fetch_all(pool)
            .await?;

        /* COUNT con los mismos filtros (sin JOIN si no hay búsqueda para eficiencia) */
        let count = if busqueda_pattern.is_some() {
            let rec = sqlx::query_scalar::<_, Option<i64>>(
                "SELECT COUNT(*) FROM ventas v \
                 LEFT JOIN clientes c ON c.id = v.cliente_id \
                 WHERE v.user_id = $1 \
                 AND ($2::DATE IS NULL OR v.fecha >= $2) \
                 AND ($3::DATE IS NULL OR v.fecha <= $3) \
                 AND (v.descripcion ILIKE $4 \
                      OR v.turno ILIKE $4 \
                      OR v.canal ILIKE $4 \
                      OR c.nombre ILIKE $4 \
                      OR c.apellidos ILIKE $4)",
            )
            .bind(user_id)
            .bind(desde)
            .bind(hasta)
            .bind(busqueda_pattern.as_deref())
            .fetch_one(pool)
            .await?;
            rec.unwrap_or(0)
        } else {
            let rec = sqlx::query_scalar::<_, Option<i64>>(
                "SELECT COUNT(*) FROM ventas WHERE user_id = $1 \
                 AND ($2::DATE IS NULL OR fecha >= $2) \
                 AND ($3::DATE IS NULL OR fecha <= $3)",
            )
            .bind(user_id)
            .bind(desde)
            .bind(hasta)
            .fetch_one(pool)
            .await?;
            rec.unwrap_or(0)
        };

        Ok((items, count))
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM ventas WHERE id = $1 AND user_id = $2",
            id,
            user_id
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /* [283A-22] Actualizar parcialmente una venta — COALESCE mantiene valores existentes
     * cuando el campo no se envía (None).
     * [014A-11] Convertido a query_as! para verificación SQL en compilación. */
    pub async fn update(
        pool: &PgPool,
        data: &ActualizarVentaData<'_>,
    ) -> Result<Option<Venta>, sqlx::Error> {
        sqlx::query_as!(
            Venta,
            "UPDATE ventas SET \
             fecha = COALESCE($3, fecha), \
             comensales = COALESCE($4, comensales), \
             descripcion = COALESCE($5, descripcion), \
             iva_porcentaje = COALESCE($6, iva_porcentaje), \
             turno = COALESCE($7, turno), \
             canal = COALESCE($8, canal), \
             metodo_pago = COALESCE($9, metodo_pago), \
             importe_base = COALESCE($10, importe_base), \
             importe_iva = COALESCE($11, importe_iva), \
             updated_at = NOW() \
             WHERE id = $1 AND user_id = $2 \
             RETURNING *",
            data.id,
            data.user_id,
            data.fecha,
            data.comensales,
            data.descripcion,
            data.iva_porcentaje,
            data.turno,
            data.canal,
            data.metodo_pago,
            data.importe_base,
            data.importe_iva
        )
        .fetch_optional(pool)
        .await
    }

    /// Suma de `importe_base` de ventas en un rango de fechas
    pub async fn total_periodo(
        pool: &PgPool,
        user_id: Uuid,
        desde: chrono::NaiveDate,
        hasta: chrono::NaiveDate,
    ) -> Result<rust_decimal::Decimal, sqlx::Error> {
        let rec = sqlx::query!(
            "SELECT COALESCE(SUM(importe_base), 0) as total FROM ventas \
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

/* 253A-5: Repositorio de reservas */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Reserva;

/// Datos para crear una reserva
pub struct NuevaReserva<'a> {
    pub user_id: Uuid,
    pub fecha: chrono::NaiveDate,
    pub hora: chrono::NaiveTime,
    pub nombre_cliente: &'a str,
    pub num_personas: i32,
    pub estado: &'a str,
    pub notas: &'a str,
    pub telefono: &'a str,
}

/// Datos para actualizar parcialmente una reserva
pub struct ActualizarReservaData<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub fecha: Option<chrono::NaiveDate>,
    pub hora: Option<chrono::NaiveTime>,
    pub nombre_cliente: Option<&'a str>,
    pub num_personas: Option<i32>,
    pub estado: Option<&'a str>,
    pub notas: Option<&'a str>,
    pub telefono: Option<&'a str>,
}

pub struct ReservaRepository;

impl ReservaRepository {
    pub async fn create(pool: &PgPool, data: &NuevaReserva<'_>) -> Result<Reserva, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as!(
            Reserva,
            "INSERT INTO reservas (id, user_id, fecha, hora, nombre_cliente, num_personas, \
             estado, notas, telefono) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
             RETURNING *",
            id,
            data.user_id,
            data.fecha,
            data.hora,
            data.nombre_cliente,
            data.num_personas,
            data.estado,
            data.notas,
            data.telefono
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Reserva>, sqlx::Error> {
        sqlx::query_as!(
            Reserva,
            "SELECT * FROM reservas WHERE id = $1 AND user_id = $2",
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
        fecha: Option<chrono::NaiveDate>,
    ) -> Result<(Vec<Reserva>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;

        let items = sqlx::query_as!(
            Reserva,
            "SELECT * FROM reservas WHERE user_id = $1 \
             AND ($4::DATE IS NULL OR fecha = $4) \
             ORDER BY fecha ASC, hora ASC LIMIT $2 OFFSET $3",
            user_id,
            per_page,
            offset,
            fecha
        )
        .fetch_all(pool)
        .await?;

        let rec = sqlx::query!(
            "SELECT COUNT(*) as total FROM reservas WHERE user_id = $1 \
             AND ($2::DATE IS NULL OR fecha = $2)",
            user_id,
            fecha
        )
        .fetch_one(pool)
        .await?;

        Ok((items, rec.total.unwrap_or(0)))
    }

    pub async fn update(
        pool: &PgPool,
        data: &ActualizarReservaData<'_>,
    ) -> Result<Option<Reserva>, sqlx::Error> {
        sqlx::query_as!(
            Reserva,
            "UPDATE reservas SET \
             fecha = COALESCE($3, fecha), \
             hora = COALESCE($4, hora), \
             nombre_cliente = COALESCE($5, nombre_cliente), \
             num_personas = COALESCE($6, num_personas), \
             estado = COALESCE($7, estado), \
             notas = COALESCE($8, notas), \
             telefono = COALESCE($9, telefono), \
             updated_at = NOW() \
             WHERE id = $1 AND user_id = $2 \
             RETURNING *",
            data.id,
            data.user_id,
            data.fecha,
            data.hora,
            data.nombre_cliente,
            data.num_personas,
            data.estado,
            data.notas,
            data.telefono
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM reservas WHERE id = $1 AND user_id = $2",
            id,
            user_id
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Conteo de reservas del mes actual y del día actual (Audio 7)
    pub async fn conteo_actual(pool: &PgPool, user_id: Uuid) -> Result<(i64, i64), sqlx::Error> {
        let rec_mes = sqlx::query!(
            "SELECT COUNT(*) as total FROM reservas WHERE user_id = $1 \
             AND fecha >= DATE_TRUNC('month', CURRENT_DATE) \
             AND fecha < DATE_TRUNC('month', CURRENT_DATE) + INTERVAL '1 month' \
             AND estado != 'cancelada'",
            user_id
        )
        .fetch_one(pool)
        .await?;

        let rec_hoy = sqlx::query!(
            "SELECT COUNT(*) as total FROM reservas WHERE user_id = $1 \
             AND fecha = CURRENT_DATE \
             AND estado != 'cancelada'",
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok((rec_mes.total.unwrap_or(0), rec_hoy.total.unwrap_or(0)))
    }
}

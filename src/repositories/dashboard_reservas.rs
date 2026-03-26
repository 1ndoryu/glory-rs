/* [263A-13] Repositorio para el dashboard de reservas.
 * Queries SQL optimizadas para cada panel del dashboard.
 * Todas filtran por user_id y rango de fechas del mes. */

use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{
    AgrupacionCanal, AgrupacionDiaSemana, AgrupacionFecha, AgrupacionHora, AgrupacionTurno,
};

pub struct DashboardReservasRepository;

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
impl DashboardReservasRepository {
    /// Total de reservas no canceladas en un rango de fechas
    pub async fn total_reservas(
        pool: &PgPool,
        user_id: Uuid,
        desde: NaiveDate,
        hasta: NaiveDate,
    ) -> Result<i64, AppError> {
        let rec = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM reservas \
             WHERE user_id = $1 AND fecha >= $2 AND fecha <= $3 \
             AND estado NOT IN ('cancelada')",
        )
        .bind(user_id)
        .bind(desde)
        .bind(hasta)
        .fetch_one(pool)
        .await?;
        Ok(rec)
    }

    /// Reservas agrupadas por dia dentro del rango
    pub async fn por_dia(
        pool: &PgPool,
        user_id: Uuid,
        desde: NaiveDate,
        hasta: NaiveDate,
    ) -> Result<Vec<AgrupacionFecha>, AppError> {
        let rows: Vec<(NaiveDate, i64, i64)> = sqlx::query_as(
            "SELECT fecha, COUNT(*)::BIGINT, COALESCE(SUM(num_personas), 0)::BIGINT \
             FROM reservas \
             WHERE user_id = $1 AND fecha >= $2 AND fecha <= $3 \
             AND estado NOT IN ('cancelada') \
             GROUP BY fecha ORDER BY fecha",
        )
        .bind(user_id)
        .bind(desde)
        .bind(hasta)
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(f, t, p)| AgrupacionFecha {
                fecha: f.to_string(),
                total: t,
                personas: p,
            })
            .collect())
    }

    /// Reservas agrupadas por dia de la semana (0=domingo..6=sabado en PG)
    pub async fn por_dia_semana(
        pool: &PgPool,
        user_id: Uuid,
        desde: NaiveDate,
        hasta: NaiveDate,
    ) -> Result<Vec<AgrupacionDiaSemana>, AppError> {
        let rows: Vec<(f64, i64)> = sqlx::query_as(
            "SELECT EXTRACT(DOW FROM fecha)::FLOAT8, COUNT(*)::BIGINT \
             FROM reservas \
             WHERE user_id = $1 AND fecha >= $2 AND fecha <= $3 \
             AND estado NOT IN ('cancelada') \
             GROUP BY 1 ORDER BY 1",
        )
        .bind(user_id)
        .bind(desde)
        .bind(hasta)
        .fetch_all(pool)
        .await?;

        let dias = ["Domingo", "Lunes", "Martes", "Miércoles", "Jueves", "Viernes", "Sábado"];
        Ok(rows
            .into_iter()
            .map(|(dow, t)| AgrupacionDiaSemana {
                dia: dias.get(dow as usize).unwrap_or(&"?").to_string(),
                total: t,
            })
            .collect())
    }

    /// Distribucion por canal de reserva
    pub async fn por_canal(
        pool: &PgPool,
        user_id: Uuid,
        desde: NaiveDate,
        hasta: NaiveDate,
        total_general: i64,
    ) -> Result<Vec<AgrupacionCanal>, AppError> {
        let rows: Vec<(String, i64)> = sqlx::query_as(
            "SELECT COALESCE(cr.nombre, 'Sin canal'), COUNT(*)::BIGINT \
             FROM reservas r \
             LEFT JOIN canales_reserva cr ON cr.id = r.canal_id \
             WHERE r.user_id = $1 AND r.fecha >= $2 AND r.fecha <= $3 \
             AND r.estado NOT IN ('cancelada') \
             GROUP BY cr.nombre ORDER BY 2 DESC",
        )
        .bind(user_id)
        .bind(desde)
        .bind(hasta)
        .fetch_all(pool)
        .await?;

        let divisor = if total_general > 0 {
            total_general as f64
        } else {
            1.0
        };

        Ok(rows
            .into_iter()
            .map(|(canal, total)| AgrupacionCanal {
                canal,
                total,
                porcentaje: (total as f64 / divisor * 100.0 * 10.0).round() / 10.0,
            })
            .collect())
    }

    /// Clientes nuevos creados en el rango de fechas
    pub async fn clientes_nuevos(
        pool: &PgPool,
        user_id: Uuid,
        desde: NaiveDate,
        hasta: NaiveDate,
    ) -> Result<i64, AppError> {
        let rec = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM clientes \
             WHERE user_id = $1 AND created_at::date >= $2 AND created_at::date <= $3",
        )
        .bind(user_id)
        .bind(desde)
        .bind(hasta)
        .fetch_one(pool)
        .await?;
        Ok(rec)
    }

    /// Datos de ocupacion: media personas, total personas, total reservas del mes
    pub async fn datos_ocupacion(
        pool: &PgPool,
        user_id: Uuid,
        desde: NaiveDate,
        hasta: NaiveDate,
    ) -> Result<(i64, i64, f64), AppError> {
        /* Retorna (total_reservas, total_personas, media_personas) */
        let rec: (i64, i64, f64) = sqlx::query_as(
            "SELECT COUNT(*)::BIGINT, \
                    COALESCE(SUM(num_personas), 0)::BIGINT, \
                    COALESCE(AVG(num_personas)::FLOAT8, 0) \
             FROM reservas \
             WHERE user_id = $1 AND fecha >= $2 AND fecha <= $3 \
             AND estado NOT IN ('cancelada')",
        )
        .bind(user_id)
        .bind(desde)
        .bind(hasta)
        .fetch_one(pool)
        .await?;
        Ok(rec)
    }

    /// Distribucion por hora (agrupa por hora de la reserva)
    pub async fn por_hora(
        pool: &PgPool,
        user_id: Uuid,
        desde: NaiveDate,
        hasta: NaiveDate,
    ) -> Result<Vec<AgrupacionHora>, AppError> {
        let rows: Vec<(f64, i64)> = sqlx::query_as(
            "SELECT EXTRACT(HOUR FROM hora)::FLOAT8, COUNT(*)::BIGINT \
             FROM reservas \
             WHERE user_id = $1 AND fecha >= $2 AND fecha <= $3 \
             AND estado NOT IN ('cancelada') \
             GROUP BY 1 ORDER BY 1",
        )
        .bind(user_id)
        .bind(desde)
        .bind(hasta)
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(h, t)| AgrupacionHora {
                hora: format!("{:02}:00", h as u32),
                total: t,
            })
            .collect())
    }

    /// Distribucion por turno (comida: 12-18h, cena: 18-24h, desayuno: 7-12h)
    pub async fn por_turno(
        pool: &PgPool,
        user_id: Uuid,
        desde: NaiveDate,
        hasta: NaiveDate,
    ) -> Result<Vec<AgrupacionTurno>, AppError> {
        let rows: Vec<(String, i64, i64)> = sqlx::query_as(
            "SELECT \
               CASE \
                 WHEN EXTRACT(HOUR FROM hora) >= 7 AND EXTRACT(HOUR FROM hora) < 12 THEN 'Desayuno' \
                 WHEN EXTRACT(HOUR FROM hora) >= 12 AND EXTRACT(HOUR FROM hora) < 18 THEN 'Comida' \
                 ELSE 'Cena' \
               END, \
               COUNT(*)::BIGINT, \
               COALESCE(SUM(num_personas), 0)::BIGINT \
             FROM reservas \
             WHERE user_id = $1 AND fecha >= $2 AND fecha <= $3 \
             AND estado NOT IN ('cancelada') \
             GROUP BY 1 ORDER BY 1",
        )
        .bind(user_id)
        .bind(desde)
        .bind(hasta)
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(turno, total, personas)| AgrupacionTurno {
                turno,
                total,
                personas,
            })
            .collect())
    }

    /// Media de dias de antelacion con la que se hacen las reservas
    pub async fn antelacion_media(
        pool: &PgPool,
        user_id: Uuid,
        desde: NaiveDate,
        hasta: NaiveDate,
    ) -> Result<f64, AppError> {
        let rec = sqlx::query_scalar::<_, f64>(
            "SELECT COALESCE(AVG(fecha - created_at::date)::FLOAT8, 0) \
             FROM reservas \
             WHERE user_id = $1 AND fecha >= $2 AND fecha <= $3 \
             AND estado NOT IN ('cancelada')",
        )
        .bind(user_id)
        .bind(desde)
        .bind(hasta)
        .fetch_one(pool)
        .await?;
        Ok(rec)
    }

    /// Reservas efectivas (completadas, sin canceladas ni no-shows) y total comensales
    pub async fn reservas_efectivas(
        pool: &PgPool,
        user_id: Uuid,
        desde: NaiveDate,
        hasta: NaiveDate,
    ) -> Result<(i64, i64), AppError> {
        let rec: (i64, i64) = sqlx::query_as(
            "SELECT COUNT(*)::BIGINT, COALESCE(SUM(num_personas), 0)::BIGINT \
             FROM reservas \
             WHERE user_id = $1 AND fecha >= $2 AND fecha <= $3 \
             AND estado NOT IN ('cancelada', 'no_show')",
        )
        .bind(user_id)
        .bind(desde)
        .bind(hasta)
        .fetch_one(pool)
        .await?;
        Ok(rec)
    }

    /// Total ventas del mes (para ticket medio) — reutiliza la misma query
    pub async fn total_ventas_periodo(
        pool: &PgPool,
        user_id: Uuid,
        desde: NaiveDate,
        hasta: NaiveDate,
    ) -> Result<Decimal, AppError> {
        let rec = sqlx::query_scalar::<_, Decimal>(
            "SELECT COALESCE(SUM(importe_base + importe_iva), 0) \
             FROM ventas \
             WHERE user_id = $1 AND fecha >= $2 AND fecha <= $3",
        )
        .bind(user_id)
        .bind(desde)
        .bind(hasta)
        .fetch_one(pool)
        .await?;
        Ok(rec)
    }
}

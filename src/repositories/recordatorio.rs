/* [263A-25] Repositorio de reglas de recordatorio y log de envíos.
 * CRUD reglas + consulta de reservas pendientes de recordatorio +
 * registro de envíos para evitar duplicados. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    RecordatorioEnviadoDetalle, ReglaRecordatorio, ReglasPaginadas,
};

pub struct NuevaRegla {
    pub nombre: String,
    pub horas_antes: Option<i32>,
    pub canal: String,
    pub mensaje_plantilla: String,
    pub tipo: String,
    pub horas_despues: Option<i32>,
}

pub struct ActualizarReglaData {
    pub nombre: Option<String>,
    pub horas_antes: Option<i32>,
    pub canal: Option<String>,
    pub mensaje_plantilla: Option<String>,
    pub activa: Option<bool>,
    pub tipo: Option<String>,
    pub horas_despues: Option<i32>,
}

/* Reserva que necesita recordatorio: datos mínimos para el envío */
#[derive(Debug, sqlx::FromRow)]
pub struct ReservaPendienteRecordatorio {
    pub reserva_id: Uuid,
    pub regla_id: Uuid,
    pub canal: String,
    pub horas_antes: Option<i32>,
    pub horas_despues: Option<i32>,
    pub tipo: String,
    pub mensaje_plantilla: String,
    pub nombre_cliente: String,
    pub telefono: String,
    /* [303A-1] Prefijo telefónico del cliente para componer número E.164 */
    pub prefijo_telefono: String,
    pub fecha: chrono::NaiveDate,
    pub hora: chrono::NaiveTime,
    pub user_id: Uuid,
    /* Email del cliente vinculado (puede ser null si no hay cliente_id) */
    pub email_cliente: Option<String>,
}

pub struct RecordatorioRepository;

impl RecordatorioRepository {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        data: NuevaRegla,
    ) -> Result<ReglaRecordatorio, sqlx::Error> {
        sqlx::query_as::<_, ReglaRecordatorio>(
            "INSERT INTO reglas_recordatorio (user_id, nombre, horas_antes, canal, mensaje_plantilla, tipo, horas_despues) \
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
        )
        .bind(user_id)
        .bind(&data.nombre)
        .bind(data.horas_antes)
        .bind(&data.canal)
        .bind(&data.mensaje_plantilla)
        .bind(&data.tipo)
        .bind(data.horas_despues)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<ReglaRecordatorio>, sqlx::Error> {
        sqlx::query_as::<_, ReglaRecordatorio>(
            "SELECT * FROM reglas_recordatorio WHERE id = $1 AND user_id = $2",
        )
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
    ) -> Result<ReglasPaginadas, sqlx::Error> {
        let offset = (page - 1) * per_page;

        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM reglas_recordatorio WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        let items = sqlx::query_as::<_, ReglaRecordatorio>(
            "SELECT * FROM reglas_recordatorio WHERE user_id = $1 \
             ORDER BY horas_antes ASC, created_at DESC \
             LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(ReglasPaginadas { items, total, page, per_page })
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        data: ActualizarReglaData,
    ) -> Result<Option<ReglaRecordatorio>, sqlx::Error> {
        sqlx::query_as::<_, ReglaRecordatorio>(
            "UPDATE reglas_recordatorio SET \
             nombre = COALESCE($3, nombre), \
             horas_antes = COALESCE($4, horas_antes), \
             canal = COALESCE($5, canal), \
             mensaje_plantilla = COALESCE($6, mensaje_plantilla), \
             activa = COALESCE($7, activa), \
             tipo = COALESCE($8, tipo), \
             horas_despues = COALESCE($9, horas_despues), \
             updated_at = now() \
             WHERE id = $1 AND user_id = $2 RETURNING *",
        )
        .bind(id)
        .bind(user_id)
        .bind(data.nombre)
        .bind(data.horas_antes)
        .bind(data.canal)
        .bind(data.mensaje_plantilla)
        .bind(data.activa)
        .bind(data.tipo)
        .bind(data.horas_despues)
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM reglas_recordatorio WHERE id = $1 AND user_id = $2",
        )
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /* [014A-3] Busca reservas que necesitan recordatorio según las reglas activas.
     * Tipo "antes": reservas futuras cuya fecha+hora está dentro de [now, now + horas_antes].
     * Tipo "despues": reservas completadas cuya fecha+hora pasó hace <= horas_despues horas.
     * Excluye reservas ya notificadas (vía recordatorios_enviados). */
    pub async fn reservas_pendientes_recordatorio(
        pool: &PgPool,
    ) -> Result<Vec<ReservaPendienteRecordatorio>, sqlx::Error> {
        sqlx::query_as::<_, ReservaPendienteRecordatorio>(
            "SELECT \
               r.id AS reserva_id, \
               rr.id AS regla_id, \
               rr.canal, \
               rr.horas_antes, \
               rr.horas_despues, \
               rr.tipo, \
               rr.mensaje_plantilla, \
               r.nombre_cliente, \
               r.telefono, \
               COALESCE(c.prefijo_telefono, '+34') AS prefijo_telefono, \
               r.fecha, \
               r.hora, \
               r.user_id, \
               c.email AS email_cliente \
             FROM reglas_recordatorio rr \
             JOIN reservas r ON r.user_id = rr.user_id \
             LEFT JOIN clientes c ON c.id = r.cliente_id \
             WHERE rr.activa = true \
               AND NOT EXISTS ( \
                 SELECT 1 FROM recordatorios_enviados re \
                 WHERE re.regla_id = rr.id AND re.reserva_id = r.id \
               ) \
               AND ( \
                 (rr.tipo = 'antes' \
                   AND r.estado IN ('pendiente', 'confirmada', 'lista_espera') \
                   AND (r.fecha + r.hora) > now() \
                   AND (r.fecha + r.hora) <= now() + (rr.horas_antes || ' hours')::interval \
                 ) \
                 OR \
                 (rr.tipo = 'despues' \
                   AND r.estado = 'completada' \
                   AND (r.fecha + r.hora) < now() \
                   AND (r.fecha + r.hora) >= now() - (rr.horas_despues || ' hours')::interval \
                 ) \
               ) \
             ORDER BY r.fecha ASC, r.hora ASC",
        )
        .fetch_all(pool)
        .await
    }

    /* Registra un recordatorio como enviado */
    pub async fn registrar_envio(
        pool: &PgPool,
        regla_id: Uuid,
        reserva_id: Uuid,
        canal: &str,
        estado: &str,
        error_mensaje: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO recordatorios_enviados (regla_id, reserva_id, canal, estado, error_mensaje) \
             VALUES ($1, $2, $3, $4, $5) \
             ON CONFLICT (regla_id, reserva_id) DO NOTHING",
        )
        .bind(regla_id)
        .bind(reserva_id)
        .bind(canal)
        .bind(estado)
        .bind(error_mensaje)
        .execute(pool)
        .await?;

        Ok(())
    }

    /* Historial de recordatorios enviados con datos de reserva y regla */
    pub async fn historial(
        pool: &PgPool,
        user_id: Uuid,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<RecordatorioEnviadoDetalle>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;

        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM recordatorios_enviados re \
             JOIN reglas_recordatorio rr ON rr.id = re.regla_id \
             WHERE rr.user_id = $1",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        let items = sqlx::query_as::<_, RecordatorioEnviadoDetalle>(
            "SELECT \
               re.id, re.regla_id, rr.nombre AS regla_nombre, \
               re.reserva_id, r.nombre_cliente, r.fecha AS fecha_reserva, \
               r.hora AS hora_reserva, re.canal, re.estado, \
               re.enviado_at, re.error_mensaje \
             FROM recordatorios_enviados re \
             JOIN reglas_recordatorio rr ON rr.id = re.regla_id \
             JOIN reservas r ON r.id = re.reserva_id \
             WHERE rr.user_id = $1 \
             ORDER BY re.enviado_at DESC \
             LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((items, total))
    }
}

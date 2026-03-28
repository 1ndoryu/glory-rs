/* [283A-20] Repositorio de notificaciones.
 * Queries runtime (no macro) porque la migración notificaciones
 * no está en el cache .sqlx offline hasta ejecutarla. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::Notificacion;

pub struct NotificacionRepository;

impl NotificacionRepository {
    /// Inserta una notificación y devuelve la fila completa
    pub async fn crear(
        pool: &PgPool,
        user_id: Uuid,
        tipo: &str,
        titulo: &str,
        mensaje: &str,
    ) -> Result<Notificacion, AppError> {
        let row = sqlx::query_as::<_, Notificacion>(
            "INSERT INTO notificaciones (user_id, tipo, titulo, mensaje)
             VALUES ($1, $2, $3, $4)
             RETURNING id, user_id, tipo, titulo, mensaje, leida, created_at",
        )
        .bind(user_id)
        .bind(tipo)
        .bind(titulo)
        .bind(mensaje)
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    /// Lista las últimas N notificaciones de un usuario (más recientes primero)
    pub async fn listar(
        pool: &PgPool,
        user_id: Uuid,
        limite: i64,
    ) -> Result<Vec<Notificacion>, AppError> {
        let rows = sqlx::query_as::<_, Notificacion>(
            "SELECT id, user_id, tipo, titulo, mensaje, leida, created_at
             FROM notificaciones
             WHERE user_id = $1
             ORDER BY created_at DESC
             LIMIT $2",
        )
        .bind(user_id)
        .bind(limite)
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    /// Cuenta notificaciones no leídas
    pub async fn contar_no_leidas(pool: &PgPool, user_id: Uuid) -> Result<i64, AppError> {
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM notificaciones WHERE user_id = $1 AND leida = FALSE",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(row.0)
    }

    /// Marca una notificación como leída
    pub async fn marcar_leida(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query(
            "UPDATE notificaciones SET leida = TRUE WHERE id = $1 AND user_id = $2",
        )
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Notificación no encontrada".into()));
        }
        Ok(())
    }

    /// Marca todas las notificaciones de un usuario como leídas
    pub async fn marcar_todas_leidas(pool: &PgPool, user_id: Uuid) -> Result<u64, AppError> {
        let result = sqlx::query(
            "UPDATE notificaciones SET leida = TRUE WHERE user_id = $1 AND leida = FALSE",
        )
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }
}

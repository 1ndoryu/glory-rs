use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;

use crate::errors::AppError;

/* [174A-25] Repositorio de moderacion: suspension/eliminacion + bloqueos. */

pub struct ModerationRepository;

impl ModerationRepository {
    pub async fn suspend(
        pool: &PgPool,
        user_id: i32,
        razon: &str,
        hasta: Option<DateTime<Utc>>,
    ) -> Result<(), AppError> {
        let res = sqlx::query(
            "UPDATE usuarios_ext SET estado = 'suspendido', suspendido_hasta = $2, suspension_razon = $3 \
             WHERE id = $1 AND estado != 'en_eliminacion'"
        )
        .bind(user_id).bind(hasta).bind(razon)
        .execute(pool).await?;
        if res.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("usuario {user_id}")));
        }
        Ok(())
    }

    pub async fn activate(pool: &PgPool, user_id: i32) -> Result<(), AppError> {
        let res = sqlx::query(
            "UPDATE usuarios_ext SET estado = 'activo', suspendido_hasta = NULL, \
             suspension_razon = NULL, marcado_eliminacion_en = NULL, sera_eliminado_en = NULL \
             WHERE id = $1",
        )
        .bind(user_id)
        .execute(pool)
        .await?;
        if res.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("usuario {user_id}")));
        }
        Ok(())
    }

    pub async fn mark_for_deletion(
        pool: &PgPool,
        user_id: i32,
        dias_gracia: i32,
    ) -> Result<(), AppError> {
        let now = Utc::now();
        let sera = now + Duration::days(i64::from(dias_gracia));
        let res = sqlx::query(
            "UPDATE usuarios_ext SET estado = 'en_eliminacion', \
             marcado_eliminacion_en = $2, sera_eliminado_en = $3 WHERE id = $1",
        )
        .bind(user_id)
        .bind(now)
        .bind(sera)
        .execute(pool)
        .await?;
        if res.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("usuario {user_id}")));
        }
        Ok(())
    }

    pub async fn block(
        pool: &PgPool,
        bloqueador_id: i32,
        target_id: i32,
        razon: &str,
    ) -> Result<(), AppError> {
        if bloqueador_id == target_id {
            return Err(AppError::BadRequest(
                "No puedes bloquearte a ti mismo".into(),
            ));
        }
        sqlx::query(
            "INSERT INTO bloqueos (bloqueador_id, bloqueado_id, razon) \
             VALUES ($1, $2, $3) ON CONFLICT (bloqueador_id, bloqueado_id) DO UPDATE SET razon = EXCLUDED.razon"
        )
        .bind(bloqueador_id).bind(target_id).bind(razon)
        .execute(pool).await?;
        Ok(())
    }

    pub async fn unblock(
        pool: &PgPool,
        bloqueador_id: i32,
        target_id: i32,
    ) -> Result<(), AppError> {
        sqlx::query("DELETE FROM bloqueos WHERE bloqueador_id = $1 AND bloqueado_id = $2")
            .bind(bloqueador_id)
            .bind(target_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn list_blocked(pool: &PgPool, bloqueador_id: i32) -> Result<Vec<i32>, AppError> {
        let rows: Vec<(i32,)> = sqlx::query_as(
            "SELECT bloqueado_id FROM bloqueos WHERE bloqueador_id = $1 ORDER BY created_at DESC",
        )
        .bind(bloqueador_id)
        .fetch_all(pool)
        .await?;
        Ok(rows.into_iter().map(|(id,)| id).collect())
    }

    /* [174A-51] Lista de usuarios que bloquearon a `target_id`. Necesario para
     * el filtrado bidireccional del feed: el algoritmo no debe mostrar samples
     * de creadores que han bloqueado al usuario, ni de creadores bloqueados
     * por el usuario. Equivale a `BloqueosRepository::idsBloqueadores` legacy. */
    pub async fn list_blockers(pool: &PgPool, target_id: i32) -> Result<Vec<i32>, AppError> {
        let rows: Vec<(i32,)> = sqlx::query_as(
            "SELECT bloqueador_id FROM bloqueos WHERE bloqueado_id = $1 ORDER BY created_at DESC",
        )
        .bind(target_id)
        .fetch_all(pool)
        .await?;
        Ok(rows.into_iter().map(|(id,)| id).collect())
    }
}

/* 263A-9: Repositorio de canales de reserva */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::CanalReserva;

pub struct CanalReservaRepository;

impl CanalReservaRepository {
    pub async fn list(pool: &PgPool, user_id: Uuid) -> Result<Vec<CanalReserva>, sqlx::Error> {
        sqlx::query_as!(
            CanalReserva,
            "SELECT * FROM canales_reserva WHERE user_id = $1 ORDER BY nombre",
            user_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        nombre: &str,
    ) -> Result<CanalReserva, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as!(
            CanalReserva,
            "INSERT INTO canales_reserva (id, user_id, nombre) VALUES ($1, $2, $3) RETURNING *",
            id,
            user_id,
            nombre
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM canales_reserva WHERE id = $1 AND user_id = $2",
            id,
            user_id
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }
}

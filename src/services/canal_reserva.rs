/* 263A-9: Servicio de canales de reserva */

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{CanalReserva, CrearCanalReservaRequest};
use crate::repositories::CanalReservaRepository;

pub struct CanalReservaService;

impl CanalReservaService {
    pub async fn list(pool: &PgPool, user_id: Uuid) -> Result<Vec<CanalReserva>, AppError> {
        let canales = CanalReservaRepository::list(pool, user_id).await?;
        Ok(canales)
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        req: CrearCanalReservaRequest,
    ) -> Result<CanalReserva, AppError> {
        let canal = CanalReservaRepository::create(pool, user_id, &req.nombre).await?;
        Ok(canal)
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        if !CanalReservaRepository::delete(pool, id, user_id).await? {
            return Err(AppError::NotFound("Canal de reserva no encontrado".into()));
        }
        Ok(())
    }
}

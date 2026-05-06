/* [283A-23] Servicio de integraciones de marketing.
 * Orquestra lectura/escritura de credentials de terceros. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{
    ActualizarIntegracionesRequest, IntegracionMarketing, IntegracionMarketingPublica,
};
use crate::repositories::IntegracionMarketingRepository;

pub struct IntegracionMarketingService;

type Repo = IntegracionMarketingRepository;

impl IntegracionMarketingService {
    /// Obtiene vista pública de las integraciones (sin credentials)
    pub async fn obtener_publica(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<IntegracionMarketingPublica, AppError> {
        let integ = Repo::obtener_o_crear(pool, user_id).await?;
        Ok(IntegracionMarketingPublica::from(&integ))
    }

    /// Obtiene las integraciones completas (con credentials, para uso interno)
    pub async fn obtener(pool: &PgPool, user_id: Uuid) -> Result<IntegracionMarketing, AppError> {
        let integ = Repo::obtener_o_crear(pool, user_id).await?;
        Ok(integ)
    }

    /// Actualiza parcialmente las integraciones
    pub async fn actualizar(
        pool: &PgPool,
        user_id: Uuid,
        req: &ActualizarIntegracionesRequest,
    ) -> Result<IntegracionMarketingPublica, AppError> {
        Repo::obtener_o_crear(pool, user_id).await?;
        let integ = Repo::actualizar(pool, user_id, req).await?;
        Ok(IntegracionMarketingPublica::from(&integ))
    }
}

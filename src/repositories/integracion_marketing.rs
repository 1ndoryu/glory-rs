/* [283A-23] Repositorio de integraciones de marketing.
 * Upsert: si no existe, crea con defaults vacios; si existe, actualiza parcialmente.
 * Runtime queries para evitar dependencia de cache .sqlx. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{ActualizarIntegracionesRequest, IntegracionMarketing};

pub struct IntegracionMarketingRepository;

impl IntegracionMarketingRepository {
    /// Obtiene las integraciones del usuario. Si no existe, crea un registro vacío.
    pub async fn obtener_o_crear(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<IntegracionMarketing, sqlx::Error> {
        let existente = sqlx::query_as::<_, IntegracionMarketing>(
            "SELECT * FROM integraciones_marketing WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        if let Some(integ) = existente {
            return Ok(integ);
        }

        let id = Uuid::new_v4();
        sqlx::query_as::<_, IntegracionMarketing>(
            "INSERT INTO integraciones_marketing (id, user_id) VALUES ($1, $2) RETURNING *",
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    /// Actualiza parcialmente las integraciones del usuario.
    pub async fn actualizar(
        pool: &PgPool,
        user_id: Uuid,
        req: &ActualizarIntegracionesRequest,
    ) -> Result<IntegracionMarketing, sqlx::Error> {
        sqlx::query_as::<_, IntegracionMarketing>(
            r"UPDATE integraciones_marketing SET
                smtp_host = COALESCE($2, smtp_host),
                smtp_port = COALESCE($3, smtp_port),
                smtp_user = COALESCE($4, smtp_user),
                smtp_password = COALESCE($5, smtp_password),
                smtp_from_email = COALESCE($6, smtp_from_email),
                smtp_from_name = COALESCE($7, smtp_from_name),
                twilio_account_sid = COALESCE($8, twilio_account_sid),
                twilio_auth_token = COALESCE($9, twilio_auth_token),
                twilio_from_number = COALESCE($10, twilio_from_number),
                meta_waba_id = COALESCE($11, meta_waba_id),
                meta_business_app_id = COALESCE($12, meta_business_app_id),
                meta_access_token = COALESCE($13, meta_access_token),
                meta_phone_number_id = COALESCE($14, meta_phone_number_id),
                updated_at = now()
            WHERE user_id = $1 RETURNING *",
        )
        .bind(user_id)
        .bind(&req.smtp_host)
        .bind(req.smtp_port)
        .bind(&req.smtp_user)
        .bind(&req.smtp_password)
        .bind(&req.smtp_from_email)
        .bind(&req.smtp_from_name)
        .bind(&req.twilio_account_sid)
        .bind(&req.twilio_auth_token)
        .bind(&req.twilio_from_number)
        .bind(&req.meta_waba_id)
        .bind(&req.meta_business_app_id)
        .bind(&req.meta_access_token)
        .bind(&req.meta_phone_number_id)
        .fetch_one(pool)
        .await
    }
}

/* sentinel-disable-file sqlx-query-sin-macro: audit usa runtime query (fire-and-forget INSERT). */
/* [064A-73] Servicio de auditoría: registra eventos de seguridad en BD.
 * Diseñado para ser fire-and-forget: loguea si falla pero no bloquea el request. */

use sqlx::PgPool;
use uuid::Uuid;

pub struct AuditService;

impl AuditService {
    /* Registra un evento de auditoría. No propaga errores — loguea y sigue.
     * Esto es intencional: un fallo en auditoría no debe bloquear la operación. */
    pub async fn log(
        pool: &PgPool,
        event_type: &str,
        user_id: Option<Uuid>,
        ip_address: Option<&str>,
        details: serde_json::Value,
    ) {
        let result = sqlx::query(
            "INSERT INTO audit_log (event_type, user_id, ip_address, details) VALUES ($1, $2, $3, $4)"
        )
        .bind(event_type)
        .bind(user_id)
        .bind(ip_address)
        .bind(&details)
        .execute(pool)
        .await;

        if let Err(e) = result {
            tracing::error!("Error registrando audit log [{event_type}]: {e}");
        }
    }
}

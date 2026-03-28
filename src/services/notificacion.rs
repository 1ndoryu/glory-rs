/* [283A-20] Servicio de notificaciones.
 * Crea la notificación en BD y la emite por el canal broadcast
 * para que los clientes SSE conectados la reciban en tiempo real. */

use sqlx::PgPool;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{Notificacion, NotificacionEvent};
use crate::repositories::NotificacionRepository;

pub struct NotificacionService;

impl NotificacionService {
    /// Crea una notificación y la emite por el canal SSE.
    /// Si no hay suscriptores SSE, la notificación se persiste igualmente en BD.
    pub async fn emitir(
        pool: &PgPool,
        tx: &broadcast::Sender<NotificacionEvent>,
        user_id: Uuid,
        tipo: &str,
        titulo: &str,
        mensaje: &str,
    ) -> Result<Notificacion, AppError> {
        let notif = NotificacionRepository::crear(pool, user_id, tipo, titulo, mensaje).await?;

        /* Ignorar error de send: significa que no hay receptores SSE conectados,
         * pero la notificación ya está persistida en BD */
        let _ = tx.send(NotificacionEvent {
            user_id,
            notificacion: notif.clone(),
        });

        Ok(notif)
    }

    pub async fn listar(
        pool: &PgPool,
        user_id: Uuid,
        limite: i64,
    ) -> Result<Vec<Notificacion>, AppError> {
        NotificacionRepository::listar(pool, user_id, limite).await
    }

    pub async fn contar_no_leidas(pool: &PgPool, user_id: Uuid) -> Result<i64, AppError> {
        NotificacionRepository::contar_no_leidas(pool, user_id).await
    }

    pub async fn marcar_leida(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        NotificacionRepository::marcar_leida(pool, id, user_id).await
    }

    pub async fn marcar_todas_leidas(pool: &PgPool, user_id: Uuid) -> Result<u64, AppError> {
        NotificacionRepository::marcar_todas_leidas(pool, user_id).await
    }
}

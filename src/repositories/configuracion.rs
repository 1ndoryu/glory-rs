/* [263A-17] Repositorio de configuración del restaurante.
 * Upsert: si no existe, crea con defaults; si existe, actualiza parcialmente. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{ActualizarConfiguracionRequest, ConfiguracionRestaurante};

pub struct ConfiguracionRepository;

impl ConfiguracionRepository {
    /// Obtiene la configuración del usuario. Si no existe, crea una con defaults.
    pub async fn obtener_o_crear(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<ConfiguracionRestaurante, sqlx::Error> {
        let existente = sqlx::query_as!(
            ConfiguracionRestaurante,
            "SELECT * FROM configuracion_restaurante WHERE user_id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(config) = existente {
            return Ok(config);
        }

        /* Crear con defaults */
        let id = Uuid::new_v4();
        sqlx::query_as!(
            ConfiguracionRestaurante,
            r#"INSERT INTO configuracion_restaurante (id, user_id)
               VALUES ($1, $2) RETURNING *"#,
            id,
            user_id
        )
        .fetch_one(pool)
        .await
    }

    /// Actualiza parcialmente la configuración del usuario.
    pub async fn actualizar(
        pool: &PgPool,
        user_id: Uuid,
        req: &ActualizarConfiguracionRequest,
    ) -> Result<ConfiguracionRestaurante, sqlx::Error> {
        sqlx::query_as!(
            ConfiguracionRestaurante,
            r#"UPDATE configuracion_restaurante SET
                reserva_email_obligatorio = COALESCE($2, reserva_email_obligatorio),
                reserva_telefono_obligatorio = COALESCE($3, reserva_telefono_obligatorio),
                reserva_nombre_obligatorio = COALESCE($4, reserva_nombre_obligatorio),
                reserva_apellidos_obligatorio = COALESCE($5, reserva_apellidos_obligatorio),
                iva_por_defecto = COALESCE($6, iva_por_defecto),
                nombre_restaurante = COALESCE($7, nombre_restaurante),
                updated_at = NOW()
               WHERE user_id = $1 RETURNING *"#,
            user_id,
            req.reserva_email_obligatorio,
            req.reserva_telefono_obligatorio,
            req.reserva_nombre_obligatorio,
            req.reserva_apellidos_obligatorio,
            req.iva_por_defecto,
            req.nombre_restaurante.as_deref()
        )
        .fetch_one(pool)
        .await
    }
}

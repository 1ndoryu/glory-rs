/* [263A-17] Repositorio de configuración del restaurante.
 * Upsert: si no existe, crea con defaults; si existe, actualiza parcialmente.
 * [014A-11] Convertido a query_as! para verificación SQL en compilación. */

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
            "INSERT INTO configuracion_restaurante (id, user_id) VALUES ($1, $2) RETURNING *",
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
            r"UPDATE configuracion_restaurante SET
                reserva_email_obligatorio = COALESCE($2, reserva_email_obligatorio),
                reserva_telefono_obligatorio = COALESCE($3, reserva_telefono_obligatorio),
                reserva_nombre_obligatorio = COALESCE($4, reserva_nombre_obligatorio),
                reserva_apellidos_obligatorio = COALESCE($5, reserva_apellidos_obligatorio),
                iva_por_defecto = COALESCE($6, iva_por_defecto),
                nombre_restaurante = COALESCE($7, nombre_restaurante),
                groq_api_key = COALESCE($8, groq_api_key),
                auto_venta_reserva = COALESCE($9, auto_venta_reserva),
                hora_desayuno_inicio = COALESCE($10, hora_desayuno_inicio),
                hora_desayuno_fin = COALESCE($11, hora_desayuno_fin),
                hora_comida_inicio = COALESCE($12, hora_comida_inicio),
                hora_comida_fin = COALESCE($13, hora_comida_fin),
                hora_cena_inicio = COALESCE($14, hora_cena_inicio),
                hora_cena_fin = COALESCE($15, hora_cena_fin),
                url_haddock = COALESCE($16, url_haddock),
                haddock_api_token = COALESCE($17, haddock_api_token),
                haddock_sync_enabled = COALESCE($18, haddock_sync_enabled),
                updated_at = NOW()
               WHERE user_id = $1 RETURNING *",
            user_id,
            req.reserva_email_obligatorio,
            req.reserva_telefono_obligatorio,
            req.reserva_nombre_obligatorio,
            req.reserva_apellidos_obligatorio,
            req.iva_por_defecto,
            req.nombre_restaurante.as_deref(),
            req.groq_api_key.as_deref(),
            req.auto_venta_reserva,
            req.hora_desayuno_inicio,
            req.hora_desayuno_fin,
            req.hora_comida_inicio,
            req.hora_comida_fin,
            req.hora_cena_inicio,
            req.hora_cena_fin,
            req.url_haddock.as_deref(),
            req.haddock_api_token.as_deref(),
            req.haddock_sync_enabled
        )
        .fetch_one(pool)
        .await
    }
}

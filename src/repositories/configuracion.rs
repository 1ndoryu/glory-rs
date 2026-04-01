/* [263A-17] Repositorio de configuración del restaurante.
 * Upsert: si no existe, crea con defaults; si existe, actualiza parcialmente.
 * [283A-8] Queries convertidas a runtime (no macro) porque groq_api_key no existe
 * en la cache offline .sqlx/ hasta ejecutar migración + cargo sqlx prepare. */

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
        let existente = sqlx::query_as::<_, ConfiguracionRestaurante>(
            "SELECT * FROM configuracion_restaurante WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        if let Some(config) = existente {
            return Ok(config);
        }

        /* Crear con defaults */
        let id = Uuid::new_v4();
        sqlx::query_as::<_, ConfiguracionRestaurante>(
            "INSERT INTO configuracion_restaurante (id, user_id) VALUES ($1, $2) RETURNING *",
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    /// Actualiza parcialmente la configuración del usuario.
    pub async fn actualizar(
        pool: &PgPool,
        user_id: Uuid,
        req: &ActualizarConfiguracionRequest,
    ) -> Result<ConfiguracionRestaurante, sqlx::Error> {
        sqlx::query_as::<_, ConfiguracionRestaurante>(
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
                updated_at = NOW()
               WHERE user_id = $1 RETURNING *",
        )
        .bind(user_id)
        .bind(req.reserva_email_obligatorio)
        .bind(req.reserva_telefono_obligatorio)
        .bind(req.reserva_nombre_obligatorio)
        .bind(req.reserva_apellidos_obligatorio)
        .bind(req.iva_por_defecto)
        .bind(req.nombre_restaurante.as_deref())
        .bind(req.groq_api_key.as_deref())
        .bind(req.auto_venta_reserva)
        .bind(req.hora_desayuno_inicio)
        .bind(req.hora_desayuno_fin)
        .bind(req.hora_comida_inicio)
        .bind(req.hora_comida_fin)
        .bind(req.hora_cena_inicio)
        .bind(req.hora_cena_fin)
        .fetch_one(pool)
        .await
    }
}

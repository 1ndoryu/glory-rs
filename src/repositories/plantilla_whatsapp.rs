/* [263A-24] Repositorio de plantillas WhatsApp.
 * CRUD básico + cambio de estado para flujo de aprobación Meta. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{PlantillaWhatsapp, PlantillasPaginadas};

pub struct NuevaPlantilla {
    pub user_id: Uuid,
    pub nombre: String,
    pub categoria: String,
    pub idioma: String,
    pub cuerpo_mensaje: String,
    pub cabecera_texto: Option<String>,
    pub pie_texto: Option<String>,
    pub cabecera_media_url: Option<String>,
    pub cabecera_media_tipo: Option<String>,
}

pub struct ActualizarPlantillaData {
    pub nombre: Option<String>,
    pub categoria: Option<String>,
    pub idioma: Option<String>,
    pub cuerpo_mensaje: Option<String>,
    pub cabecera_texto: Option<String>,
    pub pie_texto: Option<String>,
    pub cabecera_media_url: Option<String>,
    pub cabecera_media_tipo: Option<String>,
}

pub struct PlantillaRepository;

impl PlantillaRepository {
    pub async fn create(pool: &PgPool, p: NuevaPlantilla) -> Result<PlantillaWhatsapp, sqlx::Error> {
        sqlx::query_as::<_, PlantillaWhatsapp>(
            "INSERT INTO plantillas_whatsapp \
             (user_id, nombre, categoria, idioma, cuerpo_mensaje, cabecera_texto, \
              pie_texto, cabecera_media_url, cabecera_media_tipo) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
             RETURNING *",
        )
        .bind(p.user_id)
        .bind(&p.nombre)
        .bind(&p.categoria)
        .bind(&p.idioma)
        .bind(&p.cuerpo_mensaje)
        .bind(&p.cabecera_texto)
        .bind(&p.pie_texto)
        .bind(&p.cabecera_media_url)
        .bind(&p.cabecera_media_tipo)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Option<PlantillaWhatsapp>, sqlx::Error> {
        sqlx::query_as::<_, PlantillaWhatsapp>(
            "SELECT * FROM plantillas_whatsapp WHERE id = $1 AND user_id = $2",
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(pool)
        .await
    }

    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        page: i64,
        per_page: i64,
        estado: Option<&str>,
    ) -> Result<PlantillasPaginadas, sqlx::Error> {
        let offset = (page - 1) * per_page;

        let (items, total) = if let Some(est) = estado {
            let items = sqlx::query_as::<_, PlantillaWhatsapp>(
                "SELECT * FROM plantillas_whatsapp \
                 WHERE user_id = $1 AND estado = $2 \
                 ORDER BY created_at DESC LIMIT $3 OFFSET $4",
            )
            .bind(user_id)
            .bind(est)
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?;

            let total: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM plantillas_whatsapp WHERE user_id = $1 AND estado = $2",
            )
            .bind(user_id)
            .bind(est)
            .fetch_one(pool)
            .await?;

            (items, total)
        } else {
            let items = sqlx::query_as::<_, PlantillaWhatsapp>(
                "SELECT * FROM plantillas_whatsapp \
                 WHERE user_id = $1 \
                 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            )
            .bind(user_id)
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?;

            let total: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM plantillas_whatsapp WHERE user_id = $1",
            )
            .bind(user_id)
            .fetch_one(pool)
            .await?;

            (items, total)
        };

        Ok(PlantillasPaginadas { items, total, page, per_page })
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        data: ActualizarPlantillaData,
    ) -> Result<Option<PlantillaWhatsapp>, sqlx::Error> {
        sqlx::query_as::<_, PlantillaWhatsapp>(
            "UPDATE plantillas_whatsapp SET \
             nombre = COALESCE($3, nombre), \
             categoria = COALESCE($4, categoria), \
             idioma = COALESCE($5, idioma), \
             cuerpo_mensaje = COALESCE($6, cuerpo_mensaje), \
             cabecera_texto = COALESCE($7, cabecera_texto), \
             pie_texto = COALESCE($8, pie_texto), \
             cabecera_media_url = COALESCE($9, cabecera_media_url), \
             cabecera_media_tipo = COALESCE($10, cabecera_media_tipo), \
             updated_at = NOW() \
             WHERE id = $1 AND user_id = $2 AND estado = 'borrador' \
             RETURNING *",
        )
        .bind(id)
        .bind(user_id)
        .bind(&data.nombre)
        .bind(&data.categoria)
        .bind(&data.idioma)
        .bind(&data.cuerpo_mensaje)
        .bind(&data.cabecera_texto)
        .bind(&data.pie_texto)
        .bind(&data.cabecera_media_url)
        .bind(&data.cabecera_media_tipo)
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM plantillas_whatsapp WHERE id = $1 AND user_id = $2",
        )
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /* Marcar como enviada a Meta (cuando se envía la solicitud de aprobación) */
    pub async fn set_enviada(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        meta_template_id: &str,
    ) -> Result<Option<PlantillaWhatsapp>, sqlx::Error> {
        sqlx::query_as::<_, PlantillaWhatsapp>(
            "UPDATE plantillas_whatsapp SET \
             estado = 'enviada', \
             meta_template_id = $3, \
             meta_enviada_at = NOW(), \
             updated_at = NOW() \
             WHERE id = $1 AND user_id = $2 AND estado = 'borrador' \
             RETURNING *",
        )
        .bind(id)
        .bind(user_id)
        .bind(meta_template_id)
        .fetch_optional(pool)
        .await
    }

    /* Callback de Meta: aprobar o rechazar plantilla */
    pub async fn set_resultado_meta(
        pool: &PgPool,
        id: Uuid,
        aprobada: bool,
        razon_rechazo: Option<&str>,
    ) -> Result<Option<PlantillaWhatsapp>, sqlx::Error> {
        let estado = if aprobada { "aprobada" } else { "rechazada" };
        sqlx::query_as::<_, PlantillaWhatsapp>(
            "UPDATE plantillas_whatsapp SET \
             estado = $2, \
             meta_razon_rechazo = $3, \
             meta_respondida_at = NOW(), \
             updated_at = NOW() \
             WHERE id = $1 AND estado = 'enviada' \
             RETURNING *",
        )
        .bind(id)
        .bind(estado)
        .bind(razon_rechazo)
        .fetch_optional(pool)
        .await
    }
}

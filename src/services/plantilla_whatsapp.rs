/* [263A-24] Servicio de plantillas WhatsApp.
 * Orquesta CRUD y flujo de envío a Meta Business API.
 * La integración real con Meta se implementará cuando haya credenciales. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{
    ActualizarPlantillaRequest, CrearPlantillaRequest, PlantillaWhatsapp,
    PlantillasPaginadas, PlantillasQuery, CATEGORIAS_PLANTILLA,
};
use crate::repositories::plantilla_whatsapp::{
    ActualizarPlantillaData, NuevaPlantilla, PlantillaRepository,
};

pub struct PlantillaService;

impl PlantillaService {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        req: CrearPlantillaRequest,
    ) -> Result<PlantillaWhatsapp, AppError> {
        let categoria = req.categoria.unwrap_or_else(|| "MARKETING".to_string());
        validar_categoria(&categoria)?;

        let idioma = req.idioma.unwrap_or_else(|| "es".to_string());

        let plantilla = PlantillaRepository::create(
            pool,
            NuevaPlantilla {
                user_id,
                nombre: req.nombre,
                categoria,
                idioma,
                cuerpo_mensaje: req.cuerpo_mensaje.unwrap_or_default(),
                cabecera_texto: req.cabecera_texto,
                pie_texto: req.pie_texto,
                cabecera_media_url: req.cabecera_media_url,
                cabecera_media_tipo: req.cabecera_media_tipo,
            },
        )
        .await
        .map_err(AppError::Database)?;

        Ok(plantilla)
    }

    pub async fn get(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<PlantillaWhatsapp, AppError> {
        PlantillaRepository::find_by_id(pool, id, user_id)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::NotFound("Plantilla no encontrada".into()))
    }

    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        query: PlantillasQuery,
    ) -> Result<PlantillasPaginadas, AppError> {
        let page = query.page.unwrap_or(1).max(1);
        let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

        PlantillaRepository::list(pool, user_id, page, per_page, query.estado.as_deref())
            .await
            .map_err(AppError::Database)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        req: ActualizarPlantillaRequest,
    ) -> Result<PlantillaWhatsapp, AppError> {
        if let Some(ref cat) = req.categoria {
            validar_categoria(cat)?;
        }

        PlantillaRepository::update(
            pool,
            id,
            user_id,
            ActualizarPlantillaData {
                nombre: req.nombre,
                categoria: req.categoria,
                idioma: req.idioma,
                cuerpo_mensaje: req.cuerpo_mensaje,
                cabecera_texto: req.cabecera_texto,
                pie_texto: req.pie_texto,
                cabecera_media_url: req.cabecera_media_url,
                cabecera_media_tipo: req.cabecera_media_tipo,
            },
        )
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| {
            AppError::Validation("No se pudo actualizar: solo se editan borradores".into())
        })
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let deleted = PlantillaRepository::delete(pool, id, user_id)
            .await
            .map_err(AppError::Database)?;
        if !deleted {
            return Err(AppError::NotFound("Plantilla no encontrada".into()));
        }
        Ok(())
    }

    /* Enviar plantilla a Meta para aprobación.
     * Por ahora genera un ID stub — integración real pendiente de credenciales Meta. */
    pub async fn enviar_a_meta(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<PlantillaWhatsapp, AppError> {
        /* Verificar que existe y es borrador */
        let plantilla = Self::get(pool, id, user_id).await?;
        if plantilla.estado != "borrador" {
            return Err(AppError::Validation(
                "Solo se pueden enviar plantillas en estado borrador".into(),
            ));
        }
        if plantilla.cuerpo_mensaje.is_empty() {
            return Err(AppError::Validation(
                "El cuerpo del mensaje no puede estar vacío".into(),
            ));
        }

        /* Stub: en producción se llamaría a Meta Business API aquí.
         * POST https://graph.facebook.com/v21.0/{WABA_ID}/message_templates
         * El ID retornado se almacena como meta_template_id. */
        let stub_meta_id = format!("meta_stub_{}", Uuid::new_v4());
        tracing::info!(
            plantilla_id = %id,
            meta_id = %stub_meta_id,
            "Plantilla enviada a Meta (stub — integración pendiente)"
        );

        PlantillaRepository::set_enviada(pool, id, user_id, &stub_meta_id)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::Validation("No se pudo enviar: verificar estado".into()))
    }
}

fn validar_categoria(cat: &str) -> Result<(), AppError> {
    if !CATEGORIAS_PLANTILLA.contains(&cat) {
        return Err(AppError::Validation(format!(
            "Categoría inválida: '{cat}'. Válidas: {}",
            CATEGORIAS_PLANTILLA.join(", ")
        )));
    }
    Ok(())
}

/* [263A-24] Servicio de plantillas WhatsApp.
 * Orquesta CRUD y flujo de envío a Meta Business API.
 * [303A-1] enviar_a_meta implementado con llamada real a Meta Graph API v23.0.
 * POST /{WABA_ID}/message_templates (usa WABA ID, no Phone-Number-ID). */

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
use crate::repositories::IntegracionMarketingRepository;

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

    /* [303A-1] Enviar plantilla a Meta Business API para aprobación.
     * POST https://graph.facebook.com/v23.0/{WABA_ID}/message_templates
     * Si Meta no está configurado, retorna error claro en vez de ID falso. */
    pub async fn enviar_a_meta(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<PlantillaWhatsapp, AppError> {
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

        let integ = IntegracionMarketingRepository::obtener_o_crear(pool, user_id).await
            .map_err(AppError::Database)?;

        if !integ.meta_templates_configurado() {
            return Err(AppError::Validation(
                "Meta WhatsApp no configurado. Configure WABA ID y Access Token en Configuración > Integraciones".into(),
            ));
        }

        let waba_id = integ.meta_waba_id.as_deref().unwrap_or_default();
        let access_token = integ.meta_access_token.as_deref().unwrap_or_default();

        let meta_template_id = enviar_template_a_meta(
            waba_id, access_token, &plantilla,
        ).await?;

        PlantillaRepository::set_enviada(pool, id, user_id, &meta_template_id)
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

/* [303A-1] Envía la plantilla a Meta para aprobación via Graph API v23.0.
 * POST https://graph.facebook.com/v23.0/{WABA_ID}/message_templates
 * Retorna el template ID asignado por Meta. */
async fn enviar_template_a_meta(
    waba_id: &str,
    access_token: &str,
    plantilla: &PlantillaWhatsapp,
) -> Result<String, AppError> {
    let url = format!(
        "https://graph.facebook.com/v23.0/{waba_id}/message_templates"
    );

    /* Construir componentes: al menos un BODY es obligatorio */
    let mut components = vec![serde_json::json!({
        "type": "BODY",
        "text": plantilla.cuerpo_mensaje
    })];

    if let Some(ref cabecera) = plantilla.cabecera_texto {
        if !cabecera.is_empty() {
            components.push(serde_json::json!({
                "type": "HEADER",
                "format": "TEXT",
                "text": cabecera
            }));
        }
    }

    if let Some(ref pie) = plantilla.pie_texto {
        if !pie.is_empty() {
            components.push(serde_json::json!({
                "type": "FOOTER",
                "text": pie
            }));
        }
    }

    let payload = serde_json::json!({
        "name": plantilla.nombre.to_lowercase().replace(' ', "_"),
        "category": plantilla.categoria,
        "language": plantilla.idioma,
        "components": components
    });

    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .bearer_auth(access_token)
        .json(&payload)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Error HTTP Meta: {e}")))?;

    let status = resp.status().as_u16();
    let body = resp.text().await.unwrap_or_default();

    if status >= 400 {
        tracing::error!("Meta template API error {status}: {body}");
        return Err(AppError::BadRequest(format!(
            "Meta rechazó la plantilla ({status}): {body}"
        )));
    }

    /* Parsear respuesta: { "id": "123456789", "status": "PENDING", "category": "..." } */
    let parsed: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| AppError::Internal(format!("Respuesta Meta no válida: {e}")))?;

    let meta_id = parsed["id"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();

    tracing::info!(
        plantilla = %plantilla.nombre,
        meta_id = %meta_id,
        "Plantilla enviada a Meta para aprobación"
    );

    Ok(meta_id)
}

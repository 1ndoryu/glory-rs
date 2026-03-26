/* [263A-23] Servicio de campañas de marketing.
 * Lógica de negocio: validación de canales/segmentos, segmentación,
 * generación de destinatarios y envío (stub por ahora — APIs externas pendientes).
 * El envío real de SMS/WhatsApp requiere integración con gateway externo. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{
    ActualizarCampanaRequest, Campana, CampanasPaginadas, CampanasQuery, CrearCampanaRequest,
    SegmentoPreview, CANALES_VALIDOS, SEGMENTOS_VALIDOS,
};
use crate::repositories::campana::{
    ActualizarCampanaData, NuevaCampana, NuevoDestinatario,
};
use crate::repositories::CampanaRepository;

pub struct CampanaService;

impl CampanaService {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        req: CrearCampanaRequest,
    ) -> Result<Campana, AppError> {
        Self::validar_canales(&req.canales)?;
        let segmento = req.segmento.as_deref().unwrap_or("todos");
        Self::validar_segmento(segmento)?;

        let data = NuevaCampana {
            user_id,
            nombre: &req.nombre,
            descripcion_interna: req.descripcion_interna.as_deref().unwrap_or(""),
            cuerpo_mensaje: req.cuerpo_mensaje.as_deref().unwrap_or(""),
            canales: &req.canales,
            segmento,
            incluir_baja: req.incluir_baja.unwrap_or(false),
            telefono_baja: req.telefono_baja.as_deref().unwrap_or(""),
        };

        let campana = CampanaRepository::create(pool, &data).await?;
        Ok(campana)
    }

    pub async fn get(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Campana, AppError> {
        CampanaRepository::find_by_id(pool, id, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Campaña no encontrada".into()))
    }

    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        query: CampanasQuery,
    ) -> Result<CampanasPaginadas, AppError> {
        let page = query.page;
        let per_page = query.per_page;
        let (items, total) =
            CampanaRepository::list(pool, user_id, page, per_page, query.estado.as_deref())
                .await?;
        Ok(CampanasPaginadas {
            items,
            total,
            page,
            per_page,
        })
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        req: ActualizarCampanaRequest,
    ) -> Result<Campana, AppError> {
        if let Some(ref canales) = req.canales {
            Self::validar_canales(canales)?;
        }
        if let Some(ref segmento) = req.segmento {
            Self::validar_segmento(segmento)?;
        }

        let data = ActualizarCampanaData {
            id,
            user_id,
            nombre: req.nombre.as_deref(),
            descripcion_interna: req.descripcion_interna.as_deref(),
            cuerpo_mensaje: req.cuerpo_mensaje.as_deref(),
            canales: req.canales.as_deref(),
            segmento: req.segmento.as_deref(),
            incluir_baja: req.incluir_baja,
            telefono_baja: req.telefono_baja.as_deref(),
        };

        CampanaRepository::update(pool, &data)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(
                    "Campaña no encontrada o no editable (solo borradores)".into(),
                )
            })
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        if !CampanaRepository::delete(pool, id, user_id).await? {
            return Err(AppError::NotFound("Campaña no encontrada".into()));
        }
        Ok(())
    }

    pub async fn preview_segmento(
        pool: &PgPool,
        user_id: Uuid,
        segmento: &str,
    ) -> Result<SegmentoPreview, AppError> {
        Self::validar_segmento(segmento)?;
        let preview = CampanaRepository::segmento_preview(pool, user_id, segmento).await?;
        Ok(preview)
    }

    /* [263A-23] Enviar campaña: segmentar clientes, crear destinatarios, enviar.
     * El envío real es un stub que loguea — las APIs (SMS gateway, SMTP, WhatsApp)
     * se integrarán cuando se configuren las credenciales del proveedor. */
    pub async fn enviar(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Campana, AppError> {
        let campana = Self::get(pool, id, user_id).await?;

        if campana.estado != "borrador" {
            return Err(AppError::Validation(
                "Solo se pueden enviar campañas en estado borrador".into(),
            ));
        }

        if campana.canales.is_empty() {
            return Err(AppError::Validation(
                "La campaña debe tener al menos un canal".into(),
            ));
        }

        if campana.cuerpo_mensaje.is_empty() {
            return Err(AppError::Validation(
                "La campaña debe tener un cuerpo de mensaje".into(),
            ));
        }

        /* Obtener clientes del segmento con consentimiento para los canales */
        let clientes = CampanaRepository::clientes_segmento(
            pool,
            user_id,
            &campana.segmento,
            &campana.canales,
        )
        .await?;

        if clientes.is_empty() {
            return Err(AppError::Validation(
                "No hay clientes que cumplan los criterios de segmentación y consentimiento".into(),
            ));
        }

        /* Generar destinatarios: un registro por cliente por canal aplicable */
        let mut destinatarios: Vec<NuevoDestinatario> = Vec::new();
        for cliente in &clientes {
            for canal in &campana.canales {
                let aplica = match canal.as_str() {
                    "sms" | "whatsapp" => {
                        cliente.consentimiento_comercial_sms && !cliente.telefono.is_empty()
                    }
                    "email" => {
                        cliente.consentimiento_comercial_email && !cliente.email.is_empty()
                    }
                    _ => false,
                };
                if aplica {
                    destinatarios.push(NuevoDestinatario {
                        cliente_id: cliente.id,
                        canal: canal.clone(),
                    });
                }
            }
        }

        let total = i32::try_from(destinatarios.len()).unwrap_or(0);
        CampanaRepository::insertar_destinatarios(pool, id, &destinatarios).await?;

        /* Stub de envío: logueamos en vez de enviar realmente.
         * Cuando se configure SMS gateway / SMTP / WhatsApp Business API,
         * aquí se iterarían los destinatarios y se enviaría por canal. */
        tracing::info!(
            "Campaña '{}' ({}) — {} destinatarios generados en {} canales. \
             Envío real pendiente de integración con providers.",
            campana.nombre,
            id,
            total,
            campana.canales.join(", ")
        );

        let campana_actualizada =
            CampanaRepository::set_estado(pool, id, user_id, "enviada", total)
                .await?
                .ok_or_else(|| AppError::NotFound("Error actualizando estado".into()))?;

        Ok(campana_actualizada)
    }

    fn validar_canales(canales: &[String]) -> Result<(), AppError> {
        for canal in canales {
            if !CANALES_VALIDOS.contains(&canal.as_str()) {
                return Err(AppError::Validation(format!(
                    "Canal inválido: '{}'. Válidos: {}",
                    canal,
                    CANALES_VALIDOS.join(", ")
                )));
            }
        }
        Ok(())
    }

    fn validar_segmento(segmento: &str) -> Result<(), AppError> {
        if !SEGMENTOS_VALIDOS.contains(&segmento) {
            return Err(AppError::Validation(format!(
                "Segmento inválido: '{}'. Válidos: {}",
                segmento,
                SEGMENTOS_VALIDOS.join(", ")
            )));
        }
        Ok(())
    }
}

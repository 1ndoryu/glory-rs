/* [263A-23] Servicio de campañas de marketing.
 * Lógica de negocio: validación de canales/segmentos, segmentación,
 * generación de destinatarios y envío.
 * [283A-23] Email real via SMTP (integraciones_marketing). SMS/WhatsApp pendientes. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{
    ActualizarCampanaRequest, Campana, CampanasPaginadas, CampanasQuery, CrearCampanaRequest,
    SegmentoPreview, CANALES_VALIDOS, SEGMENTOS_VALIDOS,
};
use crate::repositories::campana::{
    ActualizarCampanaData, ClienteSegmentado, NuevaCampana, NuevoDestinatario,
};
use crate::models::IntegracionMarketing;
use crate::repositories::CampanaRepository;
use crate::repositories::IntegracionMarketingRepository;
use crate::services::email::EmailService;

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
     * [303A-1] Ahora actualiza contadores reales (total_enviados, total_fallidos)
     * y el estado individual de cada destinatario. */
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

        /* [283A-23] Envío real delegado a función libre para cumplir límite de líneas.
         * [303A-1] Ahora retorna contadores reales y actualiza cada destinatario. */
        let integ_mkt = IntegracionMarketingRepository::obtener_o_crear(pool, user_id).await?;
        let (enviados, fallidos) = enviar_por_canales(pool, id, &integ_mkt, &clientes, &campana).await;

        tracing::info!("Campaña '{}' ({}) — {} destinatarios, {} enviados, {} fallidos",
            campana.nombre, id, total, enviados, fallidos);

        let total_enviados = i32::try_from(enviados).unwrap_or(0);
        let total_fallidos = i32::try_from(fallidos).unwrap_or(0);

        let campana_actualizada =
            CampanaRepository::set_estado(pool, id, user_id, "enviada", total, total_enviados, total_fallidos)
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

use crate::services::meta_whatsapp::MetaWhatsappService;
use crate::services::twilio::TwilioService;

/* [283A-23] Función libre que envía la campaña por cada canal configurado.
 * Extraída de `enviar()` para cumplir el límite de líneas de clippy.
 * Email via SMTP, SMS via Twilio, `WhatsApp` via Meta Cloud API.
 * [303A-1] Retorna (enviados, fallidos) y actualiza estado de cada destinatario. */
async fn enviar_por_canales(
    pool: &PgPool,
    campana_id: Uuid,
    integ: &IntegracionMarketing,
    clientes: &[ClienteSegmentado],
    campana: &Campana,
) -> (u32, u32) {
    let mut enviados = 0u32;
    let mut errores = 0u32;

    for cliente in clientes {
        for canal in &campana.canales {
            let resultado = match canal.as_str() {
                "email" if cliente.consentimiento_comercial_email && !cliente.email.is_empty() => {
                    EmailService::enviar_campana(integ, &cliente.email, &campana.nombre, &campana.cuerpo_mensaje)
                        .await
                        .map_err(|e| e.to_string())
                }
                "sms" if cliente.consentimiento_comercial_sms && !cliente.telefono.is_empty() => {
                    let numero = format!("{}{}", cliente.prefijo_telefono, cliente.telefono);
                    TwilioService::enviar_sms(integ, &numero, &campana.cuerpo_mensaje)
                        .await
                        .map_err(|e| e.to_string())
                }
                "whatsapp" if cliente.consentimiento_comercial_sms && !cliente.telefono.is_empty() => {
                    let numero = format!("{}{}", cliente.prefijo_telefono, cliente.telefono);
                    MetaWhatsappService::enviar_mensaje(integ, &numero, &campana.cuerpo_mensaje)
                        .await
                        .map_err(|e| e.to_string())
                }
                _ => continue,
            };

            match resultado {
                Ok(true) => {
                    enviados += 1;
                    /* [303A-1] Marcar destinatario como enviado */
                    let _ = CampanaRepository::actualizar_estado_destinatario(
                        pool, campana_id, cliente.id, canal, "enviado",
                    ).await;
                }
                Ok(false) => {} /* no configurado — ya logueado en el servicio */
                Err(e) => {
                    tracing::error!("Error enviando {} a cliente {}: {}", canal, cliente.id, e);
                    errores += 1;
                    /* [303A-1] Marcar destinatario como fallido */
                    let _ = CampanaRepository::actualizar_estado_destinatario(
                        pool, campana_id, cliente.id, canal, "fallido",
                    ).await;
                }
            }
        }
    }

    tracing::info!("Envío completado: {} enviados, {} errores", enviados, errores);
    (enviados, errores)
}

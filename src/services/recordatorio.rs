/* [263A-25] Servicio de recordatorios automáticos.
 * CRUD de reglas + lógica central del scheduler: buscar reservas pendientes,
 * enviar notificaciones, registrar envíos.
 * [283A-23] Envío real de email via SMTP (integraciones_marketing). */

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{
    ActualizarReglaRequest, CrearReglaRequest, HistorialRecordatorios, ReglaRecordatorio,
    ReglasPaginadas, CANALES_RECORDATORIO,
};
use crate::repositories::recordatorio::{ActualizarReglaData, NuevaRegla, RecordatorioRepository};
use crate::repositories::IntegracionMarketingRepository;
use crate::services::email::EmailService;
use crate::services::meta_whatsapp::MetaWhatsappService;
use crate::services::twilio::TwilioService;

pub struct RecordatorioService;

impl RecordatorioService {
    /* [014A-3] Crear regla de recordatorio con soporte para tipo "antes" y "despues" */
    pub async fn crear_regla(
        pool: &PgPool,
        user_id: Uuid,
        req: CrearReglaRequest,
    ) -> Result<ReglaRecordatorio, AppError> {
        validar_canal(&req.canal)?;

        let tipo = req.tipo.as_deref().unwrap_or("antes");
        validar_tipo_y_horas(tipo, req.horas_antes, req.horas_despues)?;

        let data = NuevaRegla {
            nombre: req.nombre,
            horas_antes: req.horas_antes,
            canal: req.canal,
            mensaje_plantilla: req.mensaje_plantilla.unwrap_or_default(),
            tipo: tipo.to_string(),
            horas_despues: req.horas_despues,
        };

        RecordatorioRepository::create(pool, user_id, data)
            .await
            .map_err(AppError::Database)
    }

    pub async fn obtener_regla(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<ReglaRecordatorio, AppError> {
        RecordatorioRepository::find_by_id(pool, id, user_id)
            .await
            .map_err(AppError::Database)?
            .ok_or(AppError::NotFound("Regla de recordatorio no encontrada".into()))
    }

    pub async fn listar_reglas(
        pool: &PgPool,
        user_id: Uuid,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> Result<ReglasPaginadas, AppError> {
        let p = page.unwrap_or(1).max(1);
        let pp = per_page.unwrap_or(20).clamp(1, 100);

        RecordatorioRepository::list(pool, user_id, p, pp)
            .await
            .map_err(AppError::Database)
    }

    pub async fn actualizar_regla(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        req: ActualizarReglaRequest,
    ) -> Result<ReglaRecordatorio, AppError> {
        if let Some(ref canal) = req.canal {
            validar_canal(canal)?;
        }
        if let Some(ref tipo) = req.tipo {
            validar_tipo_y_horas(tipo, req.horas_antes, req.horas_despues)?;
        } else {
            /* Si no cambia tipo, validar horas individuales */
            if let Some(h) = req.horas_antes {
                if h <= 0 {
                    return Err(AppError::Validation("horas_antes debe ser > 0".into()));
                }
            }
            if let Some(h) = req.horas_despues {
                if h <= 0 {
                    return Err(AppError::Validation("horas_despues debe ser > 0".into()));
                }
            }
        }

        let data = ActualizarReglaData {
            nombre: req.nombre,
            horas_antes: req.horas_antes,
            canal: req.canal,
            mensaje_plantilla: req.mensaje_plantilla,
            activa: req.activa,
            tipo: req.tipo,
            horas_despues: req.horas_despues,
        };

        RecordatorioRepository::update(pool, id, user_id, data)
            .await
            .map_err(AppError::Database)?
            .ok_or(AppError::NotFound("Regla de recordatorio no encontrada".into()))
    }

    pub async fn eliminar_regla(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let deleted = RecordatorioRepository::delete(pool, id, user_id)
            .await
            .map_err(AppError::Database)?;

        if !deleted {
            return Err(AppError::NotFound("Regla de recordatorio no encontrada".into()));
        }
        Ok(())
    }

    pub async fn historial(
        pool: &PgPool,
        user_id: Uuid,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> Result<HistorialRecordatorios, AppError> {
        let p = page.unwrap_or(1).max(1);
        let pp = per_page.unwrap_or(20).clamp(1, 100);

        let (items, total) = RecordatorioRepository::historial(pool, user_id, p, pp)
            .await
            .map_err(AppError::Database)?;

        Ok(HistorialRecordatorios { items, total, page: p, per_page: pp })
    }

    /* [283A-23] Ejecuta un ciclo del scheduler: busca reservas pendientes y envía recordatorios.
     * Email se envía real via SMTP si está configurado. SMS/WhatsApp siguen como stub. */
    pub async fn ejecutar_ciclo(pool: &PgPool) -> Result<usize, AppError> {
        let pendientes = RecordatorioRepository::reservas_pendientes_recordatorio(pool)
            .await
            .map_err(AppError::Database)?;

        let mut procesados = 0;

        /* Cache de integraciones por user_id para evitar queries repetidas */
        let mut cache_integ: std::collections::HashMap<Uuid, crate::models::IntegracionMarketing> =
            std::collections::HashMap::new();

        for p in &pendientes {
            let resultado = enviar_recordatorio(pool, p, &mut cache_integ).await;

            let (estado, error) = match resultado {
                Ok(()) => ("enviado", None),
                Err(e) => ("fallido", Some(e)),
            };

            RecordatorioRepository::registrar_envio(
                pool,
                p.regla_id,
                p.reserva_id,
                &p.canal,
                estado,
                error.as_deref(),
            )
            .await
            .map_err(AppError::Database)?;

            procesados += 1;
        }

        if procesados > 0 {
            tracing::info!("Recordatorios procesados: {procesados}");
        }

        /* [094A-5] Procesar mensajes de inactividad de clientes */
        let inactivos = procesar_inactividad(pool).await;
        procesados += inactivos;

        Ok(procesados)
    }
}

fn validar_canal(canal: &str) -> Result<(), AppError> {
    if !CANALES_RECORDATORIO.contains(&canal) {
        return Err(AppError::Validation(format!(
            "Canal inválido: {canal}. Opciones: {}",
            CANALES_RECORDATORIO.join(", ")
        )));
    }
    Ok(())
}

/* [014A-3] Valida coherencia entre tipo y campos horas_antes/horas_despues */
fn validar_tipo_y_horas(tipo: &str, horas_antes: Option<i32>, horas_despues: Option<i32>) -> Result<(), AppError> {
    match tipo {
        "antes" => {
            let h = horas_antes.unwrap_or(0);
            if h <= 0 {
                return Err(AppError::Validation("Tipo 'antes' requiere horas_antes > 0".into()));
            }
        }
        "despues" => {
            let h = horas_despues.unwrap_or(0);
            if h <= 0 {
                return Err(AppError::Validation("Tipo 'despues' requiere horas_despues > 0".into()));
            }
        }
        _ => return Err(AppError::Validation("Tipo debe ser 'antes' o 'despues'".into())),
    }
    Ok(())
}

/* [283A-23] Envío real de recordatorio por canal.
 * Email: SMTP. SMS: Twilio. `WhatsApp`: Meta Cloud API.
 * Todos dependen de las credenciales en integraciones_marketing. */
async fn enviar_recordatorio(
    pool: &PgPool,
    p: &crate::repositories::recordatorio::ReservaPendienteRecordatorio,
    cache: &mut std::collections::HashMap<Uuid, crate::models::IntegracionMarketing>,
) -> Result<(), String> {
    /* Renderizar el mensaje con placeholders */
    let mensaje = p
        .mensaje_plantilla
        .replace("{nombre}", &p.nombre_cliente)
        .replace("{fecha}", &p.fecha.to_string())
        .replace("{hora}", &p.hora.format("%H:%M").to_string());

    /* Obtener integraciones del usuario (con cache) */
    let integ = if let Some(cached) = cache.get(&p.user_id) {
        cached.clone()
    } else {
        let i = IntegracionMarketingRepository::obtener_o_crear(pool, p.user_id)
            .await
            .map_err(|e| format!("Error obteniendo integraciones: {e}"))?;
        cache.insert(p.user_id, i.clone());
        i
    };

    match p.canal.as_str() {
        "email" => {
            let email_dest = p.email_cliente.as_deref().unwrap_or("");
            if email_dest.is_empty() {
                return Err("Cliente sin email".to_string());
            }
            let asunto = format!("Recordatorio: reserva del {}", p.fecha);
            match EmailService::enviar_campana(&integ, email_dest, &asunto, &mensaje).await {
                Ok(true) => Ok(()),
                Ok(false) => Err("SMTP no configurado".to_string()),
                Err(e) => Err(format!("Error SMTP: {e}")),
            }
        }
        "sms" => {
            if p.telefono.is_empty() {
                return Err("Cliente sin teléfono".to_string());
            }
            /* [303A-1] Componer número E.164 con prefijo del cliente */
            let numero = format!("{}{}", p.prefijo_telefono, p.telefono);
            match TwilioService::enviar_sms(&integ, &numero, &mensaje).await {
                Ok(true) => Ok(()),
                Ok(false) => Err("Twilio no configurado".to_string()),
                Err(e) => Err(format!("Error Twilio: {e}")),
            }
        }
        "whatsapp" => {
            if p.telefono.is_empty() {
                return Err("Cliente sin teléfono".to_string());
            }
            /* [303A-1] Componer número E.164 con prefijo del cliente */
            let numero = format!("{}{}", p.prefijo_telefono, p.telefono);
            match MetaWhatsappService::enviar_mensaje(&integ, &numero, &mensaje).await {
                Ok(true) => Ok(()),
                Ok(false) => Err("Meta WhatsApp no configurado".to_string()),
                Err(e) => Err(format!("Error Meta: {e}")),
            }
        }
        otro => Err(format!("Canal desconocido: {otro}")),
    }
}

/* [094A-5] Procesa mensajes automáticos a clientes inactivos.
 * Consulta todas las reglas activas, busca clientes que cumplen el criterio,
 * envía el mensaje y registra el envío para no repetir. */
async fn procesar_inactividad(pool: &PgPool) -> usize {
    use crate::repositories::InactividadRepository;

    let pendientes = match InactividadRepository::clientes_inactivos_pendientes(pool).await {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Error buscando clientes inactivos: {e}");
            return 0;
        }
    };

    if pendientes.is_empty() {
        return 0;
    }

    let mut cache_integ: std::collections::HashMap<Uuid, crate::models::IntegracionMarketing> =
        std::collections::HashMap::new();
    let mut procesados = 0;

    for ci in &pendientes {
        /* Renderizar mensaje con placeholders */
        let mensaje = ci.mensaje_plantilla.replace("{nombre}", &ci.nombre);

        /* Obtener integraciones del usuario */
        let integ = if let Some(cached) = cache_integ.get(&ci.user_id) {
            cached.clone()
        } else {
            match IntegracionMarketingRepository::obtener_o_crear(pool, ci.user_id).await {
                Ok(i) => {
                    cache_integ.insert(ci.user_id, i.clone());
                    i
                }
                Err(e) => {
                    tracing::error!("Error integraciones para inactividad: {e}");
                    let _ = InactividadRepository::registrar_envio(
                        pool, ci.regla_id, ci.cliente_id, "fallido",
                    )
                    .await;
                    continue;
                }
            }
        };

        let resultado = match ci.canal.as_str() {
            "email" => {
                if ci.email.is_empty() {
                    Err("Sin email".into())
                } else {
                    let asunto = "¡Te echamos de menos!".to_string();
                    EmailService::enviar_campana(&integ, &ci.email, &asunto, &mensaje)
                        .await
                        .map(|_| ())
                        .map_err(|e| format!("SMTP: {e}"))
                }
            }
            "sms" => {
                if ci.telefono.is_empty() {
                    Err("Sin teléfono".into())
                } else {
                    TwilioService::enviar_sms(&integ, &ci.telefono, &mensaje)
                        .await
                        .map(|_| ())
                        .map_err(|e| format!("Twilio: {e}"))
                }
            }
            "whatsapp" => {
                if ci.telefono.is_empty() {
                    Err("Sin teléfono".into())
                } else {
                    MetaWhatsappService::enviar_mensaje(&integ, &ci.telefono, &mensaje)
                        .await
                        .map(|_| ())
                        .map_err(|e| format!("Meta: {e}"))
                }
            }
            otro => Err(format!("Canal desconocido: {otro}")),
        };

        let estado = if resultado.is_ok() { "enviado" } else { "fallido" };
        if let Err(ref e) = resultado {
            tracing::warn!("Inactividad envío fallido cliente={}: {e}", ci.cliente_id);
        }

        let _ = InactividadRepository::registrar_envio(
            pool, ci.regla_id, ci.cliente_id, estado,
        )
        .await;
        procesados += 1;
    }

    if procesados > 0 {
        tracing::info!("Mensajes de inactividad procesados: {procesados}");
    }
    procesados
}

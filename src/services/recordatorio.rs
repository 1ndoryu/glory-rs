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
    pub async fn crear_regla(
        pool: &PgPool,
        user_id: Uuid,
        req: CrearReglaRequest,
    ) -> Result<ReglaRecordatorio, AppError> {
        validar_canal(&req.canal)?;

        if req.horas_antes <= 0 {
            return Err(AppError::Validation("horas_antes debe ser > 0".into()));
        }

        let data = NuevaRegla {
            nombre: req.nombre,
            horas_antes: req.horas_antes,
            canal: req.canal,
            mensaje_plantilla: req.mensaje_plantilla.unwrap_or_default(),
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
        if let Some(h) = req.horas_antes {
            if h <= 0 {
                return Err(AppError::Validation("horas_antes debe ser > 0".into()));
            }
        }

        let data = ActualizarReglaData {
            nombre: req.nombre,
            horas_antes: req.horas_antes,
            canal: req.canal,
            mensaje_plantilla: req.mensaje_plantilla,
            activa: req.activa,
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
            match TwilioService::enviar_sms(&integ, &p.telefono, &mensaje).await {
                Ok(true) => Ok(()),
                Ok(false) => Err("Twilio no configurado".to_string()),
                Err(e) => Err(format!("Error Twilio: {e}")),
            }
        }
        "whatsapp" => {
            if p.telefono.is_empty() {
                return Err("Cliente sin teléfono".to_string());
            }
            match MetaWhatsappService::enviar_mensaje(&integ, &p.telefono, &mensaje).await {
                Ok(true) => Ok(()),
                Ok(false) => Err("Meta WhatsApp no configurado".to_string()),
                Err(e) => Err(format!("Error Meta: {e}")),
            }
        }
        otro => Err(format!("Canal desconocido: {otro}")),
    }
}

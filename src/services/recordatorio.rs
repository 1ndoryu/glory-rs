/* [263A-25] Servicio de recordatorios automáticos.
 * CRUD de reglas + lógica central del scheduler: buscar reservas pendientes,
 * enviar notificaciones (stub), registrar envíos. */

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{
    ActualizarReglaRequest, CrearReglaRequest, HistorialRecordatorios, ReglaRecordatorio,
    ReglasPaginadas, CANALES_RECORDATORIO,
};
use crate::repositories::recordatorio::{ActualizarReglaData, NuevaRegla, RecordatorioRepository};

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

    /* Ejecuta un ciclo del scheduler: busca reservas pendientes y envía recordatorios.
     * Retorna cuántos recordatorios se procesaron. */
    pub async fn ejecutar_ciclo(pool: &PgPool) -> Result<usize, AppError> {
        let pendientes = RecordatorioRepository::reservas_pendientes_recordatorio(pool)
            .await
            .map_err(AppError::Database)?;

        let mut procesados = 0;

        for p in &pendientes {
            let resultado = enviar_recordatorio(p).await;

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

/* Stub de envío de recordatorio. Cuando se integre con proveedores reales
 * (Twilio SMS, SMTP email, Meta WhatsApp API), reemplazar esta función.
 * Por ahora solo loguea la intención. Se mantiene async porque la
 * implementación real será async (llamadas HTTP a Twilio/Meta). */
#[allow(clippy::unused_async)]
async fn enviar_recordatorio(
    p: &crate::repositories::recordatorio::ReservaPendienteRecordatorio,
) -> Result<(), String> {
    tracing::info!(
        canal = %p.canal,
        reserva = %p.reserva_id,
        cliente = %p.nombre_cliente,
        fecha = %p.fecha,
        hora = %p.hora,
        "Enviando recordatorio de reserva"
    );

    /* Simulación: el envío siempre tiene éxito en el stub */
    Ok(())
}

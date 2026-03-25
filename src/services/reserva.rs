/* 253A-5: Servicio de reservas */

use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{
    ActualizarReservaRequest, CrearReservaRequest, Reserva, ReservasConteo, ReservasPaginadas,
};
use crate::repositories::reserva::{ActualizarReservaData, NuevaReserva};
use crate::repositories::ReservaRepository;

pub struct ReservaService;

impl ReservaService {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        req: CrearReservaRequest,
    ) -> Result<Reserva, AppError> {
        let estado = req
            .estado
            .as_ref()
            .and_then(|e| {
                serde_json::to_value(e)
                    .ok()
                    .and_then(|v| v.as_str().map(String::from))
            })
            .unwrap_or_else(|| "confirmada".into());

        let data = NuevaReserva {
            user_id,
            fecha: req.fecha,
            hora: req.hora,
            nombre_cliente: &req.nombre_cliente,
            num_personas: req.num_personas,
            estado: &estado,
            notas: req.notas.as_deref().unwrap_or(""),
            telefono: req.telefono.as_deref().unwrap_or(""),
        };

        let reserva = ReservaRepository::create(pool, &data).await?;
        Ok(reserva)
    }

    pub async fn get(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Reserva, AppError> {
        ReservaRepository::find_by_id(pool, id, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Reserva no encontrada".into()))
    }

    pub async fn list(
        pool: &PgPool,
        user_id: Uuid,
        page: i64,
        per_page: i64,
        fecha: Option<chrono::NaiveDate>,
    ) -> Result<ReservasPaginadas, AppError> {
        let (items, total) = ReservaRepository::list(pool, user_id, page, per_page, fecha).await?;
        Ok(ReservasPaginadas {
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
        req: ActualizarReservaRequest,
    ) -> Result<Reserva, AppError> {
        let estado_str = req.estado.as_ref().and_then(|e| {
            serde_json::to_value(e)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
        });

        let data = ActualizarReservaData {
            id,
            user_id,
            fecha: req.fecha,
            hora: req.hora,
            nombre_cliente: req.nombre_cliente.as_deref(),
            num_personas: req.num_personas,
            estado: estado_str.as_deref(),
            notas: req.notas.as_deref(),
            telefono: req.telefono.as_deref(),
        };

        ReservaRepository::update(pool, &data)
            .await?
            .ok_or_else(|| AppError::NotFound("Reserva no encontrada".into()))
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        if !ReservaRepository::delete(pool, id, user_id).await? {
            return Err(AppError::NotFound("Reserva no encontrada".into()));
        }
        Ok(())
    }

    /// Conteo de reservas del mes y del día (para Home)
    pub async fn conteo(pool: &PgPool, user_id: Uuid) -> Result<ReservasConteo, AppError> {
        let (total_mes, total_hoy) = ReservaRepository::conteo_actual(pool, user_id).await?;
        Ok(ReservasConteo {
            total_mes,
            total_hoy,
        })
    }
}

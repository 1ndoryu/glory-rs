/* 253A-5: Servicio de reservas
   263A-6: Filtros turno/estado, num_mesa, apellidos_cliente, resumen mensual */

use chrono::NaiveTime;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{
    ActualizarReservaRequest, CrearReservaRequest, NoShowStats, Reserva, ReservasConteo,
    ReservasPaginadas, ResumenDiario,
};
use crate::repositories::reserva::{ActualizarReservaData, FiltrosReserva, NuevaReserva};
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
            num_mesa: req.num_mesa,
            apellidos_cliente: req.apellidos_cliente.as_deref().unwrap_or(""),
            canal_id: req.canal_id,
            mesa_id: req.mesa_id,
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
        estado: Option<&str>,
        turno: Option<&str>,
    ) -> Result<ReservasPaginadas, AppError> {
        /* 263A-6: Mapear turno a rango horario */
        let (hora_desde, hora_hasta) = Self::turno_a_horas(turno);

        /* [263A-20] Normalizar filtros vacíos a None.
         * Orval puede enviar estado="" o turno="" como query param
         * en vez de omitirlo, lo que causa que el SQL busque estado = '' */
        let estado_normalizado = estado.filter(|s| !s.is_empty()).map(String::from);

        let filtros = FiltrosReserva {
            user_id,
            page,
            per_page,
            fecha,
            estado: estado_normalizado,
            hora_desde,
            hora_hasta,
        };

        let (items, total) = ReservaRepository::list(pool, &filtros).await?;
        Ok(ReservasPaginadas {
            items,
            total,
            page,
            per_page,
        })
    }

    /// Convierte un turno de reserva a un rango horario
    fn turno_a_horas(turno: Option<&str>) -> (Option<NaiveTime>, Option<NaiveTime>) {
        match turno {
            Some("desayuno") => (
                Some(NaiveTime::from_hms_opt(7, 0, 0).expect("hora válida")),
                Some(NaiveTime::from_hms_opt(12, 0, 0).expect("hora válida")),
            ),
            Some("comida") => (
                Some(NaiveTime::from_hms_opt(12, 0, 0).expect("hora válida")),
                Some(NaiveTime::from_hms_opt(18, 0, 0).expect("hora válida")),
            ),
            Some("cena") => (
                Some(NaiveTime::from_hms_opt(18, 0, 0).expect("hora válida")),
                Some(NaiveTime::from_hms_opt(23, 59, 0).expect("hora válida")),
            ),
            _ => (None, None),
        }
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
            num_mesa: req.num_mesa,
            apellidos_cliente: req.apellidos_cliente.as_deref(),
            canal_id: req.canal_id,
            mesa_id: req.mesa_id,
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

    /// Resumen diario de un mes — para la vista calendario (263A-7)
    pub async fn resumen_mensual(
        pool: &PgPool,
        user_id: Uuid,
        anio: i32,
        mes: i32,
    ) -> Result<Vec<ResumenDiario>, AppError> {
        let datos = ReservaRepository::resumen_mensual(pool, user_id, anio, mes).await?;
        Ok(datos)
    }

    /// Estadísticas de no-shows con desglose por canal (263A-8)
    pub async fn no_show_stats(
        pool: &PgPool,
        user_id: Uuid,
        fecha_desde: Option<chrono::NaiveDate>,
        fecha_hasta: Option<chrono::NaiveDate>,
    ) -> Result<NoShowStats, AppError> {
        let (total_reservas, total_no_shows) =
            ReservaRepository::no_show_totales(pool, user_id, fecha_desde, fecha_hasta).await?;

        let ratio_porcentaje = if total_reservas > 0 {
            #[allow(clippy::cast_precision_loss)] // conteos de reservas nunca excederán 2^52
            { (total_no_shows as f64 / total_reservas as f64) * 100.0 }
        } else {
            0.0
        };

        let por_canal =
            ReservaRepository::no_show_por_canal(pool, user_id, fecha_desde, fecha_hasta).await?;

        Ok(NoShowStats {
            total_reservas,
            total_no_shows,
            ratio_porcentaje,
            por_canal,
        })
    }
}

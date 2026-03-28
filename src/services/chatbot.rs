/* [283A-2] Servicio de chatbot — lógica de disponibilidad, info restaurante
 * y operaciones de reserva adaptadas para chatbots externos.
 * Reutiliza repositorios existentes (reservas, plano_sala, configuracion). */

use chrono::{NaiveDate, NaiveTime, Timelike};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{
    CamposObligatorios, ChatbotCrearReservaRequest, ChatbotReservaResponse,
    DisponibilidadResponse, FranjaDisponibilidad, Reserva, RestauranteInfoResponse, ZonaResumen,
};
use crate::repositories::reserva::{FiltrosReserva, NuevaReserva};
use crate::repositories::{ConfiguracionRepository, PlanoSalaRepository, ReservaRepository};

pub struct ChatbotService;

/// Duración estimada de una reserva en minutos (para calcular solapamiento)
const DURACION_RESERVA_MIN: i64 = 90;

/// Franjas horarias para calcular disponibilidad (cada 30 min)
const FRANJA_INICIO_HORA: u32 = 12;
const FRANJA_FIN_HORA: u32 = 24;
const FRANJA_INTERVALO_MIN: u32 = 30;

impl ChatbotService {
    /// Calcula la disponibilidad de mesas para una fecha.
    /// Retorna franjas de 30 min con capacidad disponible.
    pub async fn disponibilidad(
        pool: &PgPool,
        user_id: Uuid,
        fecha: NaiveDate,
    ) -> Result<DisponibilidadResponse, AppError> {
        /* Obtener todas las mesas activas */
        let zonas = PlanoSalaRepository::listar_zonas(pool, user_id).await?;
        let mut todas_mesas = Vec::new();
        for zona in &zonas {
            let mesas = PlanoSalaRepository::listar_mesas_zona(pool, zona.id).await?;
            for mesa in mesas {
                if mesa.activa {
                    todas_mesas.push(mesa);
                }
            }
        }

        let total_mesas = todas_mesas.len().try_into().unwrap_or(i32::MAX);
        let capacidad_total: i32 = todas_mesas.iter().map(|m| m.max_personas).sum();

        /* Obtener reservas del día (excluyendo canceladas) */
        let reservas = ReservaRepository::listar_por_fecha(pool, user_id, fecha).await
            .unwrap_or_default();

        /* Generar franjas de 30 min */
        let mut franjas = Vec::new();
        let mut hora = FRANJA_INICIO_HORA;
        let mut minuto = 0u32;

        while hora < FRANJA_FIN_HORA {
            let slot_time = NaiveTime::from_hms_opt(hora % 24, minuto, 0);
            let Some(slot) = slot_time else { break };

            /* Contar reservas que solapan con esta franja */
            let mut personas_reservadas = 0i32;
            let mut mesas_ocupadas = 0i32;

            for r in &reservas {
                if Self::solapa_franja(r.hora, slot) {
                    personas_reservadas += r.num_personas;
                    mesas_ocupadas += 1;
                }
            }

            franjas.push(FranjaDisponibilidad {
                hora: slot,
                personas_reservadas,
                mesas_ocupadas,
                mesas_disponibles: total_mesas - mesas_ocupadas,
                capacidad_disponible: capacidad_total - personas_reservadas,
            });

            minuto += FRANJA_INTERVALO_MIN;
            if minuto >= 60 {
                minuto -= 60;
                hora += 1;
            }
        }

        Ok(DisponibilidadResponse {
            fecha,
            franjas,
            capacidad_total,
        })
    }

    /// Info pública del restaurante para el chatbot
    pub async fn restaurante_info(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<RestauranteInfoResponse, AppError> {
        let config = ConfiguracionRepository::obtener_o_crear(pool, user_id).await?;
        let zonas_db = PlanoSalaRepository::listar_zonas(pool, user_id).await?;

        let mut zonas = Vec::with_capacity(zonas_db.len());
        let mut capacidad_total = 0i32;

        for zona in zonas_db {
            let mesas = PlanoSalaRepository::listar_mesas_zona(pool, zona.id).await?;
            let activas: Vec<_> = mesas.into_iter().filter(|m| m.activa).collect();
            let cap_min: i32 = activas.iter().map(|m| m.min_personas).sum();
            let cap_max: i32 = activas.iter().map(|m| m.max_personas).sum();
            capacidad_total += cap_max;

            zonas.push(ZonaResumen {
                nombre: zona.nombre,
                mesas: activas.len().try_into().unwrap_or(i32::MAX),
                capacidad_min: cap_min,
                capacidad_max: cap_max,
            });
        }

        Ok(RestauranteInfoResponse {
            nombre: config.nombre_restaurante,
            campos_obligatorios: CamposObligatorios {
                email: config.reserva_email_obligatorio,
                telefono: config.reserva_telefono_obligatorio,
                nombre: config.reserva_nombre_obligatorio,
                apellidos: config.reserva_apellidos_obligatorio,
            },
            capacidad_total,
            zonas,
        })
    }

    /// Crea una reserva desde el chatbot
    pub async fn crear_reserva(
        pool: &PgPool,
        user_id: Uuid,
        req: ChatbotCrearReservaRequest,
    ) -> Result<Reserva, AppError> {
        let data = NuevaReserva {
            user_id,
            fecha: req.fecha,
            hora: req.hora,
            nombre_cliente: &req.nombre_cliente,
            num_personas: req.num_personas,
            estado: "confirmada",
            notas: req.notas.as_deref().unwrap_or(""),
            telefono: req.telefono.as_deref().unwrap_or(""),
            num_mesa: None,
            apellidos_cliente: req.apellidos_cliente.as_deref().unwrap_or(""),
            canal_id: None,
            mesa_id: None,
        };

        let reserva = ReservaRepository::create(pool, &data).await?;
        Ok(reserva)
    }

    /// Busca reservas por teléfono, nombre o fecha
    pub async fn buscar_reservas(
        pool: &PgPool,
        user_id: Uuid,
        telefono: Option<&str>,
        nombre: Option<&str>,
        fecha: Option<NaiveDate>,
    ) -> Result<Vec<ChatbotReservaResponse>, AppError> {
        /* Usar filtros del repositorio existente con paginación amplia */
        let filtros = FiltrosReserva {
            user_id,
            page: 1,
            per_page: 50,
            fecha,
            estado: None,
            hora_desde: None,
            hora_hasta: None,
        };

        let resultado = ReservaRepository::list(pool, &filtros).await?;

        /* Filtrar en memoria por teléfono/nombre si se proporcionan */
        let reservas: Vec<ChatbotReservaResponse> = resultado.0
            .into_iter()
            .filter(|r| {
                let pasa_telefono = telefono
                    .is_none_or(|t| !t.is_empty() && r.telefono.contains(t));
                let pasa_nombre = nombre.is_none_or(|n| {
                    !n.is_empty()
                        && (r.nombre_cliente.to_lowercase().contains(&n.to_lowercase())
                            || r.apellidos_cliente.to_lowercase().contains(&n.to_lowercase()))
                });
                pasa_telefono && pasa_nombre
            })
            .map(|r| ChatbotReservaResponse {
                id: r.id,
                fecha: r.fecha,
                hora: r.hora,
                nombre_cliente: r.nombre_cliente,
                apellidos_cliente: r.apellidos_cliente,
                num_personas: r.num_personas,
                estado: r.estado,
                telefono: r.telefono,
                notas: r.notas,
                mesa_numero: r.num_mesa,
            })
            .collect();

        Ok(reservas)
    }

    /// Obtiene una reserva por ID (para el chatbot)
    pub async fn obtener_reserva(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<ChatbotReservaResponse, AppError> {
        let r = ReservaRepository::find_by_id(pool, id, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Reserva no encontrada".into()))?;

        Ok(ChatbotReservaResponse {
            id: r.id,
            fecha: r.fecha,
            hora: r.hora,
            nombre_cliente: r.nombre_cliente,
            apellidos_cliente: r.apellidos_cliente,
            num_personas: r.num_personas,
            estado: r.estado,
            telefono: r.telefono,
            notas: r.notas,
            mesa_numero: r.num_mesa,
        })
    }

    /// Cancela una reserva (cambia estado a "cancelada")
    pub async fn cancelar_reserva(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let reserva = ReservaRepository::find_by_id(pool, id, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Reserva no encontrada".into()))?;

        if reserva.estado == "cancelada" {
            return Err(AppError::BadRequest("La reserva ya está cancelada".into()));
        }

        ReservaRepository::cancelar(pool, id, user_id).await?;
        Ok(())
    }

    /// Verifica si una reserva solapa con una franja horaria de 30 min
    fn solapa_franja(hora_reserva: NaiveTime, hora_franja: NaiveTime) -> bool {
        let reserva_start = i64::from(hora_reserva.hour()) * 60 + i64::from(hora_reserva.minute());
        let slot_start = i64::from(hora_franja.hour()) * 60 + i64::from(hora_franja.minute());

        /* La reserva solapa si empieza antes de que termine la franja
         * y termina después de que empiece la franja */
        let reserva_end = reserva_start + DURACION_RESERVA_MIN;
        let slot_end = slot_start + i64::from(FRANJA_INTERVALO_MIN);

        reserva_start < slot_end && reserva_end > slot_start
    }
}

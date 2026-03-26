/* [263A-13] Modelos para el dashboard de reservas Fase 2.
 * Tres paneles: resumen, ocupacion, analisis.
 * Cada sub-struct es serializable y documentado en OpenAPI. */

use rust_decimal::Decimal;
use serde::Serialize;
use utoipa::ToSchema;

/// Dashboard completo de reservas: 3 paneles de analisis
#[derive(Debug, Serialize, ToSchema)]
pub struct DashboardReservas {
    pub resumen: ResumenReservas,
    pub ocupacion: OcupacionReservas,
    pub analisis: AnalisisReservas,
}

/// Panel 1 — Resumen: totales, comparativa, distribuciones
#[derive(Debug, Serialize, ToSchema)]
pub struct ResumenReservas {
    pub total_reservas: i64,
    pub total_mes_anterior: i64,
    pub variacion_porcentaje: f64,
    pub por_dia: Vec<AgrupacionFecha>,
    pub por_dia_semana: Vec<AgrupacionDiaSemana>,
    pub por_canal: Vec<AgrupacionCanal>,
    pub clientes_nuevos: i64,
}

/// Panel 2 — Ocupacion: medias, distribucion horaria, turnos, procedencia
#[derive(Debug, Serialize, ToSchema)]
pub struct OcupacionReservas {
    pub media_personas: f64,
    pub media_reservas_dia: f64,
    pub total_reservas: i64,
    pub por_hora: Vec<AgrupacionHora>,
    pub por_turno: Vec<AgrupacionTurno>,
    pub por_procedencia: Vec<AgrupacionCanal>,
    pub antelacion_media_dias: f64,
}

/// Panel 3 — Analisis: efectividad, comensales, ticket medio
#[derive(Debug, Serialize, ToSchema)]
pub struct AnalisisReservas {
    pub reservas_efectivas: i64,
    pub total_comensales: i64,
    pub comensales_por_reserva: f64,
    pub ticket_medio_reserva: Option<Decimal>,
    pub ticket_medio_persona: Option<Decimal>,
}

/// Reservas agrupadas por fecha
#[derive(Debug, Serialize, ToSchema)]
pub struct AgrupacionFecha {
    pub fecha: String,
    pub total: i64,
    pub personas: i64,
}

/// Reservas agrupadas por dia de la semana (lunes-domingo)
#[derive(Debug, Serialize, ToSchema)]
pub struct AgrupacionDiaSemana {
    pub dia: String,
    pub total: i64,
}

/// Reservas agrupadas por canal de reserva
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AgrupacionCanal {
    pub canal: String,
    pub total: i64,
    pub porcentaje: f64,
}

/// Reservas agrupadas por hora del dia
#[derive(Debug, Serialize, ToSchema)]
pub struct AgrupacionHora {
    pub hora: String,
    pub total: i64,
}

/// Reservas agrupadas por turno (comida/cena)
#[derive(Debug, Serialize, ToSchema)]
pub struct AgrupacionTurno {
    pub turno: String,
    pub total: i64,
    pub personas: i64,
}

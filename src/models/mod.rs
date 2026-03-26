pub mod common;
mod canal_reserva;
mod cliente;
mod dashboard;
mod dashboard_reservas;
mod etiqueta;
mod gasto;
mod plano_sala;
mod reserva;
mod user;
mod venta;

pub use canal_reserva::{CanalReserva, CrearCanalReservaRequest};
pub use cliente::{
    ActualizarClienteRequest, Cliente, ClientesPaginados, ClientesQuery, CrearClienteRequest,
};
pub use dashboard::ResumenEconomico;
pub use dashboard_reservas::{
    AgrupacionCanal, AgrupacionDiaSemana, AgrupacionFecha, AgrupacionHora, AgrupacionTurno,
    AnalisisReservas, DashboardReservas, OcupacionReservas, ResumenReservas,
};
pub use etiqueta::{
    CategoriaEtiqueta, CrearCategoriaEtiquetaRequest, CrearEtiquetaRequest, Etiqueta,
    EtiquetaConCategoria, EtiquetasQuery,
};
pub use gasto::{
    CategoriaGasto, CrearGastoRequest, Gasto, GastosPaginados, GastosQuery, TipoDocumento,
};
pub use reserva::{
    ActualizarReservaRequest, CrearReservaRequest, EstadoReserva, NoShowPorCanal, NoShowQuery,
    NoShowStats, Reserva, ReservasConteo, ReservasPaginadas, ReservasQuery, ResumenDiario,
    ResumenMesQuery,
};
pub use user::{
    AuthResponse, ForgotPasswordRequest, LoginRequest, MessageResponse, RegisterRequest,
    ResetPasswordRequest, User, UserResponse,
};
pub use venta::{
    CanalVenta, CrearVentaRequest, MetodoPago, Turno, Venta, VentasPaginadas, VentasQuery,
};
pub use plano_sala::{
    ActualizarMesaRequest, ActualizarPosicionesRequest, ActualizarZonaRequest,
    CombinacionConMesas, CombinacionExport, CombinacionMesas, CrearCombinacionRequest,
    CrearMesaRequest, CrearZonaRequest, Mesa, MesaExport, PlanoExport, PlanoSala,
    PosicionMesa, ZonaConMesas, ZonaExport, ZonaSala,
};

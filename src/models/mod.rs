pub mod common;
mod dashboard;
mod gasto;
mod reserva;
mod user;
mod venta;

pub use dashboard::ResumenEconomico;
pub use gasto::{
    CategoriaGasto, CrearGastoRequest, Gasto, GastosPaginados, GastosQuery, TipoDocumento,
};
pub use reserva::{
    ActualizarReservaRequest, CrearReservaRequest, EstadoReserva, Reserva, ReservasConteo,
    ReservasPaginadas, ReservasQuery,
};
pub use user::{AuthResponse, LoginRequest, RegisterRequest, User, UserResponse};
pub use venta::{
    CanalVenta, CrearVentaRequest, MetodoPago, Turno, Venta, VentasPaginadas, VentasQuery,
};

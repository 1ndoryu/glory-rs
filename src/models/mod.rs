pub mod common;
mod canal_reserva;
mod cliente;
mod dashboard;
mod etiqueta;
mod gasto;
mod reserva;
mod user;
mod venta;

pub use canal_reserva::{CanalReserva, CrearCanalReservaRequest};
pub use cliente::{
    ActualizarClienteRequest, Cliente, ClientesPaginados, ClientesQuery, CrearClienteRequest,
};
pub use dashboard::ResumenEconomico;
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
pub use user::{AuthResponse, LoginRequest, RegisterRequest, User, UserResponse};
pub use venta::{
    CanalVenta, CrearVentaRequest, MetodoPago, Turno, Venta, VentasPaginadas, VentasQuery,
};

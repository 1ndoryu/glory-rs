pub mod common;
mod api_key;
mod campana;
mod canal_reserva;
mod cliente;
mod configuracion;
mod dashboard;
mod dashboard_reservas;
mod digitalizacion;
mod etiqueta;
mod gasto;
mod notificacion;
mod plano_sala;
mod plantilla_whatsapp;
mod reserva;
mod user;
mod venta;

pub use campana::{
    ActualizarCampanaRequest, Campana, CampanaDestinatario, CampanasPaginadas, CampanasQuery,
    CrearCampanaRequest, SegmentoPreview, SegmentoPreviewQuery, CANALES_VALIDOS,
    SEGMENTOS_VALIDOS,
};
pub use canal_reserva::{CanalReserva, CrearCanalReservaRequest};
pub use cliente::{
    ActualizarClienteRequest, Cliente, ClientesPaginados, ClientesQuery, CrearClienteRequest,
    MergeClientesRequest, MergeClientesResponse,
};
pub use configuracion::{ActualizarConfiguracionRequest, ConfiguracionRestaurante};
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
    CrearMesaRequest, CrearZonaRequest, Mesa, MesaExport, MesaOcupacion, PlanoExport,
    PlanoOcupacion, PlanoOcupacionQuery, PlanoSala, PosicionMesa, ReservaMesa, ZonaConMesas,
    ZonaExport, ZonaOcupacion, ZonaSala,
};
pub use plantilla_whatsapp::{
    ActualizarPlantillaRequest, CrearPlantillaRequest, PlantillaWhatsapp,
    PlantillasPaginadas, PlantillasQuery, CATEGORIAS_PLANTILLA,
};
mod recordatorio;
pub use recordatorio::{
    ActualizarReglaRequest, CrearReglaRequest, HistorialRecordatorios, RecordatorioEnviado,
    RecordatorioEnviadoDetalle, ReglaRecordatorio, ReglasPaginadas, ReglasQuery,
    CANALES_RECORDATORIO,
};
pub use api_key::{
    ApiKey, ApiKeyCreatedResponse, ApiKeyResponse, CamposObligatorios,
    ChatbotBuscarReservasQuery, ChatbotCrearReservaRequest, ChatbotReservaResponse,
    CrearApiKeyRequest, DisponibilidadResponse, FranjaDisponibilidad, RestauranteInfoResponse,
    ZonaResumen,
};
pub use digitalizacion::{DatosDocumentoExtraidos, DigitalizarDocumentoRequest};
pub use notificacion::{Notificacion, NotificacionEvent};

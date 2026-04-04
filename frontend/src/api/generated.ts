/* [263A-10] Barrel re-export  Orval ahora genera un archivo por tag OpenAPI.
 * Este archivo mantiene compatibilidad con imports existentes.
 * Para imports nuevos, importar directamente: '../api/generated/ventas/ventas'
 * [044A-2] Tags renombrados a ASCII para evitar encoding corrupta en Windows.
 * Campañas → Campanas, Gestión → Gestion */
export * from './generated/gestionRestauranteAPI.schemas';
export * from './generated/auth/auth';
export * from './generated/ventas/ventas';
export * from './generated/gastos/gastos';
export * from './generated/reservas/reservas';
export * from './generated/clientes/clientes';
export * from './generated/etiquetas/etiquetas';
export * from './generated/canales/canales';
export * from './generated/dashboard/dashboard';
export * from './generated/plano-sala/plano-sala';
export * from './generated/health/health';
export * from './generated/configuracion/configuracion';
export * from './generated/campanas/campanas';
export * from './generated/plantillas-whatsapp/plantillas-whatsapp';
export * from './generated/recordatorios/recordatorios';
export * from './generated/chatbot/chatbot';
export * from './generated/api-keys/api-keys';
export * from './generated/notificaciones/notificaciones';
export * from './generated/errores/errores';
export * from './generated/admin/admin';

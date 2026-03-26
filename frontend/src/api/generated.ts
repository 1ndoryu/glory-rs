/* [263A-10] Barrel re-export  Orval ahora genera un archivo por tag OpenAPI.
 * Este archivo mantiene compatibilidad con imports existentes.
 * Para imports nuevos, importar directamente: '../api/generated/ventas/ventas' */
export * from './generated/gestiónRestauranteAPI.schemas';
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

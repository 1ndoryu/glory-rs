/* [034A-1] Activar auto_venta_reserva por defecto.
 * El cliente espera que al completar una reserva se genere venta automáticamente.
 * Cambiar default a true y actualizar filas existentes. */

UPDATE configuracion_restaurante SET auto_venta_reserva = true WHERE auto_venta_reserva = false;

ALTER TABLE configuracion_restaurante ALTER COLUMN auto_venta_reserva SET DEFAULT true;

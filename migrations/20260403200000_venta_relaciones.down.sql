DROP INDEX IF EXISTS idx_ventas_cliente_id;
DROP INDEX IF EXISTS idx_ventas_reserva_id;
ALTER TABLE ventas DROP COLUMN IF EXISTS cliente_id;
ALTER TABLE ventas DROP COLUMN IF EXISTS reserva_id;

/* [034A-5] Añadir relaciones opcionales a ventas:
 * - reserva_id: vincula venta con la reserva que la generó (auto-venta)
 * - cliente_id: vincula venta con el cliente
 * Ambas ON DELETE SET NULL para no perder la venta si se borra la reserva/cliente. */
ALTER TABLE ventas ADD COLUMN IF NOT EXISTS reserva_id UUID REFERENCES reservas(id) ON DELETE SET NULL;
ALTER TABLE ventas ADD COLUMN IF NOT EXISTS cliente_id UUID REFERENCES clientes(id) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS idx_ventas_reserva_id ON ventas(reserva_id) WHERE reserva_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_ventas_cliente_id ON ventas(cliente_id) WHERE cliente_id IS NOT NULL;

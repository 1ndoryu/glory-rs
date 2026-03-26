/* 263A-1: Rollback CRM, etiquetas y canales */

ALTER TABLE reservas DROP COLUMN IF EXISTS canal_id;
ALTER TABLE reservas DROP COLUMN IF EXISTS cliente_id;
ALTER TABLE reservas DROP COLUMN IF EXISTS no_show;

DROP TABLE IF EXISTS reservas_etiquetas;
DROP TABLE IF EXISTS clientes_etiquetas;
DROP TABLE IF EXISTS etiquetas;
DROP TABLE IF EXISTS categorias_etiqueta;
DROP TABLE IF EXISTS canales_reserva;
DROP TABLE IF EXISTS clientes;

/* Restaurar constraints originales */
ALTER TABLE ventas DROP CONSTRAINT IF EXISTS ventas_metodo_pago_check;
ALTER TABLE ventas ADD CONSTRAINT ventas_metodo_pago_check
    CHECK (metodo_pago IN ('efectivo', 'tarjeta', 'transferencia'));

ALTER TABLE gastos DROP CONSTRAINT IF EXISTS gastos_metodo_pago_check;
ALTER TABLE gastos ADD CONSTRAINT gastos_metodo_pago_check
    CHECK (metodo_pago IN ('efectivo', 'tarjeta', 'transferencia'));

ALTER TABLE reservas DROP CONSTRAINT IF EXISTS reservas_estado_check;
ALTER TABLE reservas ADD CONSTRAINT reservas_estado_check
    CHECK (estado IN ('pendiente', 'confirmada', 'cancelada', 'completada'));

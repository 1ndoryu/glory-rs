/* 263A-6: Rollback de mejoras vista día */

ALTER TABLE reservas DROP COLUMN IF EXISTS num_mesa;
ALTER TABLE reservas DROP COLUMN IF EXISTS apellidos_cliente;

ALTER TABLE reservas DROP CONSTRAINT IF EXISTS reservas_estado_check;
ALTER TABLE reservas ADD CONSTRAINT reservas_estado_check
    CHECK (estado IN ('pendiente', 'confirmada', 'cancelada', 'completada', 'no_show'));

/* [014A-1] Auto-venta al completar reserva (configurable).
 * [014A-4] Turnos configurables (horas de desayuno, comida, cena).
 * Agregar campos a configuracion_restaurante. */

ALTER TABLE configuracion_restaurante
    ADD COLUMN IF NOT EXISTS auto_venta_reserva BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN IF NOT EXISTS hora_desayuno_inicio TIME NOT NULL DEFAULT '00:00',
    ADD COLUMN IF NOT EXISTS hora_desayuno_fin TIME NOT NULL DEFAULT '12:00',
    ADD COLUMN IF NOT EXISTS hora_comida_inicio TIME NOT NULL DEFAULT '12:00',
    ADD COLUMN IF NOT EXISTS hora_comida_fin TIME NOT NULL DEFAULT '18:00',
    ADD COLUMN IF NOT EXISTS hora_cena_inicio TIME NOT NULL DEFAULT '18:00',
    ADD COLUMN IF NOT EXISTS hora_cena_fin TIME NOT NULL DEFAULT '23:59';

/* [014A-3] Recordatorios post-reserva: cambiar horas_antes a nullable,
 * agregar horas_despues y tipo de regla. */
ALTER TABLE reglas_recordatorio
    ALTER COLUMN horas_antes DROP NOT NULL,
    ADD COLUMN IF NOT EXISTS horas_despues INTEGER CHECK (horas_despues IS NULL OR horas_despues > 0),
    ADD COLUMN IF NOT EXISTS tipo VARCHAR(20) NOT NULL DEFAULT 'antes'
        CHECK (tipo IN ('antes', 'despues'));

/* Constraint: exactamente uno de horas_antes/horas_despues debe tener valor */
ALTER TABLE reglas_recordatorio
    ADD CONSTRAINT chk_horas_antes_o_despues
        CHECK (
            (tipo = 'antes' AND horas_antes IS NOT NULL AND horas_despues IS NULL) OR
            (tipo = 'despues' AND horas_despues IS NOT NULL AND horas_antes IS NULL)
        );

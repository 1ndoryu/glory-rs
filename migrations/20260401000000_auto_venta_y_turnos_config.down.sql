ALTER TABLE reglas_recordatorio DROP CONSTRAINT IF EXISTS chk_horas_antes_o_despues;
ALTER TABLE reglas_recordatorio DROP COLUMN IF EXISTS horas_despues;
ALTER TABLE reglas_recordatorio DROP COLUMN IF EXISTS tipo;
ALTER TABLE reglas_recordatorio ALTER COLUMN horas_antes SET NOT NULL;

ALTER TABLE configuracion_restaurante
    DROP COLUMN IF EXISTS auto_venta_reserva,
    DROP COLUMN IF EXISTS hora_desayuno_inicio,
    DROP COLUMN IF EXISTS hora_desayuno_fin,
    DROP COLUMN IF EXISTS hora_comida_inicio,
    DROP COLUMN IF EXISTS hora_comida_fin,
    DROP COLUMN IF EXISTS hora_cena_inicio,
    DROP COLUMN IF EXISTS hora_cena_fin;

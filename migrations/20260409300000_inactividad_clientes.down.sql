DROP TABLE IF EXISTS envios_inactividad;
DROP TABLE IF EXISTS reglas_inactividad;
ALTER TABLE clientes DROP COLUMN IF EXISTS ultima_visita;

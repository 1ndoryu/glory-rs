/* 263A-6: Mejoras para vista de reservas por día.
   Agrega num_mesa, estado lista_espera, y apellidos_cliente. */

/* Nº de mesa asignada a la reserva (opcional hasta que exista plano de sala) */
ALTER TABLE reservas ADD COLUMN num_mesa INTEGER;

/* Apellidos del cliente directamente en la reserva (para búsquedas rápidas
   sin depender del JOIN a clientes) */
ALTER TABLE reservas ADD COLUMN apellidos_cliente VARCHAR(255) NOT NULL DEFAULT '';

/* Ampliar el CHECK de estado para incluir lista_espera */
ALTER TABLE reservas DROP CONSTRAINT IF EXISTS reservas_estado_check;
ALTER TABLE reservas ADD CONSTRAINT reservas_estado_check
    CHECK (estado IN ('pendiente', 'confirmada', 'cancelada', 'completada', 'no_show', 'lista_espera'));

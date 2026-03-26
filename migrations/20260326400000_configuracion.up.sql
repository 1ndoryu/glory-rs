/* [263A-17] Configuración del restaurante — datos obligatorios al reservar + IVA por defecto.
 * Una fila por usuario (restaurante). Si no existe, se usan defaults. */

CREATE TABLE IF NOT EXISTS configuracion_restaurante (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,

    /* Campos obligatorios al reservar (true = obligatorio) */
    reserva_email_obligatorio BOOLEAN NOT NULL DEFAULT false,
    reserva_telefono_obligatorio BOOLEAN NOT NULL DEFAULT true,
    reserva_nombre_obligatorio BOOLEAN NOT NULL DEFAULT true,
    reserva_apellidos_obligatorio BOOLEAN NOT NULL DEFAULT false,

    /* IVA por defecto del establecimiento */
    iva_por_defecto NUMERIC(5, 2) NOT NULL DEFAULT 10.00,

    /* Nombre del restaurante (para emails, reportes) */
    nombre_restaurante VARCHAR(255) NOT NULL DEFAULT '',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

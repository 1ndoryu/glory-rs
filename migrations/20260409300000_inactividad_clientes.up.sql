/* [094A-5] Automatización mensajes a clientes inactivos.
 * ultima_visita en clientes se actualiza al completar reserva.
 * reglas_inactividad define cuándo y cómo contactar. */

ALTER TABLE clientes
    ADD COLUMN IF NOT EXISTS ultima_visita TIMESTAMPTZ;

/* Actualizar ultima_visita con la fecha de la última reserva completada por cliente */
UPDATE clientes c
SET ultima_visita = sub.max_fecha
FROM (
    SELECT r.cliente_id, MAX(r.fecha) AS max_fecha
    FROM reservas r
    WHERE r.estado = 'completada' AND r.cliente_id IS NOT NULL
    GROUP BY r.cliente_id
) sub
WHERE c.id = sub.cliente_id;

CREATE TABLE IF NOT EXISTS reglas_inactividad (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    nombre VARCHAR(100) NOT NULL DEFAULT '',
    dias_inactividad INT NOT NULL CHECK (dias_inactividad > 0),
    canal VARCHAR(20) NOT NULL DEFAULT 'whatsapp' CHECK (canal IN ('email', 'sms', 'whatsapp')),
    mensaje_plantilla TEXT NOT NULL DEFAULT '',
    activa BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

/* Registro de envíos para evitar re-envíos */
CREATE TABLE IF NOT EXISTS envios_inactividad (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    regla_id UUID NOT NULL REFERENCES reglas_inactividad(id) ON DELETE CASCADE,
    cliente_id UUID NOT NULL REFERENCES clientes(id) ON DELETE CASCADE,
    enviado_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    estado VARCHAR(20) NOT NULL DEFAULT 'enviado',
    UNIQUE(regla_id, cliente_id)
);

CREATE INDEX idx_reglas_inactividad_user ON reglas_inactividad(user_id);
CREATE INDEX idx_envios_inactividad_regla ON envios_inactividad(regla_id);
CREATE INDEX idx_clientes_ultima_visita ON clientes(ultima_visita);

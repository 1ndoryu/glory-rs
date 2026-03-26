/* [263A-25] Reglas de recordatorio automático de reservas.
 * Cada restaurante configura N reglas: cuántas horas antes enviar, por qué canal.
 * recordatorios_enviados rastrea qué ya se envió para evitar duplicados. */

CREATE TABLE reglas_recordatorio (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    nombre VARCHAR(255) NOT NULL,
    horas_antes INTEGER NOT NULL CHECK (horas_antes > 0),
    canal VARCHAR(20) NOT NULL CHECK (canal IN ('sms', 'email', 'whatsapp')),
    mensaje_plantilla TEXT NOT NULL DEFAULT '',
    activa BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE recordatorios_enviados (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    regla_id UUID NOT NULL REFERENCES reglas_recordatorio(id) ON DELETE CASCADE,
    reserva_id UUID NOT NULL REFERENCES reservas(id) ON DELETE CASCADE,
    canal VARCHAR(20) NOT NULL,
    estado VARCHAR(20) NOT NULL DEFAULT 'enviado' CHECK (estado IN ('enviado', 'fallido')),
    enviado_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    error_mensaje TEXT,
    UNIQUE (regla_id, reserva_id)
);

CREATE INDEX idx_reglas_recordatorio_user_id ON reglas_recordatorio(user_id);
CREATE INDEX idx_recordatorios_enviados_reserva ON recordatorios_enviados(reserva_id);
CREATE INDEX idx_recordatorios_enviados_regla ON recordatorios_enviados(regla_id);

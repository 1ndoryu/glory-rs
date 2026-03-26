/* [263A-23] Módulo de Marketing — Campañas manuales multi-canal.
 * Tablas: campanas (master) + campana_destinatarios (recipients).
 * Segmentación por actividad del cliente (última reserva).
 * Canales soportados: SMS, email, WhatsApp. */

CREATE TABLE campanas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    nombre VARCHAR(255) NOT NULL,
    descripcion_interna TEXT NOT NULL DEFAULT '',
    cuerpo_mensaje TEXT NOT NULL DEFAULT '',
    canales TEXT[] NOT NULL DEFAULT '{}',
    segmento VARCHAR(50) NOT NULL DEFAULT 'todos',
    incluir_baja BOOLEAN NOT NULL DEFAULT false,
    telefono_baja VARCHAR(100) NOT NULL DEFAULT '',
    estado VARCHAR(20) NOT NULL DEFAULT 'borrador',
    total_destinatarios INT NOT NULL DEFAULT 0,
    total_enviados INT NOT NULL DEFAULT 0,
    total_fallidos INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_campanas_user_id ON campanas(user_id);
CREATE INDEX idx_campanas_estado ON campanas(user_id, estado);

CREATE TABLE campana_destinatarios (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    campana_id UUID NOT NULL REFERENCES campanas(id) ON DELETE CASCADE,
    cliente_id UUID NOT NULL REFERENCES clientes(id) ON DELETE CASCADE,
    canal VARCHAR(20) NOT NULL,
    estado VARCHAR(20) NOT NULL DEFAULT 'pendiente',
    enviado_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_campana_dest_campana ON campana_destinatarios(campana_id);
CREATE INDEX idx_campana_dest_cliente ON campana_destinatarios(cliente_id);
CREATE INDEX idx_campana_dest_estado ON campana_destinatarios(campana_id, estado);

/* Constraint: canales válidos y segmento válido */
ALTER TABLE campanas ADD CONSTRAINT chk_campana_estado
    CHECK (estado IN ('borrador', 'enviada', 'cancelada'));

ALTER TABLE campanas ADD CONSTRAINT chk_campana_segmento
    CHECK (segmento IN ('habitual', 'sin_1m', 'sin_3m', 'sin_6m', 'sin_9m', 'sin_1a', 'sin_mas_1a', 'todos'));

ALTER TABLE campana_destinatarios ADD CONSTRAINT chk_dest_canal
    CHECK (canal IN ('sms', 'email', 'whatsapp'));

ALTER TABLE campana_destinatarios ADD CONSTRAINT chk_dest_estado
    CHECK (estado IN ('pendiente', 'enviado', 'fallido'));

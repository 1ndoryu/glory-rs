/* [283A-20] Tabla de notificaciones en tiempo real.
 * Las notificaciones se generan automáticamente cuando el chatbot crea/cancela reservas
 * u ocurren otras acciones externas. Se emiten via SSE al panel del usuario. */
CREATE TABLE notificaciones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tipo VARCHAR(50) NOT NULL,
    titulo VARCHAR(255) NOT NULL,
    mensaje TEXT NOT NULL,
    leida BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_notificaciones_user_leida ON notificaciones(user_id, leida);
CREATE INDEX idx_notificaciones_user_created ON notificaciones(user_id, created_at DESC);

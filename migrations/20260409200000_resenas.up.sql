/* [094A-4] Tabla de reseñas (review gating).
 * Después de una reserva completada, se envía enlace a landing de reseña.
 * Si el cliente puntúa 4-5, se le redirige a Google Business.
 * Si puntúa 1-3, el feedback queda interno. */

CREATE TABLE IF NOT EXISTS resenas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reserva_id UUID REFERENCES reservas(id) ON DELETE SET NULL,
    cliente_id UUID REFERENCES clientes(id) ON DELETE SET NULL,
    token VARCHAR(64) NOT NULL UNIQUE,
    puntuacion SMALLINT CHECK (puntuacion BETWEEN 1 AND 5),
    comentario TEXT DEFAULT '',
    redirigido_google BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    respondida_at TIMESTAMPTZ
);

CREATE INDEX idx_resenas_user ON resenas(user_id);
CREATE INDEX idx_resenas_token ON resenas(token);
CREATE INDEX idx_resenas_cliente ON resenas(cliente_id);

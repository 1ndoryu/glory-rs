/* [094A-7] Tabla de paredes en plano de sala.
 * Permite dibujar paredes (rectángulos de color) para representar la distribución física del local. */
CREATE TABLE IF NOT EXISTS paredes_sala (
    id UUID PRIMARY KEY,
    zona_id UUID NOT NULL REFERENCES zonas_sala(id) ON DELETE CASCADE,
    pos_x INT NOT NULL DEFAULT 0,
    pos_y INT NOT NULL DEFAULT 0,
    ancho INT NOT NULL DEFAULT 100,
    alto INT NOT NULL DEFAULT 20,
    rotacion INT NOT NULL DEFAULT 0,
    color VARCHAR(20) NOT NULL DEFAULT '#6b7280',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

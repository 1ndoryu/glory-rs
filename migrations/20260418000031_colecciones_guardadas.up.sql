-- [174A-66] Saved collections (bookmarks).
-- Tabla legacy `colecciones_guardadas` con coleccion_id INT incompatible con
-- nueva `colecciones.id BIGINT`. Drop limpio + recreación.
DROP TABLE IF EXISTS colecciones_guardadas CASCADE;

CREATE TABLE colecciones_guardadas (
    usuario_id     INT     NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    coleccion_id   BIGINT  NOT NULL REFERENCES colecciones(id) ON DELETE CASCADE,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (usuario_id, coleccion_id)
);

CREATE INDEX idx_colecciones_guardadas_usuario ON colecciones_guardadas (usuario_id, created_at DESC);
CREATE INDEX idx_colecciones_guardadas_coleccion ON colecciones_guardadas (coleccion_id);

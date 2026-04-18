-- [174A-64] Colecciones (playlists/folders polimórficas) + M2M con orden.
--
-- Soporta:
--   * Carpetas anidadas (parent_id, profundidad chequeada en código).
--   * Visibilidad pública/privada.
--   * Optimistic locking via columna `version` (incrementada en UPDATE en código).
--   * Soft delete (eliminado_en).
--   * Contador denormalizado total_samples.
--
-- Tabla join `coleccion_samples` con `orden` para drag&drop y `added_at` para historial.
-- PK compuesta evita duplicados; índice (coleccion_id, orden) acelera listado ordenado.

CREATE TABLE IF NOT EXISTS colecciones (
    id              BIGSERIAL PRIMARY KEY,
    usuario_id      INT NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    nombre          TEXT NOT NULL,
    descripcion     TEXT,
    publica         BOOLEAN NOT NULL DEFAULT TRUE,
    parent_id       BIGINT REFERENCES colecciones(id) ON DELETE SET NULL,
    imagen_url      TEXT,
    version         INT NOT NULL DEFAULT 1,
    total_samples   INT NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    eliminado_en    TIMESTAMPTZ,
    CHECK (parent_id IS NULL OR parent_id <> id)
);

CREATE INDEX IF NOT EXISTS idx_colecciones_usuario
    ON colecciones(usuario_id) WHERE eliminado_en IS NULL;
CREATE INDEX IF NOT EXISTS idx_colecciones_parent
    ON colecciones(parent_id) WHERE eliminado_en IS NULL;
-- Nombre único por (usuario, parent_id) — usa COALESCE porque NULL != NULL en índices únicos.
CREATE UNIQUE INDEX IF NOT EXISTS idx_colecciones_nombre_unico
    ON colecciones(usuario_id, COALESCE(parent_id, 0), nombre)
    WHERE eliminado_en IS NULL;

CREATE TABLE IF NOT EXISTS coleccion_samples (
    coleccion_id BIGINT NOT NULL REFERENCES colecciones(id) ON DELETE CASCADE,
    sample_id    INT NOT NULL REFERENCES samples(id) ON DELETE CASCADE,
    orden        INT NOT NULL DEFAULT 0,
    added_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (coleccion_id, sample_id)
);

CREATE INDEX IF NOT EXISTS idx_coleccion_samples_orden
    ON coleccion_samples(coleccion_id, orden);
CREATE INDEX IF NOT EXISTS idx_coleccion_samples_sample
    ON coleccion_samples(sample_id);

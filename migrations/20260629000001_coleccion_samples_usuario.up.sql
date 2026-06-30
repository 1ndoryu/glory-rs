-- [296A-1] Restaurar modelo legacy: 1 sample = 1 coleccion por usuario.
-- En PHP, UNIQUE(usuario_id, sample_id) garantizaba que un sample solo
-- perteneciera a una coleccion a la vez (necesario para sync Desktop/Tauri
-- donde 1 archivo = 1 carpeta). El puerto Rust original cambio a
-- PRIMARY KEY(coleccion_id, sample_id) sin usuario_id, lo que permitia
-- duplicados. Esta migracion restaura la restriccion.

-- 1. Agregar columna usuario_id (nullable temporalmente).
ALTER TABLE coleccion_samples ADD COLUMN IF NOT EXISTS usuario_id INT;

-- 2. Backfill desde colecciones.usuario_id.
UPDATE coleccion_samples cs
   SET usuario_id = c.usuario_id
  FROM colecciones c
 WHERE cs.coleccion_id = c.id
   AND cs.usuario_id IS NULL;

-- 3. NOT NULL + FK.
ALTER TABLE coleccion_samples
    ALTER COLUMN usuario_id SET NOT NULL,
    ADD CONSTRAINT fk_coleccion_samples_usuario
        FOREIGN KEY (usuario_id) REFERENCES usuarios_ext(id) ON DELETE CASCADE;

-- 4. UNIQUE(usuario_id, sample_id) — la constraint clave del modelo legacy.
CREATE UNIQUE INDEX IF NOT EXISTS idx_coleccion_samples_usuario_sample
    ON coleccion_samples(usuario_id, sample_id);

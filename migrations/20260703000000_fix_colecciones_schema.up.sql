/* [037A-1] Fix: alinear colecciones y coleccion_samples con el schema canónico.
 *
 * Problema: dos migraciones (20260417000003 y 20260418000030) crean colecciones
 * con CREATE TABLE IF NOT EXISTS. La primera (samples_core) corre antes y crea
 * la tabla sin eliminado_en, BIGSERIAL, CHECK ni índices parciales. La segunda
 * (colecciones) es no-op. Resultado: schema viejo sin soft-delete support.
 *
 * Además, coleccion_samples quedó con 'posicion' en vez de 'orden' (renombrado
 * en el schema canónico de 20260418000030) y sin índices parciales.
 *
 * Este fix es IDEMPOTENTE (usa IF NOT EXISTS / DO blocks). */

-- 1. Renombrar posicion → orden en coleccion_samples (si existe).
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'coleccion_samples' AND column_name = 'posicion'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'coleccion_samples' AND column_name = 'orden'
    ) THEN
        ALTER TABLE coleccion_samples RENAME COLUMN posicion TO orden;
    END IF;
END $$;

-- 2. Asegurar NOT NULL en orden.
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'coleccion_samples'
          AND column_name = 'orden'
          AND is_nullable = 'YES'
    ) THEN
        ALTER TABLE coleccion_samples ALTER COLUMN orden SET NOT NULL;
    END IF;
END $$;

-- 3. Índice para orden (si no existe).
CREATE INDEX IF NOT EXISTS idx_coleccion_samples_orden
    ON coleccion_samples(coleccion_id, orden);

-- 4. CHECK constraint en colecciones (parent_id <> id).
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
        WHERE conrelid = 'colecciones'::regclass
          AND conname = 'chk_colecciones_no_self_parent'
    ) THEN
        ALTER TABLE colecciones
            ADD CONSTRAINT chk_colecciones_no_self_parent
            CHECK (parent_id IS NULL OR parent_id <> id);
    END IF;
END $$;

-- 5. Índices parciales con WHERE eliminado_en IS NULL.
--    Recrear con nuevo nombre para evitar conflictos con los viejos.
CREATE INDEX IF NOT EXISTS idx_colecciones_usuario_soft
    ON colecciones(usuario_id) WHERE eliminado_en IS NULL;

CREATE INDEX IF NOT EXISTS idx_colecciones_parent_soft
    ON colecciones(parent_id) WHERE eliminado_en IS NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_colecciones_nombre_unico_soft
    ON colecciones(usuario_id, COALESCE(parent_id, 0), nombre)
    WHERE eliminado_en IS NULL;

-- 6. Índice para coleccion_samples(usuario_id) si no existe.
CREATE INDEX IF NOT EXISTS idx_coleccion_samples_usuario_sample
    ON coleccion_samples(usuario_id, sample_id);

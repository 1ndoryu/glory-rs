/* [037A-1] Revert: deshacer cambios de schema de colecciones. */

-- Eliminar índices parciales nuevos.
DROP INDEX IF EXISTS idx_colecciones_usuario_soft;
DROP INDEX IF EXISTS idx_colecciones_parent_soft;
DROP INDEX IF EXISTS idx_colecciones_nombre_unico_soft;
DROP INDEX IF EXISTS idx_coleccion_samples_usuario_sample;
DROP INDEX IF EXISTS idx_coleccion_samples_orden;

-- Eliminar CHECK constraint.
ALTER TABLE colecciones DROP CONSTRAINT IF EXISTS chk_colecciones_no_self_parent;

-- Revertir orden → posicion.
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'coleccion_samples' AND column_name = 'orden'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'coleccion_samples' AND column_name = 'posicion'
    ) THEN
        ALTER TABLE coleccion_samples RENAME COLUMN orden TO posicion;
    END IF;
END $$;

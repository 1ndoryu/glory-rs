/* [254A-3] Compatibilidad legacy: algunas bases creadas antes del port Rust tienen colecciones sin slug.
 * El frontend legacy navega por /colecciones/por-slug/{slug}; asegurar la columna evita 400/404
 * y mantiene el contrato SEO de PHP sin depender de recrear la tabla. */
ALTER TABLE colecciones
    ADD COLUMN IF NOT EXISTS slug VARCHAR(255);

CREATE UNIQUE INDEX IF NOT EXISTS idx_colecciones_slug
    ON colecciones (slug) WHERE slug IS NOT NULL;

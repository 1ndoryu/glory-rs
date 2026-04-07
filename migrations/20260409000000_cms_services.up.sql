/* [074A-8] Añadir campos CMS a services: SEO, imagen, galería, skills, contenido rich-text.
 * Estos campos permiten editar servicios desde el panel admin sin tocar código. */
ALTER TABLE services ADD COLUMN IF NOT EXISTS image_url VARCHAR(500);
ALTER TABLE services ADD COLUMN IF NOT EXISTS gallery JSONB NOT NULL DEFAULT '[]';
ALTER TABLE services ADD COLUMN IF NOT EXISTS skills JSONB NOT NULL DEFAULT '[]';
ALTER TABLE services ADD COLUMN IF NOT EXISTS content TEXT;
ALTER TABLE services ADD COLUMN IF NOT EXISTS meta_title VARCHAR(255);
ALTER TABLE services ADD COLUMN IF NOT EXISTS meta_description TEXT;
ALTER TABLE services ADD COLUMN IF NOT EXISTS status VARCHAR(20) NOT NULL DEFAULT 'published';
ALTER TABLE services ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

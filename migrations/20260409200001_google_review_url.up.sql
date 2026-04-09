/* [094A-4] Campo google_review_url en configuración. */
ALTER TABLE configuracion_restaurante
    ADD COLUMN IF NOT EXISTS google_review_url TEXT NOT NULL DEFAULT '';

-- [264A-1] Tabla key/value para configuracion runtime de la app.
-- Sirve como contrato entre el backend Rust y el scraper Python:
-- ambos leen las mismas claves para coordinar intervalos y tamano de lote
-- sin necesidad de redeploy. El backend expone endpoints admin para mutar
-- estos valores en caliente.

CREATE TABLE IF NOT EXISTS app_config (
    clave        VARCHAR(64)  PRIMARY KEY,
    valor        JSONB        NOT NULL,
    descripcion  TEXT,
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE app_config IS
    'Configuracion runtime compartida entre backend Rust y scraper Python';

-- Defaults conservadores. ON CONFLICT DO NOTHING para que aplicar la
-- migracion dos veces (o tras un reset parcial) no sobrescriba valores
-- ya ajustados en produccion.
INSERT INTO app_config (clave, valor, descripcion) VALUES
    ('extraccion_intervalo_seg', '60'::jsonb,
     'Segundos entre ciclos del pipeline de extraccion (yt-dlp + recorte)'),
    ('extraccion_lote_size', '20'::jsonb,
     'Cantidad maxima de items que el scraper procesa por ciclo de extraccion'),
    ('extraccion_enabled', 'true'::jsonb,
     'Si esta en false el scraper salta los ciclos de extraccion'),
    ('scraping_intervalo_seg', '900'::jsonb,
     'Segundos entre ciclos de scraping de WhoSampled'),
    ('scraping_lote_size', '10'::jsonb,
     'Cantidad maxima de paginas que se scrapean por ciclo'),
    ('scraping_enabled', 'true'::jsonb,
     'Si esta en false el scraper salta los ciclos de scraping')
ON CONFLICT (clave) DO NOTHING;

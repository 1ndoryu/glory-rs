-- [274A-1] El scraper auto-encola track overviews al insertar relaciones
-- (sample_detail.py::_seguir_track_overviews). El CHECK original de scraping_log
-- no incluia 'track_overview' y rompia el dedup, dejando metadata extra
-- (genero, tags, youtube_id, spotify_id) sin actualizar.
-- Solucion: agregar 'track_overview' a la whitelist sin tocar el resto.
BEGIN;

ALTER TABLE scraping_log DROP CONSTRAINT IF EXISTS scraping_log_tipo_pagina_check;

ALTER TABLE scraping_log
    ADD CONSTRAINT scraping_log_tipo_pagina_check
    CHECK (tipo_pagina IN (
        'hot_samples', 'hot_covers', 'hot_remixes',
        'sample_detail', 'cover_detail', 'remix_detail',
        'artist', 'track', 'track_overview', 'track_samples', 'track_sampled',
        'browse_year', 'browse_genre'
    ));

COMMIT;

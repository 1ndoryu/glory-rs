BEGIN;

CREATE TABLE IF NOT EXISTS scraping_log (
    id                SERIAL PRIMARY KEY,
    url               VARCHAR(1000) UNIQUE NOT NULL,
    tipo_pagina       VARCHAR(30) NOT NULL
                      CHECK (tipo_pagina IN (
                          'hot_samples', 'hot_covers', 'hot_remixes',
                          'sample_detail', 'cover_detail', 'remix_detail',
                          'artist', 'track', 'track_samples', 'track_sampled',
                          'browse_year', 'browse_genre'
                      )),
    estado            VARCHAR(20) DEFAULT 'pendiente'
                      CHECK (estado IN ('pendiente', 'procesado', 'error', 'skip')),
    intentos          SMALLINT DEFAULT 0,
    bytes_descargados INT DEFAULT 0,
    error_mensaje     TEXT,
    re_scrapeable     BOOLEAN DEFAULT FALSE,
    proximo_rescrape  TIMESTAMPTZ,
    veces_rescrapeado SMALLINT DEFAULT 0,
    procesado_at      TIMESTAMPTZ,
    created_at        TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_scraping_estado  ON scraping_log (estado);
CREATE INDEX IF NOT EXISTS idx_scraping_tipo    ON scraping_log (tipo_pagina);
CREATE INDEX IF NOT EXISTS idx_scraping_rescrape
    ON scraping_log (proximo_rescrape)
    WHERE re_scrapeable = TRUE AND proximo_rescrape IS NOT NULL;

CREATE TABLE IF NOT EXISTS cola_extraccion_samples (
    id                   SERIAL PRIMARY KEY,
    relacion_id          INT NOT NULL REFERENCES relaciones_sample(id),
    youtube_id           VARCHAR(20),
    spotify_id           VARCHAR(30),
    timing_inicio_seg    SMALLINT NOT NULL,
    bpm_detectado        SMALLINT,
    duracion_compas_seg  DECIMAL(5,2),
    compas_inicio_seg    DECIMAL(5,2),
    compas_fin_seg       DECIMAL(5,2),
    lado                 VARCHAR(10) DEFAULT 'fuente'
                         CHECK (lado IN ('fuente', 'destino')),
    estado               VARCHAR(20) DEFAULT 'pendiente'
                         CHECK (estado IN (
                             'pendiente', 'descargando', 'analizando', 'recortando',
                             'extraido', 'completado', 'error', 'revision_humana', 'unificado'
                         )),
    sample_id            INT REFERENCES samples(id),
    ruta_audio_extraido  TEXT,
    ruta_audio_completo  TEXT,
    metadata_extraccion  JSONB,
    error_mensaje        TEXT,
    intentos             SMALLINT DEFAULT 0,
    proximo_intento_at   TIMESTAMPTZ NULL,
    procesado_at         TIMESTAMPTZ,
    created_at           TIMESTAMPTZ DEFAULT NOW(),
    CONSTRAINT uq_cola_relacion_lado UNIQUE (relacion_id, lado),
    CONSTRAINT chk_cola_tiene_fuente_audio
        CHECK (youtube_id IS NOT NULL OR spotify_id IS NOT NULL)
);

CREATE INDEX IF NOT EXISTS idx_cola_estado     ON cola_extraccion_samples (estado);
CREATE INDEX IF NOT EXISTS idx_cola_relacion   ON cola_extraccion_samples (relacion_id);
CREATE INDEX IF NOT EXISTS idx_cola_extraccion_sample_id
    ON cola_extraccion_samples (sample_id);
CREATE INDEX IF NOT EXISTS idx_cola_extraccion_backoff
    ON cola_extraccion_samples (proximo_intento_at)
    WHERE estado = 'pendiente' AND proximo_intento_at IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_cola_ext_dedup_youtube_timing
    ON cola_extraccion_samples (youtube_id, timing_inicio_seg)
    WHERE estado IN ('completado', 'unificado') AND sample_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS cola_procesamiento_ia (
    id               SERIAL PRIMARY KEY,
    tipo             TEXT NOT NULL
                     CHECK (tipo IN ('sample', 'comentario', 'publicacion')),
    entidad_id       INTEGER NOT NULL,
    operacion        TEXT NOT NULL
                     CHECK (operacion IN (
                         'analisis_audio', 'moderacion_texto',
                         'moderacion_imagen', 'moderacion_completa'
                     )),
    estado           TEXT NOT NULL DEFAULT 'pendiente'
                     CHECK (estado IN (
                         'pendiente', 'procesando', 'completado',
                         'error_reintento', 'error_final'
                     )),
    intentos         INTEGER NOT NULL DEFAULT 0,
    max_intentos     INTEGER NOT NULL DEFAULT 30,
    ultimo_error     TEXT,
    proximo_intento  TIMESTAMPTZ,
    metadata         JSONB NOT NULL DEFAULT '{}',
    procesado_at     TIMESTAMPTZ,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_cola_ia_estado_proximo
    ON cola_procesamiento_ia (estado, proximo_intento, created_at)
    WHERE estado IN ('pendiente', 'error_reintento');
CREATE INDEX IF NOT EXISTS idx_cola_ia_entidad
    ON cola_procesamiento_ia (tipo, entidad_id);

CREATE TABLE IF NOT EXISTS lotes_procesamiento (
    id                  SERIAL PRIMARY KEY,
    tipo                VARCHAR(20) NOT NULL CHECK (tipo IN ('extraccion', 'scraping')),
    estado              VARCHAR(20) NOT NULL DEFAULT 'ejecutando'
                        CHECK (estado IN ('ejecutando', 'completado', 'error', 'detenido')),
    iniciado_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completado_at       TIMESTAMPTZ,
    exitosos            INT NOT NULL DEFAULT 0,
    fallidos            INT NOT NULL DEFAULT 0,
    recortes            INT NOT NULL DEFAULT 0,
    samples_publicados  INT NOT NULL DEFAULT 0,
    canciones_nuevas    INT NOT NULL DEFAULT 0,
    sampleos_nuevos     INT NOT NULL DEFAULT 0,
    error_mensaje       TEXT,
    metadata            JSONB DEFAULT '{}'
);

CREATE INDEX IF NOT EXISTS idx_lotes_tipo         ON lotes_procesamiento (tipo);
CREATE INDEX IF NOT EXISTS idx_lotes_iniciado     ON lotes_procesamiento (iniciado_at DESC);
CREATE INDEX IF NOT EXISTS idx_lotes_tipo_estado  ON lotes_procesamiento (tipo, estado);

COMMIT;



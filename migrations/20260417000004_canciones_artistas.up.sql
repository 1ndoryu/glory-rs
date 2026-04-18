BEGIN;

CREATE TABLE IF NOT EXISTS artistas_musicales (
    id              SERIAL PRIMARY KEY,
    nombre          VARCHAR(300) NOT NULL,
    slug            VARCHAR(350) UNIQUE NOT NULL,
    imagen_url      TEXT,
    whosampled_slug VARCHAR(350) UNIQUE,
    musicbrainz_id  VARCHAR(36),
    metadata        JSONB DEFAULT '{}',
    prioridad       SMALLINT DEFAULT 0,
    total_canciones INT DEFAULT 0,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_artistas_slug      ON artistas_musicales (slug);
CREATE INDEX IF NOT EXISTS idx_artistas_ws_slug   ON artistas_musicales (whosampled_slug);
CREATE INDEX IF NOT EXISTS idx_artistas_prioridad ON artistas_musicales (prioridad DESC) WHERE prioridad > 0;
CREATE INDEX IF NOT EXISTS idx_artistas_nombre_trgm ON artistas_musicales USING GIN (nombre gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_artistas_nombre_fts
    ON artistas_musicales USING GIN (to_tsvector('simple', nombre));

CREATE TABLE IF NOT EXISTS canciones (
    id                SERIAL PRIMARY KEY,
    titulo            VARCHAR(500) NOT NULL,
    slug              VARCHAR(550) UNIQUE NOT NULL,
    artista_id        INT NOT NULL REFERENCES artistas_musicales(id),
    album             VARCHAR(500),
    sello             VARCHAR(200),
    anio              SMALLINT,
    duracion_segundos SMALLINT,
    genero            VARCHAR(100),
    youtube_id        VARCHAR(20),
    spotify_id        VARCHAR(30) DEFAULT NULL,
    imagen_url        TEXT,
    whosampled_url    VARCHAR(500) UNIQUE,
    bpm               SMALLINT,
    tonalidad         VARCHAR(5),
    metadata          JSONB DEFAULT '{}',
    total_sampleada   INT DEFAULT 0,
    total_samplea     INT DEFAULT 0,
    total_likes       INT DEFAULT 0,
    total_comentarios INT DEFAULT 0,
    created_at        TIMESTAMPTZ DEFAULT NOW(),
    updated_at        TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_canciones_artista_id        ON canciones (artista_id);
CREATE INDEX IF NOT EXISTS idx_canciones_slug              ON canciones (slug);
CREATE INDEX IF NOT EXISTS idx_canciones_ws                ON canciones (whosampled_url);
CREATE INDEX IF NOT EXISTS idx_canciones_anio              ON canciones (anio);
CREATE INDEX IF NOT EXISTS idx_canciones_youtube           ON canciones (youtube_id);
CREATE INDEX IF NOT EXISTS idx_canciones_genero            ON canciones (genero) WHERE genero IS NOT NULL AND genero != '';
CREATE INDEX IF NOT EXISTS idx_canciones_total_sampleada   ON canciones (total_sampleada DESC);
CREATE INDEX IF NOT EXISTS idx_canciones_total_likes       ON canciones (total_likes DESC);
CREATE INDEX IF NOT EXISTS idx_canciones_titulo_trgm       ON canciones USING GIN (titulo gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_canciones_busqueda_fts
    ON canciones USING GIN (to_tsvector('simple', titulo || ' ' || COALESCE(album, '')));

CREATE TABLE IF NOT EXISTS canciones_artistas (
    cancion_id INT NOT NULL REFERENCES canciones(id) ON DELETE CASCADE,
    artista_id INT NOT NULL REFERENCES artistas_musicales(id) ON DELETE CASCADE,
    rol        VARCHAR(20) NOT NULL DEFAULT 'principal'
               CHECK (rol IN ('principal', 'featuring', 'producer')),
    PRIMARY KEY (cancion_id, artista_id, rol)
);

CREATE INDEX IF NOT EXISTS idx_ca_artista ON canciones_artistas (artista_id);

CREATE TABLE IF NOT EXISTS relaciones_sample (
    id                  SERIAL PRIMARY KEY,
    cancion_destino_id  INT NOT NULL REFERENCES canciones(id),
    cancion_fuente_id   INT NOT NULL REFERENCES canciones(id),
    whosampled_id       INT UNIQUE,
    tipo_relacion       VARCHAR(20) NOT NULL DEFAULT 'sample'
                        CHECK (tipo_relacion IN ('sample', 'cover', 'remix', 'interpolation')),
    tipo_elemento       VARCHAR(50) DEFAULT 'multiple_elements'
                        CHECK (tipo_elemento IN (
                            'hook_riff', 'vocals_lyrics', 'drums', 'bass',
                            'keys_synth', 'sound_effect', 'multiple_elements', 'other'
                        )),
    timings_destino     JSONB DEFAULT '[]',
    timings_fuente      JSONB DEFAULT '[]',
    aparece_en_todo     BOOLEAN DEFAULT FALSE,
    sample_id           INT REFERENCES samples(id) ON DELETE SET NULL,
    sample_fuente_id    INT REFERENCES samples(id) ON DELETE SET NULL,
    sample_destino_id   INT REFERENCES samples(id) ON DELETE SET NULL,
    votos_total         INT DEFAULT 0,
    votos_promedio      DECIMAL(2,1) DEFAULT 0,
    fuente              VARCHAR(20) DEFAULT 'scraping'
                        CHECK (fuente IN ('scraping', 'comunidad', 'musicbrainz', 'import')),
    contribuidor_id     INT REFERENCES usuarios_ext(id),
    verificada          BOOLEAN DEFAULT FALSE,
    total_likes         INT DEFAULT 0,
    total_comentarios   INT DEFAULT 0,
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE (cancion_destino_id, cancion_fuente_id, tipo_relacion)
);

CREATE INDEX IF NOT EXISTS idx_rel_destino       ON relaciones_sample (cancion_destino_id);
CREATE INDEX IF NOT EXISTS idx_rel_fuente        ON relaciones_sample (cancion_fuente_id);
CREATE INDEX IF NOT EXISTS idx_rel_tipo          ON relaciones_sample (tipo_relacion);
CREATE INDEX IF NOT EXISTS idx_rel_sample        ON relaciones_sample (sample_id);
CREATE INDEX IF NOT EXISTS idx_rel_verificada    ON relaciones_sample (verificada);
CREATE INDEX IF NOT EXISTS idx_rel_ws            ON relaciones_sample (whosampled_id);
CREATE INDEX IF NOT EXISTS idx_rel_destino_tipo  ON relaciones_sample (cancion_destino_id, tipo_relacion);
CREATE INDEX IF NOT EXISTS idx_rel_fuente_tipo   ON relaciones_sample (cancion_fuente_id, tipo_relacion);
CREATE INDEX IF NOT EXISTS idx_rel_fuente_tipo_recursivo_opt
    ON relaciones_sample (cancion_fuente_id, tipo_relacion)
    INCLUDE (cancion_destino_id);
CREATE INDEX IF NOT EXISTS idx_rel_verificada_reciente
    ON relaciones_sample (verificada, created_at DESC) WHERE verificada = TRUE;
CREATE INDEX IF NOT EXISTS idx_relaciones_sample_fuente_id
    ON relaciones_sample (sample_fuente_id) WHERE sample_fuente_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_relaciones_sample_destino_id
    ON relaciones_sample (sample_destino_id) WHERE sample_destino_id IS NOT NULL;

/* FKs diferidas de samples → canciones / relaciones_sample (definidas como columnas en 0003) */
ALTER TABLE samples
    ADD CONSTRAINT samples_cancion_origen_fk
    FOREIGN KEY (cancion_origen_id) REFERENCES canciones(id) ON DELETE SET NULL;

ALTER TABLE samples
    ADD CONSTRAINT samples_relacion_sampleo_fk
    FOREIGN KEY (relacion_sampleo_id) REFERENCES relaciones_sample(id) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS idx_samples_cancion_origen
    ON samples (cancion_origen_id) WHERE cancion_origen_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_samples_relacion_sampleo
    ON samples (relacion_sampleo_id) WHERE relacion_sampleo_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_samples_cancion_activo_preview
    ON samples (cancion_origen_id)
    WHERE estado = 'activo' AND ruta_preview IS NOT NULL;

/* Triggers de contadores total_sampleada / total_samplea (v029) */
CREATE OR REPLACE FUNCTION trg_actualizar_contadores_relacion()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        IF NEW.tipo_relacion = 'sample' THEN
            UPDATE canciones SET total_samplea = total_samplea + 1
                WHERE id = NEW.cancion_destino_id;
            UPDATE canciones SET total_sampleada = total_sampleada + 1
                WHERE id = NEW.cancion_fuente_id;
        END IF;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        IF OLD.tipo_relacion = 'sample' THEN
            UPDATE canciones SET total_samplea = GREATEST(total_samplea - 1, 0)
                WHERE id = OLD.cancion_destino_id;
            UPDATE canciones SET total_sampleada = GREATEST(total_sampleada - 1, 0)
                WHERE id = OLD.cancion_fuente_id;
        END IF;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_relaciones_contadores ON relaciones_sample;
CREATE TRIGGER trg_relaciones_contadores
    AFTER INSERT OR DELETE ON relaciones_sample
    FOR EACH ROW EXECUTE FUNCTION trg_actualizar_contadores_relacion();

COMMIT;



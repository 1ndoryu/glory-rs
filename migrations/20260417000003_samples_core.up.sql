BEGIN;

CREATE TABLE IF NOT EXISTS samples (
    id                    SERIAL PRIMARY KEY,
    creador_id            INT NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    titulo                VARCHAR(200) NOT NULL,
    slug                  VARCHAR(250) UNIQUE NOT NULL,
    id_corto              VARCHAR(10) UNIQUE,
    descripcion           TEXT DEFAULT '',
    bpm                   INT,
    key                   VARCHAR(3),
    escala                VARCHAR(10),
    duracion              REAL NOT NULL DEFAULT 0,
    formato               VARCHAR(10) NOT NULL DEFAULT 'wav',
    tamano                BIGINT NOT NULL DEFAULT 0,
    metadata              JSONB DEFAULT '{}',
    tags                  TEXT[] DEFAULT '{}',
    tags_enriquecidos     TEXT[] DEFAULT ARRAY[]::TEXT[],
    audio_hash            VARCHAR(64),
    hash_parcial          VARCHAR(64),
    embedding             vector(128),
    estado                VARCHAR(20) NOT NULL DEFAULT 'procesando'
                          CHECK (estado IN ('procesando', 'activo', 'inactivo', 'eliminado', 'en_supervision')),
    tipo                  VARCHAR(20) NOT NULL DEFAULT 'loop'
                          CHECK (tipo IN ('loop', 'oneshot', 'fx', 'vocal', 'stem', 'otro')),
    es_premium            BOOLEAN DEFAULT FALSE,
    precio                DECIMAL(10, 2),
    permitir_descarga     BOOLEAN DEFAULT TRUE,
    licencia_libre        BOOLEAN DEFAULT FALSE,
    mostrar_en_comunidad  BOOLEAN DEFAULT TRUE,
    verificado            BOOLEAN DEFAULT FALSE,
    ruta_original         TEXT,
    ruta_optimizada       TEXT,
    ruta_preview          TEXT,
    ruta_waveform         TEXT,
    imagen_url            TEXT,
    cancion_origen_id     INT,  -- FK a canciones, definida en 0004
    relacion_sampleo_id   INT,  -- FK a relaciones_sample, definida en 0004
    total_descargas       INT DEFAULT 0,
    total_likes           INT DEFAULT 0,
    total_reproducciones  INT DEFAULT 0,
    total_comentarios     INT DEFAULT 0,
    publicado_at          TIMESTAMPTZ,
    eliminado_en          TIMESTAMPTZ NULL,
    created_at            TIMESTAMPTZ DEFAULT NOW(),
    updated_at            TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_samples_creador          ON samples (creador_id);
CREATE INDEX IF NOT EXISTS idx_samples_slug             ON samples (slug);
CREATE INDEX IF NOT EXISTS idx_samples_id_corto         ON samples (id_corto);
CREATE INDEX IF NOT EXISTS idx_samples_estado           ON samples (estado, publicado_at DESC);
CREATE INDEX IF NOT EXISTS idx_samples_bpm              ON samples (bpm) WHERE bpm IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_samples_key              ON samples (key) WHERE key IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_samples_metadata         ON samples USING GIN (metadata);
CREATE INDEX IF NOT EXISTS idx_samples_tags             ON samples USING GIN (tags);
CREATE INDEX IF NOT EXISTS idx_samples_tags_enriquecidos ON samples USING GIN (tags_enriquecidos);
CREATE INDEX IF NOT EXISTS idx_samples_genero           ON samples USING GIN ((metadata->'genero'));
CREATE INDEX IF NOT EXISTS idx_samples_carpeta          ON samples ((metadata->>'carpeta_primaria'));
CREATE INDEX IF NOT EXISTS idx_samples_audio_hash       ON samples (audio_hash) WHERE audio_hash IS NOT NULL;
CREATE UNIQUE INDEX IF NOT EXISTS idx_samples_audio_hash_unique
    ON samples (audio_hash)
    WHERE audio_hash IS NOT NULL AND estado IN ('activo', 'en_supervision');
CREATE INDEX IF NOT EXISTS idx_samples_hash_parcial     ON samples (hash_parcial) WHERE hash_parcial IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_samples_embedding
    ON samples USING hnsw (embedding vector_cosine_ops)
    WITH (m = 16, ef_construction = 64);
CREATE INDEX IF NOT EXISTS idx_samples_tipo             ON samples (tipo) WHERE estado = 'activo';
CREATE INDEX IF NOT EXISTS idx_samples_premium
    ON samples (es_premium, publicado_at DESC) WHERE es_premium = TRUE AND estado = 'activo';
CREATE INDEX IF NOT EXISTS idx_samples_verificado       ON samples (verificado) WHERE verificado = TRUE;
CREATE INDEX IF NOT EXISTS idx_samples_papelera         ON samples (eliminado_en) WHERE eliminado_en IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_samples_activo_id        ON samples (id) WHERE estado = 'activo';
CREATE INDEX IF NOT EXISTS idx_samples_engagement_activo
    ON samples ((total_likes + total_reproducciones + total_descargas) DESC)
    WHERE estado = 'activo';

CREATE TABLE IF NOT EXISTS colecciones (
    id              SERIAL PRIMARY KEY,
    usuario_id      INT NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    nombre          VARCHAR(200) NOT NULL,
    slug            VARCHAR(255),
    descripcion     TEXT DEFAULT '',
    imagen_url      TEXT,
    portada_url     TEXT,
    publica         BOOLEAN DEFAULT TRUE,
    parent_id       INT NULL REFERENCES colecciones(id) ON DELETE CASCADE,
    version         INTEGER NOT NULL DEFAULT 1,
    total_samples   INT DEFAULT 0,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_colecciones_usuario ON colecciones (usuario_id);
CREATE INDEX IF NOT EXISTS idx_colecciones_usuario_opt ON colecciones (usuario_id, publica, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_colecciones_publica
    ON colecciones (publica, created_at DESC) WHERE publica = TRUE;
CREATE INDEX IF NOT EXISTS idx_colecciones_parent
    ON colecciones (parent_id) WHERE parent_id IS NOT NULL;
CREATE UNIQUE INDEX IF NOT EXISTS idx_colecciones_nombre_unico_por_padre
    ON colecciones (usuario_id, COALESCE(parent_id, 0), LOWER(nombre));
CREATE UNIQUE INDEX IF NOT EXISTS idx_colecciones_slug
    ON colecciones (slug) WHERE slug IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_colecciones_nombre_trgm
    ON colecciones USING GIN (nombre gin_trgm_ops);

DROP TRIGGER IF EXISTS trg_colecciones_updated ON colecciones;
CREATE TRIGGER trg_colecciones_updated
    BEFORE UPDATE ON colecciones
    FOR EACH ROW EXECUTE FUNCTION actualizar_updated_at();

CREATE TABLE IF NOT EXISTS coleccion_samples (
    coleccion_id INT NOT NULL REFERENCES colecciones(id) ON DELETE CASCADE,
    sample_id    INT NOT NULL REFERENCES samples(id) ON DELETE CASCADE,
    usuario_id   INT NOT NULL REFERENCES usuarios_ext(id),
    posicion     INT DEFAULT 0,
    added_at     TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (coleccion_id, sample_id),
    CONSTRAINT uq_usuario_sample UNIQUE (usuario_id, sample_id)
);

CREATE INDEX IF NOT EXISTS idx_cs_usuario_id ON coleccion_samples (usuario_id);
CREATE INDEX IF NOT EXISTS idx_coleccion_samples_sample_coleccion
    ON coleccion_samples (sample_id, coleccion_id);
CREATE INDEX IF NOT EXISTS idx_coleccion_samples_sample
    ON coleccion_samples (sample_id, coleccion_id);

DROP TRIGGER IF EXISTS trg_samples_updated ON samples;
CREATE TRIGGER trg_samples_updated
    BEFORE UPDATE ON samples
    FOR EACH ROW EXECUTE FUNCTION actualizar_updated_at();

/* Trigger que mantiene tags_enriquecidos sincronizado con tags + metadata.
 * Incluye genero, instrumentos, emocion, artista_vibes y metadata.tags (v058+v070). */
CREATE OR REPLACE FUNCTION fn_recalcular_tags_enriquecidos()
RETURNS TRIGGER AS $$
DECLARE
    resultado TEXT[];
    elem      TEXT;
BEGIN
    resultado := ARRAY[]::TEXT[];

    IF NEW.tags IS NOT NULL THEN
        FOREACH elem IN ARRAY NEW.tags LOOP
            IF elem IS NOT NULL AND elem != '' THEN
                resultado := array_append(resultado, LOWER(elem));
            END IF;
        END LOOP;
    END IF;

    IF NEW.metadata IS NOT NULL AND NEW.metadata->>'genero' IS NOT NULL THEN
        IF jsonb_typeof(NEW.metadata->'genero') = 'array' THEN
            FOR elem IN SELECT jsonb_array_elements_text(NEW.metadata->'genero') LOOP
                IF elem IS NOT NULL AND elem != '' THEN
                    resultado := array_append(resultado, LOWER(elem));
                END IF;
            END LOOP;
        ELSIF NEW.metadata->>'genero' != '' THEN
            resultado := array_append(resultado, LOWER(NEW.metadata->>'genero'));
        END IF;
    END IF;

    IF NEW.metadata IS NOT NULL AND NEW.metadata->>'instrumentos' IS NOT NULL THEN
        IF jsonb_typeof(NEW.metadata->'instrumentos') = 'array' THEN
            FOR elem IN SELECT jsonb_array_elements_text(NEW.metadata->'instrumentos') LOOP
                IF elem IS NOT NULL AND elem != '' THEN
                    resultado := array_append(resultado, LOWER(elem));
                END IF;
            END LOOP;
        ELSIF NEW.metadata->>'instrumentos' != '' THEN
            resultado := array_append(resultado, LOWER(NEW.metadata->>'instrumentos'));
        END IF;
    END IF;

    IF NEW.metadata IS NOT NULL AND NEW.metadata->>'emocion' IS NOT NULL THEN
        IF jsonb_typeof(NEW.metadata->'emocion') = 'array' THEN
            FOR elem IN SELECT jsonb_array_elements_text(NEW.metadata->'emocion') LOOP
                IF elem IS NOT NULL AND elem != '' THEN
                    resultado := array_append(resultado, LOWER(elem));
                END IF;
            END LOOP;
        ELSIF NEW.metadata->>'emocion' != '' THEN
            resultado := array_append(resultado, LOWER(NEW.metadata->>'emocion'));
        END IF;
    END IF;

    IF NEW.metadata IS NOT NULL AND NEW.metadata->'artista_vibes' IS NOT NULL
       AND jsonb_typeof(NEW.metadata->'artista_vibes') = 'array' THEN
        FOR elem IN SELECT jsonb_array_elements_text(NEW.metadata->'artista_vibes') LOOP
            IF elem IS NOT NULL AND elem != '' THEN
                resultado := array_append(resultado, LOWER(elem));
            END IF;
        END LOOP;
    END IF;

    IF NEW.metadata IS NOT NULL AND NEW.metadata->'tags' IS NOT NULL
       AND jsonb_typeof(NEW.metadata->'tags') = 'array' THEN
        FOR elem IN SELECT jsonb_array_elements_text(NEW.metadata->'tags') LOOP
            IF elem IS NOT NULL AND elem != '' THEN
                resultado := array_append(resultado, LOWER(elem));
            END IF;
        END LOOP;
    END IF;

    NEW.tags_enriquecidos := resultado;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_tags_enriquecidos ON samples;
CREATE TRIGGER trg_tags_enriquecidos
    BEFORE INSERT OR UPDATE OF tags, metadata ON samples
    FOR EACH ROW EXECUTE FUNCTION fn_recalcular_tags_enriquecidos();

COMMIT;



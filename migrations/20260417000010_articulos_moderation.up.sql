BEGIN;

CREATE TABLE IF NOT EXISTS articulos (
    id                  SERIAL PRIMARY KEY,
    autor_id            INT NOT NULL REFERENCES usuarios_ext(id),
    titulo              VARCHAR(300) NOT NULL,
    slug                VARCHAR(300) UNIQUE NOT NULL,
    contenido           TEXT NOT NULL DEFAULT '',
    extracto            VARCHAR(500) NOT NULL DEFAULT '',
    portada_url         TEXT,
    categoria           VARCHAR(50) NOT NULL DEFAULT 'inspiracion'
                        CHECK (categoria IN (
                            'inspiracion', 'mastering', 'mezcla', 'promocion-musical', 'teoria-musical',
                            'grabacion', 'sampling', 'diseno-sonoro', 'herramientas',
                            'ableton-live', 'bitwig-studio', 'cubase', 'fl-studio', 'garageband',
                            'logic-pro', 'pro-tools', 'studio-one',
                            'drops-gratis', 'midi-gratis', 'plugins-gratis', 'presets-gratis',
                            'proyectos-gratis', 'sonidos-gratis',
                            'entrevistas', 'destacados', 'noticias'
                        )),
    embeds              JSONB NOT NULL DEFAULT '[]',
    descarga_publica    BOOLEAN NOT NULL DEFAULT FALSE,
    total_likes         INT NOT NULL DEFAULT 0,
    total_comentarios   INT NOT NULL DEFAULT 0,
    moderacion_estado   VARCHAR(20) NOT NULL DEFAULT 'pendiente'
                        CHECK (moderacion_estado IN ('pendiente', 'revision', 'aprobado', 'rechazado')),
    moderacion_razon    VARCHAR(255),
    publicado_en        TIMESTAMPTZ,
    eliminado_en        TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_articulos_autor       ON articulos (autor_id);
CREATE INDEX IF NOT EXISTS idx_articulos_slug        ON articulos (slug);
CREATE INDEX IF NOT EXISTS idx_articulos_categoria   ON articulos (categoria);
CREATE INDEX IF NOT EXISTS idx_articulos_publicados
    ON articulos (publicado_en DESC)
    WHERE moderacion_estado = 'aprobado' AND eliminado_en IS NULL;
CREATE INDEX IF NOT EXISTS idx_articulos_moderacion
    ON articulos (moderacion_estado) WHERE eliminado_en IS NULL;

CREATE TABLE IF NOT EXISTS articulos_likes (
    usuario_id  INT NOT NULL REFERENCES usuarios_ext(id),
    articulo_id INT NOT NULL REFERENCES articulos(id) ON DELETE CASCADE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (usuario_id, articulo_id)
);

CREATE INDEX IF NOT EXISTS idx_articulos_likes_articulo ON articulos_likes (articulo_id);

CREATE TABLE IF NOT EXISTS reportes (
    id              SERIAL PRIMARY KEY,
    tipo            VARCHAR(30) NOT NULL,
    target_id       INT NOT NULL,
    reportador_id   INT NOT NULL REFERENCES usuarios_ext(id),
    reportado_id    INT REFERENCES usuarios_ext(id),
    razon           TEXT NOT NULL,
    detalles        TEXT,
    estado          VARCHAR(20) DEFAULT 'pendiente',
    resuelto_por    INT REFERENCES usuarios_ext(id),
    resuelto_at     TIMESTAMPTZ,
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_reportes_estado      ON reportes (estado);
CREATE INDEX IF NOT EXISTS idx_reportes_tipo_target ON reportes (tipo, target_id);

CREATE TABLE IF NOT EXISTS reportes_duplicados (
    id                    SERIAL PRIMARY KEY,
    sample_original_id    INT NOT NULL REFERENCES samples(id) ON DELETE CASCADE,
    sample_duplicado_id   INT NOT NULL REFERENCES samples(id) ON DELETE CASCADE,
    reportador_id         INT NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    estado                VARCHAR(20) NOT NULL DEFAULT 'reportado'
                          CHECK (estado IN ('reportado', 'en_revision', 'resuelto', 'rechazado')),
    pruebas_texto         TEXT DEFAULT '',
    resuelto_at           TIMESTAMPTZ,
    created_at            TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_reportes_dup_estado    ON reportes_duplicados (estado);
CREATE INDEX IF NOT EXISTS idx_reportes_dup_original  ON reportes_duplicados (sample_original_id);

CREATE TABLE IF NOT EXISTS duplicados_pendientes (
    id                  SERIAL PRIMARY KEY,
    sample_original_id  INT NOT NULL REFERENCES samples(id) ON DELETE CASCADE,
    sample_duplicado_id INT NOT NULL REFERENCES samples(id) ON DELETE CASCADE,
    tipo                VARCHAR(20) NOT NULL CHECK (tipo IN ('cross_usuario', 'mismo_usuario', 'backfill')),
    estado              VARCHAR(20) NOT NULL DEFAULT 'pendiente'
                        CHECK (estado IN ('pendiente', 'aprobado', 'rechazado', 'fusionado')),
    resuelto_por        INT REFERENCES usuarios_ext(id),
    resuelto_at         TIMESTAMPTZ,
    notas               TEXT,
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE (sample_original_id, sample_duplicado_id)
);

CREATE INDEX IF NOT EXISTS idx_duplicados_estado
    ON duplicados_pendientes (estado) WHERE estado = 'pendiente';

CREATE TABLE IF NOT EXISTS contribuciones_pendientes (
    id                          SERIAL PRIMARY KEY,
    contribuidor_id             INT NOT NULL REFERENCES usuarios_ext(id),
    cancion_destino_id          INT REFERENCES canciones(id),
    cancion_fuente_id           INT REFERENCES canciones(id),
    cancion_nueva_titulo        VARCHAR(500),
    cancion_nueva_artista       VARCHAR(300),
    cancion_nueva_youtube_url   VARCHAR(500),
    cancion_nueva_lado          VARCHAR(10) CHECK (cancion_nueva_lado IN ('destino', 'fuente')),
    tipo_relacion               VARCHAR(20) DEFAULT 'sample'
                                CHECK (tipo_relacion IN ('sample', 'cover', 'remix', 'interpolation')),
    tipo_elemento               VARCHAR(50) DEFAULT 'multiple_elements'
                                CHECK (tipo_elemento IN (
                                    'hook_riff', 'vocals_lyrics', 'drums', 'bass',
                                    'keys_synth', 'sound_effect', 'multiple_elements', 'other'
                                )),
    tipo_contribucion           VARCHAR(20) DEFAULT 'nueva'
                                CHECK (tipo_contribucion IN ('nueva', 'edicion', 'eliminacion')),
    relacion_existente_id       INT REFERENCES relaciones_sample(id) ON DELETE SET NULL,
    cambios_propuestos          JSONB,
    estado                      VARCHAR(20) DEFAULT 'pendiente'
                                CHECK (estado IN ('pendiente', 'aprobada', 'rechazada')),
    moderador_id                INT REFERENCES usuarios_ext(id),
    moderador_nota              TEXT,
    relacion_creada_id          INT REFERENCES relaciones_sample(id),
    created_at                  TIMESTAMPTZ DEFAULT NOW(),
    resuelto_at                 TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_contribuciones_estado
    ON contribuciones_pendientes (estado);
CREATE INDEX IF NOT EXISTS idx_contribuciones_contribuidor
    ON contribuciones_pendientes (contribuidor_id);
CREATE INDEX IF NOT EXISTS idx_contribuciones_estado_created
    ON contribuciones_pendientes (estado, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_contribuciones_relacion_existente
    ON contribuciones_pendientes (relacion_existente_id)
    WHERE relacion_existente_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_contribuciones_tipo_estado
    ON contribuciones_pendientes (tipo_contribucion, estado);

COMMIT;



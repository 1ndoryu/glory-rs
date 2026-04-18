BEGIN;

CREATE TABLE IF NOT EXISTS usuarios_ext (
    id                       SERIAL PRIMARY KEY,
    wp_user_id               BIGINT UNIQUE NOT NULL,
    username                 VARCHAR(50) UNIQUE NOT NULL,
    email                    VARCHAR(255) UNIQUE,
    nombre_visible           VARCHAR(100) NOT NULL DEFAULT '',
    bio                      TEXT DEFAULT '',
    avatar_url               TEXT,
    portada_url              TEXT,
    sitio_web                VARCHAR(500) DEFAULT NULL,
    generos_favoritos        JSONB DEFAULT '[]'::jsonb,
    registro_ip              VARCHAR(45) DEFAULT NULL,
    paypal_email             VARCHAR(255) DEFAULT NULL,
    plan                     VARCHAR(20) NOT NULL DEFAULT 'free'
                             CHECK (plan IN ('free', 'pro', 'premium')),
    rol                      VARCHAR(20) NOT NULL DEFAULT 'usuario'
                             CHECK (rol IN ('usuario', 'creador', 'admin')),
    verificado               BOOLEAN DEFAULT FALSE,
    es_seed                  BOOLEAN DEFAULT FALSE,
    total_seguidores         INT DEFAULT 0,
    total_seguidos           INT DEFAULT 0,
    total_samples            INT DEFAULT 0,
    total_descargas          INT DEFAULT 0,
    creditos_bonus           INTEGER NOT NULL DEFAULT 0,
    stripe_customer_id       VARCHAR(100),
    stripe_connect_id        VARCHAR(100),
    violaciones_moderacion   INT DEFAULT 0,
    baneado_hasta            TIMESTAMPTZ,
    ban_razon                TEXT,
    estado                   VARCHAR(20) NOT NULL DEFAULT 'activo'
                             CHECK (estado IN ('activo', 'suspendido', 'en_eliminacion')),
    suspendido_hasta         TIMESTAMPTZ,
    suspension_razon         TEXT,
    marcado_eliminacion_en   TIMESTAMPTZ,
    sera_eliminado_en        TIMESTAMPTZ,
    created_at               TIMESTAMPTZ DEFAULT NOW(),
    updated_at               TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_usuarios_ext_wp           ON usuarios_ext (wp_user_id);
CREATE INDEX IF NOT EXISTS idx_usuarios_ext_username     ON usuarios_ext (username);
CREATE INDEX IF NOT EXISTS idx_usuarios_ext_es_seed      ON usuarios_ext (es_seed) WHERE es_seed = TRUE;
CREATE INDEX IF NOT EXISTS idx_usuarios_ext_estado       ON usuarios_ext (estado) WHERE estado != 'activo';
CREATE INDEX IF NOT EXISTS idx_usuarios_ext_sera_eliminado
    ON usuarios_ext (sera_eliminado_en) WHERE sera_eliminado_en IS NOT NULL;

CREATE TABLE IF NOT EXISTS bloqueos (
    id              SERIAL PRIMARY KEY,
    bloqueador_id   INT NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    bloqueado_id    INT NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    razon           VARCHAR(255) NOT NULL DEFAULT '',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT bloqueos_par_unico   UNIQUE (bloqueador_id, bloqueado_id),
    CONSTRAINT bloqueos_no_autoblock CHECK (bloqueador_id <> bloqueado_id)
);

CREATE INDEX IF NOT EXISTS idx_bloqueos_bloqueador ON bloqueos (bloqueador_id);
CREATE INDEX IF NOT EXISTS idx_bloqueos_bloqueado  ON bloqueos (bloqueado_id);

CREATE OR REPLACE FUNCTION actualizar_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_usuarios_ext_updated ON usuarios_ext;
CREATE TRIGGER trg_usuarios_ext_updated
    BEFORE UPDATE ON usuarios_ext
    FOR EACH ROW EXECUTE FUNCTION actualizar_updated_at();

COMMIT;



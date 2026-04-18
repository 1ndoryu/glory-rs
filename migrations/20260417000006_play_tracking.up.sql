BEGIN;

CREATE TABLE IF NOT EXISTS reproducciones (
    id                  SERIAL PRIMARY KEY,
    usuario_id          INT REFERENCES usuarios_ext(id) ON DELETE SET NULL,
    sample_id           INT NOT NULL REFERENCES samples(id) ON DELETE CASCADE,
    duracion_escuchada  REAL DEFAULT 0,
    completada          BOOLEAN DEFAULT FALSE,
    created_at          TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_reproducciones_usuario
    ON reproducciones (usuario_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_reproducciones_sample
    ON reproducciones (sample_id);
CREATE INDEX IF NOT EXISTS idx_reproducciones_created
    ON reproducciones (created_at DESC);
CREATE INDEX IF NOT EXISTS idx_reproducciones_sample_created
    ON reproducciones (sample_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_reproducciones_usuario_sample
    ON reproducciones (usuario_id, sample_id);
CREATE INDEX IF NOT EXISTS idx_reproducciones_usuario_sample_opt
    ON reproducciones (usuario_id, sample_id) WHERE usuario_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS descargas (
    id            SERIAL PRIMARY KEY,
    usuario_id    INT NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    sample_id     INT NOT NULL REFERENCES samples(id) ON DELETE CASCADE,
    calidad       VARCHAR(10) NOT NULL DEFAULT 'mp3',
    tamano_bytes  BIGINT DEFAULT 0,
    created_at    TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_descargas_usuario_dia      ON descargas (usuario_id, created_at);
CREATE INDEX IF NOT EXISTS idx_descargas_usuario_created  ON descargas (usuario_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_descargas_sample_created   ON descargas (sample_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_descargas_usuario_sample   ON descargas (usuario_id, sample_id);

CREATE TABLE IF NOT EXISTS codigos_descarga_gratis (
    id              BIGSERIAL PRIMARY KEY,
    codigo          VARCHAR(64) UNIQUE NOT NULL,
    tipo            VARCHAR(32) NOT NULL CHECK (tipo IN ('sample', 'coleccion')),
    target_id       BIGINT NOT NULL,
    nombre_item     VARCHAR(255) NOT NULL DEFAULT '',
    creado_por_id   BIGINT NOT NULL,
    activo          BOOLEAN NOT NULL DEFAULT TRUE,
    expires_at      TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '1 year'),
    creado_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS codigos_gratis_usos (
    id            BIGSERIAL PRIMARY KEY,
    codigo_id     BIGINT NOT NULL REFERENCES codigos_descarga_gratis(id) ON DELETE CASCADE,
    usuario_id    BIGINT NOT NULL,
    expirado      BOOLEAN NOT NULL DEFAULT FALSE,
    reclamado_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (codigo_id, usuario_id)
);

CREATE INDEX IF NOT EXISTS idx_codigos_gratis_tipo_target
    ON codigos_descarga_gratis (tipo, target_id, activo);
CREATE INDEX IF NOT EXISTS idx_codigos_gratis_expires_activo
    ON codigos_descarga_gratis (expires_at) WHERE activo = TRUE;
CREATE INDEX IF NOT EXISTS idx_codigos_gratis_usos_usuario
    ON codigos_gratis_usos (usuario_id, codigo_id);

COMMIT;



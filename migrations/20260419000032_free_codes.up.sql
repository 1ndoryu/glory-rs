BEGIN;

CREATE TABLE IF NOT EXISTS codigos_descarga_gratis (
    id             BIGSERIAL PRIMARY KEY,
    codigo         VARCHAR(64) NOT NULL UNIQUE,
    tipo           VARCHAR(20) NOT NULL
                   CHECK (tipo IN ('sample', 'coleccion')),
    target_id      BIGINT NOT NULL,
    creado_por_id  BIGINT NOT NULL,
    activo         BOOLEAN NOT NULL DEFAULT TRUE,
    nombre_item    VARCHAR(255) NOT NULL DEFAULT '',
    expires_at     TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '1 year'),
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_codigos_gratis_tipo_target_activo
    ON codigos_descarga_gratis (tipo, target_id)
    WHERE activo = TRUE;

CREATE INDEX IF NOT EXISTS idx_codigos_gratis_expires_activo
    ON codigos_descarga_gratis (expires_at)
    WHERE activo = TRUE;

CREATE TABLE IF NOT EXISTS codigos_gratis_usos (
    id          BIGSERIAL PRIMARY KEY,
    codigo_id   BIGINT NOT NULL REFERENCES codigos_descarga_gratis(id) ON DELETE CASCADE,
    usuario_id  BIGINT NOT NULL,
    expirado    BOOLEAN NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT codigos_gratis_usos_unico UNIQUE (codigo_id, usuario_id)
);

CREATE INDEX IF NOT EXISTS idx_codigos_gratis_usos_usuario
    ON codigos_gratis_usos (usuario_id, codigo_id);

COMMIT;
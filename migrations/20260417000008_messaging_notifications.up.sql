BEGIN;

CREATE TABLE IF NOT EXISTS conversaciones (
    id                 SERIAL PRIMARY KEY,
    participante_1     INT NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    participante_2     INT NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    aceptada           BOOLEAN DEFAULT FALSE,
    ultimo_mensaje_at  TIMESTAMPTZ DEFAULT NOW(),
    created_at         TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE (participante_1, participante_2)
);

CREATE INDEX IF NOT EXISTS idx_conversaciones_participantes
    ON conversaciones (participante_1, participante_2);
CREATE INDEX IF NOT EXISTS idx_conversaciones_ultimo_msg
    ON conversaciones (ultimo_mensaje_at DESC);

CREATE TABLE IF NOT EXISTS mensajes (
    id              SERIAL PRIMARY KEY,
    conversacion_id INT NOT NULL REFERENCES conversaciones(id) ON DELETE CASCADE,
    autor_id        INT NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    contenido       TEXT NOT NULL,
    tipo            VARCHAR(20) DEFAULT 'texto',
    media_url       TEXT,
    media_metadata  JSONB,
    leido           BOOLEAN DEFAULT FALSE,
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_mensajes_conversacion
    ON mensajes (conversacion_id, created_at);
CREATE INDEX IF NOT EXISTS idx_mensajes_tipo            ON mensajes (tipo);
CREATE INDEX IF NOT EXISTS idx_mensajes_no_leidos
    ON mensajes (autor_id, leido) WHERE leido = FALSE;
CREATE INDEX IF NOT EXISTS idx_mensajes_conv_no_leidos_opt
    ON mensajes (conversacion_id) WHERE leido = FALSE;

CREATE TABLE IF NOT EXISTS notificaciones (
    id          SERIAL PRIMARY KEY,
    usuario_id  INT NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    tipo        VARCHAR(30) NOT NULL,
    titulo      VARCHAR(200) DEFAULT '',
    mensaje     TEXT DEFAULT '',
    datos       JSONB DEFAULT '{}',
    leida       BOOLEAN DEFAULT FALSE,
    enlace      TEXT,
    actor_id    INT REFERENCES usuarios_ext(id) ON DELETE SET NULL,
    created_at  TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_notificaciones_usuario
    ON notificaciones (usuario_id, leida, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_notificaciones_no_leidas
    ON notificaciones (usuario_id) WHERE leida = FALSE;
CREATE INDEX IF NOT EXISTS idx_notificaciones_usuario_tipo_opt
    ON notificaciones (usuario_id, tipo) WHERE leida = FALSE;
CREATE INDEX IF NOT EXISTS idx_notificaciones_datos
    ON notificaciones USING GIN (datos);

CREATE TABLE IF NOT EXISTS push_subscriptions (
    id          SERIAL PRIMARY KEY,
    usuario_id  INTEGER NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    endpoint    TEXT NOT NULL,
    p256dh      TEXT NOT NULL,
    auth        TEXT NOT NULL,
    plataforma  VARCHAR(20) NOT NULL DEFAULT 'web',
    activa      BOOLEAN NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_push_subscriptions_usuario
    ON push_subscriptions (usuario_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_push_subscriptions_endpoint
    ON push_subscriptions (endpoint);

CREATE TABLE IF NOT EXISTS fcm_tokens (
    id          SERIAL PRIMARY KEY,
    usuario_id  INTEGER NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    token       TEXT NOT NULL UNIQUE,
    plataforma  VARCHAR(20) NOT NULL DEFAULT 'android',
    activo      BOOLEAN NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_fcm_tokens_usuario ON fcm_tokens (usuario_id);
CREATE INDEX IF NOT EXISTS idx_fcm_tokens_activo
    ON fcm_tokens (usuario_id, activo) WHERE activo = TRUE;

COMMIT;



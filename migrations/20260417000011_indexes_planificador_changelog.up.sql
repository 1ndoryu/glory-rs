BEGIN;

CREATE TABLE IF NOT EXISTS algoritmo_estado (
    usuario_id                  INTEGER PRIMARY KEY REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    cnt_likes                   INTEGER NOT NULL DEFAULT 0,
    cnt_reproducciones          INTEGER NOT NULL DEFAULT 0,
    cnt_completas               INTEGER NOT NULL DEFAULT 0,
    cnt_descargas               INTEGER NOT NULL DEFAULT 0,
    cnt_follows                 INTEGER NOT NULL DEFAULT 0,
    cnt_comentarios             INTEGER NOT NULL DEFAULT 0,
    cnt_likes_preciso           INTEGER NOT NULL DEFAULT 0,
    cnt_reproducciones_preciso  INTEGER NOT NULL DEFAULT 0,
    cnt_completas_preciso       INTEGER NOT NULL DEFAULT 0,
    cnt_descargas_preciso       INTEGER NOT NULL DEFAULT 0,
    cnt_follows_preciso         INTEGER NOT NULL DEFAULT 0,
    cnt_comentarios_preciso     INTEGER NOT NULL DEFAULT 0,
    ultimo_rapido               TIMESTAMPTZ DEFAULT NOW(),
    ultimo_preciso              TIMESTAMPTZ DEFAULT NOW(),
    ultima_actividad            TIMESTAMPTZ DEFAULT NOW(),
    version_perfil              INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_algoritmo_ultimo_rapido
    ON algoritmo_estado (ultimo_rapido);
CREATE INDEX IF NOT EXISTS idx_algoritmo_ultimo_preciso
    ON algoritmo_estado (ultimo_preciso);

CREATE TABLE IF NOT EXISTS sync_changelog (
    id          BIGSERIAL PRIMARY KEY,
    usuario_id  INTEGER NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    tipo        TEXT NOT NULL
                CHECK (tipo IN (
                    'sample_added', 'sample_removed', 'sample_updated',
                    'collection_created', 'collection_renamed', 'collection_deleted',
                    'collection_merged'
                )),
    entidad_id  INTEGER NOT NULL,
    metadata    JSONB NOT NULL DEFAULT '{}',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_sync_changelog_usuario_id
    ON sync_changelog (usuario_id, id);
CREATE INDEX IF NOT EXISTS idx_sync_changelog_created
    ON sync_changelog (created_at);

CREATE TABLE IF NOT EXISTS user_tag_scores (
    user_id        INT NOT NULL,
    tag            TEXT NOT NULL,
    w_likes        REAL DEFAULT 0,
    w_repro        REAL DEFAULT 0,
    w_tiempo       REAL DEFAULT 0,
    w_descargas    REAL DEFAULT 0,
    w_completadas  REAL DEFAULT 0,
    w_dislikes     REAL DEFAULT 0,
    w_ctx          REAL DEFAULT 0,
    updated_at     TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (user_id, tag)
);

CREATE INDEX IF NOT EXISTS idx_user_tag_scores_user
    ON user_tag_scores (user_id);

/* Índices críticos adicionales (full-text + trgm sobre samples/usuarios) */
CREATE INDEX IF NOT EXISTS idx_samples_titulo_trgm
    ON samples USING GIN (titulo gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_samples_busqueda_fts
    ON samples USING GIN (
        to_tsvector('spanish', COALESCE(titulo, '') || ' ' || COALESCE(descripcion, ''))
    );
CREATE INDEX IF NOT EXISTS idx_samples_titulo_fts
    ON samples USING GIN (to_tsvector('spanish', COALESCE(titulo, '')));
CREATE INDEX IF NOT EXISTS idx_usuarios_username_trgm
    ON usuarios_ext USING GIN (username gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_usuarios_nombre_visible_trgm
    ON usuarios_ext USING GIN (nombre_visible gin_trgm_ops);

COMMIT;

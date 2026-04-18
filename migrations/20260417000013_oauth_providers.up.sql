-- [174A-21] Vincular cuentas externas (Google, etc.) a usuarios_ext.
-- Una fila por (proveedor, sub_externo). Un mismo usuario puede tener varios providers.

CREATE TABLE IF NOT EXISTS usuarios_ext_oauth (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL REFERENCES usuarios_ext(id) ON DELETE CASCADE,
    provider     TEXT NOT NULL,
    provider_sub TEXT NOT NULL,
    email        TEXT,
    created_at   TIMESTAMPTZ DEFAULT now(),
    UNIQUE (provider, provider_sub)
);

CREATE INDEX IF NOT EXISTS idx_oauth_user ON usuarios_ext_oauth(user_id);
CREATE INDEX IF NOT EXISTS idx_oauth_provider_email ON usuarios_ext_oauth(provider, email);

/* [283A-2] Tabla de API keys para integraciones externas (chatbots, etc.)
 * Las keys se almacenan hasheadas con SHA-256 para seguridad.
 * Cada key pertenece a un user_id (propietario del restaurante). */

CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    nombre VARCHAR(100) NOT NULL,
    key_hash VARCHAR(64) NOT NULL UNIQUE,
    key_prefix VARCHAR(8) NOT NULL,
    permisos JSONB NOT NULL DEFAULT '["chatbot"]'::jsonb,
    activa BOOLEAN NOT NULL DEFAULT TRUE,
    last_used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_api_keys_key_hash ON api_keys(key_hash);
CREATE INDEX idx_api_keys_user_id ON api_keys(user_id);

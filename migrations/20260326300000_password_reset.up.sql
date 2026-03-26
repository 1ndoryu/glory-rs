-- [263A-15] Agregar columnas para recuperación de contraseña
ALTER TABLE users ADD COLUMN reset_token VARCHAR(64);
ALTER TABLE users ADD COLUMN reset_token_expires_at TIMESTAMPTZ;
CREATE INDEX idx_users_reset_token ON users(reset_token) WHERE reset_token IS NOT NULL;

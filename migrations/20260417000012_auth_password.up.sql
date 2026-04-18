-- [174A-18] Auth nativo Rust: agrega password_hash y hace wp_user_id opcional.
-- En greenfield Rust no dependemos de wp_users; wp_user_id queda como referencia
-- legacy para usuarios migrados desde WordPress.
BEGIN;

ALTER TABLE usuarios_ext
    ALTER COLUMN wp_user_id DROP NOT NULL;

ALTER TABLE usuarios_ext
    ADD COLUMN IF NOT EXISTS password_hash VARCHAR(255);

-- Email pasa a ser obligatorio para nuevos registros nativos.
-- (No forzamos NOT NULL aún para no romper filas legacy importadas.)
CREATE UNIQUE INDEX IF NOT EXISTS idx_usuarios_ext_email_lower
    ON usuarios_ext (LOWER(email)) WHERE email IS NOT NULL;

COMMIT;

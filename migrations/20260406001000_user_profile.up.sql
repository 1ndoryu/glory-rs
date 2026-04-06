/* [044A-43] Añadir campos de perfil a users: avatar, display_name
 * [064A-fix] Migrado a timestamp 20260406001000 para evitar conflicto de versión con add_medium_plans.
 * IF NOT EXISTS porque las columnas pueden existir en DBs donde user_profile se aplicó antes del rename. */
ALTER TABLE users ADD COLUMN IF NOT EXISTS avatar_url TEXT;
ALTER TABLE users ADD COLUMN IF NOT EXISTS display_name VARCHAR(100);

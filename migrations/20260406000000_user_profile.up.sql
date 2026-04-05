/* [044A-43] Añadir campos de perfil a users: avatar, display_name */
ALTER TABLE users ADD COLUMN avatar_url TEXT;
ALTER TABLE users ADD COLUMN display_name VARCHAR(100);

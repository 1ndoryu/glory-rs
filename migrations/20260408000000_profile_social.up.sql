/* [074A-23] Añadir campos sociales a user_profiles para completar el formulario de perfil.
 * linkedin, twitter, website — el frontend ya muestra estos campos pero no se persistían. */
ALTER TABLE user_profiles ADD COLUMN IF NOT EXISTS linkedin VARCHAR(255);
ALTER TABLE user_profiles ADD COLUMN IF NOT EXISTS twitter VARCHAR(255);
ALTER TABLE user_profiles ADD COLUMN IF NOT EXISTS website VARCHAR(500);

/* [034A-3] URL de Haddock: enlace configurable a plataforma externa de gestión
 * financiera detallada. El propietario puede configurar la URL de su cuenta
 * de Haddock para acceder desde la sidebar. */

ALTER TABLE configuracion_restaurante
    ADD COLUMN IF NOT EXISTS url_haddock TEXT NOT NULL DEFAULT '';

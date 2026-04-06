/* [064A-5] Campos para integración con Haddock POS API.
 * haddock_api_token: token Base64 de autenticación (Basic Auth).
 * haddock_sync_enabled: toggle para activar/desactivar la sincronización sin borrar el token. */

ALTER TABLE configuracion_restaurante
    ADD COLUMN IF NOT EXISTS haddock_api_token TEXT NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS haddock_sync_enabled BOOLEAN NOT NULL DEFAULT false;

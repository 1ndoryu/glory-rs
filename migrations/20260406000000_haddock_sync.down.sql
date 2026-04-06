ALTER TABLE configuracion_restaurante
    DROP COLUMN IF EXISTS haddock_api_token,
    DROP COLUMN IF EXISTS haddock_sync_enabled;

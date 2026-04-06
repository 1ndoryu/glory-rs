ALTER TABLE ventas
    DROP COLUMN IF EXISTS haddock_synced,
    DROP COLUMN IF EXISTS haddock_synced_at,
    DROP COLUMN IF EXISTS haddock_sync_error;

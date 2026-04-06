/* [064A-6] Tracking de sincronización Haddock en ventas.
 * Permite saber qué ventas fueron sincronizadas exitosamente,
 * cuándo fue la última sincronización y si hubo errores. */
ALTER TABLE ventas
    ADD COLUMN IF NOT EXISTS haddock_synced BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN IF NOT EXISTS haddock_synced_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS haddock_sync_error TEXT;

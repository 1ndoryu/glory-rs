BEGIN;

ALTER TABLE reportes
    ALTER COLUMN reportador_id DROP NOT NULL;

ALTER TABLE reportes
    ADD COLUMN IF NOT EXISTS ip_origen VARCHAR(64);

CREATE INDEX IF NOT EXISTS idx_reportes_ip_origen_created
    ON reportes (ip_origen, created_at DESC)
    WHERE ip_origen IS NOT NULL;

COMMIT;
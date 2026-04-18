CREATE TABLE IF NOT EXISTS ia_queue (
    id                  SERIAL PRIMARY KEY,
    sample_id           INT NOT NULL REFERENCES samples(id) ON DELETE CASCADE,
    status              TEXT NOT NULL DEFAULT 'pending'
                        CHECK (status IN ('pending', 'processing', 'retry_scheduled', 'completed', 'failed')),
    attempts            INT NOT NULL DEFAULT 0,
    max_attempts        INT NOT NULL DEFAULT 12,
    next_retry_at       TIMESTAMPTZ,
    last_error          TEXT,
    metadata            JSONB NOT NULL DEFAULT '{}'::jsonb,
    processed_at        TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_ia_queue_status_retry
    ON ia_queue (status, next_retry_at, created_at)
    WHERE status IN ('pending', 'retry_scheduled');

CREATE UNIQUE INDEX IF NOT EXISTS idx_ia_queue_unique_active_sample
    ON ia_queue (sample_id)
    WHERE status IN ('pending', 'processing', 'retry_scheduled');
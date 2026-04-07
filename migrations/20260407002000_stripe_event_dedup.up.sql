/* [064A-73] Deduplicación de webhooks Stripe: evita procesar el mismo evento 2 veces.
 * Stripe reenvía webhooks si no obtiene 200 en ~5 minutos — sin dedup, un pago
 * podría acreditarse doble. */
CREATE TABLE IF NOT EXISTS stripe_processed_events (
    event_id TEXT PRIMARY KEY,
    event_type TEXT NOT NULL,
    processed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

/* Limpieza automática: eventos >30 días ya no son relevantes */
CREATE INDEX idx_stripe_events_processed_at ON stripe_processed_events (processed_at);

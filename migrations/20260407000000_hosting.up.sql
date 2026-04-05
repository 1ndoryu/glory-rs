-- [054A-2] Hosting service: subscriptions + events.
-- Registra suscripciones de hosting de clientes y eventos del ciclo de vida.

CREATE TABLE hosting_subscriptions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id),
    client_name VARCHAR(200) NOT NULL,
    client_email VARCHAR(254) NOT NULL,
    plan VARCHAR(20) NOT NULL,
    domain VARCHAR(253),
    coolify_site_name VARCHAR(100),
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    stripe_subscription_id VARCHAR(100),
    monthly_price_cents INT NOT NULL,
    storage_limit_mb INT NOT NULL DEFAULT 5120,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE hosting_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subscription_id UUID NOT NULL REFERENCES hosting_subscriptions(id) ON DELETE CASCADE,
    event_type VARCHAR(50) NOT NULL,
    details JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_hosting_subscriptions_status ON hosting_subscriptions(status);
CREATE INDEX idx_hosting_subscriptions_user ON hosting_subscriptions(user_id) WHERE user_id IS NOT NULL;
CREATE INDEX idx_hosting_events_subscription ON hosting_events(subscription_id, created_at DESC);

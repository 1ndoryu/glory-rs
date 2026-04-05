-- [044A-38 Fase 5] Chat system: sessions, messages, typing.
-- Integrado con orders (order_id nullable = pre-venta).
-- sender_type: client/ai/employee/admin (marketplace roles).

CREATE TABLE chat_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    /* Visitante anónimo (pre-venta) O usuario autenticado */
    visitor_id VARCHAR(64),
    visitor_name VARCHAR(100),
    user_id UUID REFERENCES users(id),
    
    /* Vínculo con orden (NULL = pre-venta) */
    order_id UUID REFERENCES orders(id),
    
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    assigned_staff_id UUID REFERENCES users(id),
    ai_enabled BOOLEAN NOT NULL DEFAULT true,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE chat_messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id UUID NOT NULL REFERENCES chat_sessions(id) ON DELETE CASCADE,
    sender_type VARCHAR(10) NOT NULL,
    sender_id VARCHAR(64),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE chat_typing (
    session_id UUID PRIMARY KEY REFERENCES chat_sessions(id) ON DELETE CASCADE,
    content TEXT NOT NULL DEFAULT '',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_chat_messages_session ON chat_messages(session_id, created_at);
CREATE INDEX idx_chat_sessions_status ON chat_sessions(status);
CREATE INDEX idx_chat_sessions_order ON chat_sessions(order_id) WHERE order_id IS NOT NULL;
CREATE INDEX idx_chat_sessions_user ON chat_sessions(user_id) WHERE user_id IS NOT NULL;

/* Vincular orders con chat: columna opcional en orders */
ALTER TABLE orders ADD COLUMN IF NOT EXISTS chat_session_id UUID REFERENCES chat_sessions(id);

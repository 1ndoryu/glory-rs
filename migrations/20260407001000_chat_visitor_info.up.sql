/* [064A-72] Metadata de visitante en sesiones + notas de sesión.
 * visitor_ip y visitor_user_agent capturados en la conexión WS.
 * chat_session_notes: notas libres del staff sobre la sesión/visitante. */

ALTER TABLE chat_sessions ADD COLUMN IF NOT EXISTS visitor_ip TEXT;
ALTER TABLE chat_sessions ADD COLUMN IF NOT EXISTS visitor_user_agent TEXT;

CREATE TABLE IF NOT EXISTS chat_session_notes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id UUID NOT NULL REFERENCES chat_sessions(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_chat_session_notes_session ON chat_session_notes(session_id);

-- Revert chat system tables
ALTER TABLE orders DROP COLUMN IF EXISTS chat_session_id;
DROP TABLE IF EXISTS chat_typing;
DROP TABLE IF EXISTS chat_messages;
DROP TABLE IF EXISTS chat_sessions;

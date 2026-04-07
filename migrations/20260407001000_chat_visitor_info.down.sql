DROP TABLE IF EXISTS chat_session_notes;
ALTER TABLE chat_sessions DROP COLUMN IF EXISTS visitor_user_agent;
ALTER TABLE chat_sessions DROP COLUMN IF EXISTS visitor_ip;

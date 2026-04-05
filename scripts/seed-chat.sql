-- [054A-10] Seed de chat de prueba.
-- Crea una sesión de visitante con mensajes simulados para testear el panel de chat.
-- Ejecutar con: psql -U postgres -d nakomi_db -f scripts/seed-chat.sql

DO $$
DECLARE
    v_session_id UUID;
BEGIN
    -- Crear sesión de visitante anónimo
    INSERT INTO chat_sessions (visitor_id, visitor_name, status, ai_enabled)
    VALUES ('test-visitor-001', 'María García', 'active', true)
    RETURNING id INTO v_session_id;

    -- Mensaje 1: visitante
    INSERT INTO chat_messages (session_id, sender_type, sender_id, content, created_at)
    VALUES (v_session_id, 'visitor', 'test-visitor-001', '¡Hola! Me interesa el servicio de diseño web. ¿Podrían darme más información?', NOW() - INTERVAL '10 minutes');

    -- Mensaje 2: IA responde
    INSERT INTO chat_messages (session_id, sender_type, sender_id, content, created_at)
    VALUES (v_session_id, 'ai', 'ai', '¡Hola María! Gracias por tu interés en nuestros servicios de diseño web. Tenemos dos planes disponibles: Básico y Avanzado. El plan Básico incluye diseño responsive, hasta 5 páginas, y optimización SEO básica. El plan Avanzado incluye todo lo anterior más animaciones personalizadas, integración con APIs y soporte prioritario. ¿Cuál te interesa más?', NOW() - INTERVAL '9 minutes');

    -- Mensaje 3: visitante
    INSERT INTO chat_messages (session_id, sender_type, sender_id, content, created_at)
    VALUES (v_session_id, 'visitor', 'test-visitor-001', 'El plan avanzado suena bien. ¿Cuánto cuesta y cuánto tarda approximately?', NOW() - INTERVAL '7 minutes');

    -- Mensaje 4: IA responde
    INSERT INTO chat_messages (session_id, sender_type, sender_id, content, created_at)
    VALUES (v_session_id, 'ai', 'ai', 'El plan Avanzado tiene un costo de $2,500 USD y el tiempo estimado de entrega es de 4-6 semanas dependiendo de la complejidad del proyecto. ¿Te gustaría agendar una llamada con nuestro equipo para discutir los detalles de tu proyecto?', NOW() - INTERVAL '6 minutes');

    -- Mensaje 5: visitante
    INSERT INTO chat_messages (session_id, sender_type, sender_id, content, created_at)
    VALUES (v_session_id, 'visitor', 'test-visitor-001', 'Sí, me gustaría hablar con alguien del equipo directamente. ¿Hay algún empleado disponible?', NOW() - INTERVAL '4 minutes');

    RAISE NOTICE 'Sesión de chat creada: %', v_session_id;
END $$;

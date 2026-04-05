/* [054A-10] Hook de la sección de chat del panel.
 * Centraliza estado local, envío, cierre y conexión WS por sesión activa. */

import {useCallback, useEffect, useRef, useState, type KeyboardEvent} from 'react';

import {apiCloseSession} from '../api/chat';
import {useChat, useChatWs} from './useChat';
import {toast} from '../stores/toastStore';

export function useSeccionChat() {
    const [activeSessionId, setActiveSessionId] = useState<string | null>(null);
    const [input, setInput] = useState('');
    const [closing, setClosing] = useState(false);
    const messagesEndRef = useRef<HTMLDivElement>(null);

    const {sessions, messages, cargandoSesiones, cargandoMensajes, enviarMensaje, enviando} =
        useChat(activeSessionId ?? undefined);
    const ws = useChatWs();

    useEffect(() => {
        ws.connect();
        return () => ws.disconnect();
    }, [ws.connect, ws.disconnect]);

    useEffect(() => {
        if (activeSessionId) {
            ws.joinSession(activeSessionId);
        }
    }, [activeSessionId, ws.joinSession]);

    useEffect(() => {
        messagesEndRef.current?.scrollIntoView({behavior: 'smooth'});
    }, [messages]);

    const handleSend = useCallback(async () => {
        if (!input.trim() || !activeSessionId || enviando) return;

        const content = input.trim();
        setInput('');

        try {
            await enviarMensaje({sId: activeSessionId, content});
        } catch {
            setInput(content);
        }
    }, [activeSessionId, enviando, enviarMensaje, input]);

    const handleKeyDown = useCallback(
        (event: KeyboardEvent<HTMLTextAreaElement>) => {
            if (event.key === 'Enter' && !event.shiftKey) {
                event.preventDefault();
                void handleSend();
            }
        },
        [handleSend],
    );

    const handleCloseSession = useCallback(async () => {
        if (!activeSessionId || closing) return;

        setClosing(true);
        try {
            await apiCloseSession(activeSessionId);
            toast.success('Conversación cerrada');
            setActiveSessionId(null);
        } catch {
            toast.error('Error al cerrar conversación');
        } finally {
            setClosing(false);
        }
    }, [activeSessionId, closing]);

    const selectSession = useCallback((sessionId: string) => {
        setActiveSessionId(sessionId);
    }, []);

    const clearActiveSession = useCallback(() => {
        setActiveSessionId(null);
    }, []);

    return {
        activeSessionId,
        sessions,
        messages,
        cargandoSesiones,
        cargandoMensajes,
        enviando,
        closing,
        input,
        messagesEndRef,
        typingMap: ws.typingMap,
        showingChat: activeSessionId !== null,
        setInput,
        selectSession,
        clearActiveSession,
        handleKeyDown,
        handleSend,
        handleCloseSession,
    };
}
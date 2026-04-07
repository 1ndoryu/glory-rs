/* [064A-29] Hook para el chat widget de visitantes.
 * Gestiona conexión WebSocket, persistencia de visitor_id/session_id en localStorage,
 * y carga de historial de mensajes al reconectar (via WS — el backend envia history).
 * Independiente del hook useChat (que es para staff). */

import {useState, useCallback, useRef, useEffect} from 'react';
import {buildVisitorWsUrl, type WsServerMessage, type ChatMessage} from '../api/chat';

const STORAGE_KEY_VISITOR_ID = 'nakomi_visitor_id';
const STORAGE_KEY_SESSION_ID = 'nakomi_chat_session_id';
const TYPING_THROTTLE_MS = 200;

function getOrCreateVisitorId(): string {
    const saved = localStorage.getItem(STORAGE_KEY_VISITOR_ID);
    if (saved) return saved;
    const id = crypto.randomUUID();
    localStorage.setItem(STORAGE_KEY_VISITOR_ID, id);
    return id;
}

function getSavedSessionId(): string | null {
    return localStorage.getItem(STORAGE_KEY_SESSION_ID);
}

function saveSessionId(id: string): void {
    localStorage.setItem(STORAGE_KEY_SESSION_ID, id);
}

export interface TypingIndicator {
    sender: string;
    content: string;
}

export function useChatWidget() {
    const [connected, setConnected] = useState(false);
    const [connecting, setConnecting] = useState(false);
    const [messages, setMessages] = useState<ChatMessage[]>([]);
    const [typing, setTyping] = useState<TypingIndicator | null>(null);
    const [sessionId, setSessionId] = useState<string | null>(getSavedSessionId);
    const wsRef = useRef<WebSocket | null>(null);
    const typingTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
    const lastTypingSentRef = useRef<number>(0);

    const connect = useCallback((visitorName?: string) => {
        if (wsRef.current) return;
        setConnecting(true);

        const visitorId = getOrCreateVisitorId();
        const url = buildVisitorWsUrl(visitorId, visitorName);
        const ws = new WebSocket(url);
        wsRef.current = ws;

        ws.onopen = () => {
            setConnected(true);
            setConnecting(false);
            /* [064A-29] El backend envia historial de mensajes automaticamente
             * al reconectar (via WsServerMessage::Message). El hook los recibe
             * en onmessage y deduplica por ID. */
        };

        ws.onclose = () => {
            setConnected(false);
            setConnecting(false);
            wsRef.current = null;
        };

        ws.onerror = () => {
            setConnected(false);
            setConnecting(false);
            wsRef.current = null;
        };

        ws.onmessage = (event) => {
            try {
                const msg: WsServerMessage = JSON.parse(event.data);

                switch (msg.type) {
                    case 'message':
                        if (msg.id && msg.session_id && msg.content) {
                            if (!sessionId) {
                                setSessionId(msg.session_id);
                                saveSessionId(msg.session_id);
                            }
                            /* [064A-29] Deduplicar: el backend envia historial al reconectar,
                             * y puede duplicar con mensajes que llegan por broadcast. */
                            setMessages(prev => {
                                if (prev.some(m => m.id === msg.id)) return prev;
                                return [
                                    ...prev,
                                    {
                                        id: msg.id!,
                                        session_id: msg.session_id!,
                                        sender_type: msg.sender || 'unknown',
                                        sender_id: msg.sender_id ?? null,
                                        content: msg.content!,
                                        created_at: msg.created_at || new Date().toISOString(),
                                        sender_avatar_url: null,
                                        sender_display_name: null,
                                    },
                                ];
                            });
                            /* Limpiar typing al recibir mensaje */
                            setTyping(null);
                            if (typingTimerRef.current) {
                                clearTimeout(typingTimerRef.current);
                                typingTimerRef.current = null;
                            }
                        }
                        break;

                    case 'typing':
                        /* [064A-54] No mostrar typing del propio visitante/cliente */
                        if (msg.sender && msg.sender !== 'visitor' && msg.sender !== 'client') {
                            setTyping({sender: msg.sender, content: msg.content || ''});
                            if (typingTimerRef.current) {
                                clearTimeout(typingTimerRef.current);
                            }
                            typingTimerRef.current = setTimeout(() => {
                                setTyping(null);
                                typingTimerRef.current = null;
                            }, 3000);
                        }
                        break;

                    case 'session_new':
                        if (msg.session?.id) {
                            const sid = String(msg.session.id);
                            setSessionId(sid);
                            saveSessionId(sid);
                        }
                        break;

                    case 'status':
                        /* Ignorar status changes en el widget */
                        break;

                    case 'error':
                        /* Log pero no crashear */
                        console.warn('[ChatWidget] Server error:', msg.message);
                        break;
                }
            } catch {
                /* Mensaje no JSON, ignorar */
            }
        };
    }, [sessionId]);

    const disconnect = useCallback(() => {
        if (wsRef.current) {
            wsRef.current.close();
            wsRef.current = null;
        }
        if (typingTimerRef.current) {
            clearTimeout(typingTimerRef.current);
            typingTimerRef.current = null;
        }
        setConnected(false);
        setTyping(null);
    }, []);

    const sendMessage = useCallback((content: string) => {
        if (wsRef.current?.readyState === WebSocket.OPEN) {
            wsRef.current.send(JSON.stringify({type: 'message', content}));
        }
    }, []);

    /* [054A-8] Throttle typing a 200ms para no saturar el WS */
    const sendTyping = useCallback((content: string) => {
        const now = Date.now();
        if (now - lastTypingSentRef.current < TYPING_THROTTLE_MS) return;
        lastTypingSentRef.current = now;

        if (wsRef.current?.readyState === WebSocket.OPEN) {
            wsRef.current.send(JSON.stringify({type: 'typing', content}));
        }
    }, []);

    /* Limpiar al desmontar */
    useEffect(() => () => disconnect(), [disconnect]);

    return {
        connected,
        connecting,
        messages,
        typing,
        sessionId,
        connect,
        disconnect,
        sendMessage,
        sendTyping,
    };
}

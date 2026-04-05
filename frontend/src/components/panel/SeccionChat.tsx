/* [044A-38 Fase 5] Sección "Mensajes" del panel.
 * Lista de conversaciones + chat activo. Soporta chat por orden y pre-venta.
 * Staff: ve todas las sesiones. Cliente: solo sus chats de órdenes. */

import React, {useState, useRef, useEffect, useCallback} from 'react';
import {MessageCircle, Send, Bot, User, ChevronLeft, XCircle} from 'lucide-react';
import {useChat, useChatWs} from '../../hooks/useChat';
import {
    apiCloseSession,
    SENDER_LABELS,
    SENDER_COLORS,
    SESSION_STATUS_LABELS,
    type ChatSession,
    type ChatMessage,
} from '../../api/chat';
import {toast} from '../../stores/toastStore';
import {Textarea} from '../ui/Textarea';
import './SeccionChat.css';

export const SeccionChat: React.FC = () => {
    const [activeSessionId, setActiveSessionId] = useState<string | null>(null);
    const {sessions, messages, cargandoSesiones, cargandoMensajes, enviarMensaje, enviando, crearSesion} =
        useChat(activeSessionId ?? undefined);

    /* [054A-3] WebSocket en tiempo real para typing + sesiones nuevas */
    const ws = useChatWs();

    /* Conectar WS al montar */
    useEffect(() => {
        ws.connect();
        return () => ws.disconnect();
    }, [ws.connect, ws.disconnect]);

    const [input, setInput] = useState('');
    const [closing, setClosing] = useState(false);
    const messagesEndRef = useRef<HTMLDivElement>(null);

    /* Auto-scroll al recibir mensajes */
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
    }, [input, activeSessionId, enviando, enviarMensaje]);

    const handleKeyDown = useCallback(
        (e: React.KeyboardEvent) => {
            if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                handleSend();
            }
        },
        [handleSend],
    );

    const handleNewChat = useCallback(async () => {
        try {
            const session = await crearSesion(undefined);
            setActiveSessionId(session.id);
        } catch {
            /* Error manejado internamente */
        }
    }, [crearSesion]);

    /* [054A-9] Cerrar sesión activa */
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

    /* Vista mobile: lista o chat */
    const showingChat = activeSessionId !== null;

    if (cargandoSesiones) {
        return (
            <div className="chatLoading">
                <div className="chatSpinner" />
                <p>Cargando conversaciones...</p>
            </div>
        );
    }

    return (
        <div className="chatContenedor">
            {/* Lista de sesiones */}
            <div className={`chatListaSesiones ${showingChat ? 'chatListaOculta' : ''}`}>
                <div className="chatListaHeader">
                    <h3>Conversaciones</h3>
                    <button
                        className="chatBtnNuevo"
                        onClick={handleNewChat}
                        type="button"
                    >
                        + Nuevo
                    </button>
                </div>
                {sessions.length === 0 ? (
                    <div className="chatVacio">
                        <MessageCircle size={32} strokeWidth={1.2} />
                        <p>Sin conversaciones activas</p>
                    </div>
                ) : (
                    sessions.map(s => (
                        <SessionItem
                            key={s.id}
                            session={s}
                            active={s.id === activeSessionId}
                            onClick={() => setActiveSessionId(s.id)}
                        />
                    ))
                )}
            </div>

            {/* Chat activo */}
            <div className={`chatAreaMensajes ${!showingChat ? 'chatAreaOculta' : ''}`}>
                {activeSessionId ? (
                    <>
                        <div className="chatAreaHeader">
                            <button
                                className="chatBtnVolver"
                                onClick={() => setActiveSessionId(null)}
                                type="button"
                            >
                                <ChevronLeft size={18} />
                            </button>
                            <span className="chatAreaTitulo">
                                {sessions.find(s => s.id === activeSessionId)?.order_id
                                    ? `Chat de orden`
                                    : 'Chat general'}
                            </span>
                            {/* [054A-9] Botón cerrar conversación */}
                            <button
                                className="chatBtnCerrar"
                                onClick={handleCloseSession}
                                disabled={closing}
                                type="button"
                                title="Cerrar conversación"
                            >
                                <XCircle size={18} />
                            </button>
                        </div>

                        <div className="chatMensajes">
                            {cargandoMensajes && (
                                <div className="chatLoading"><div className="chatSpinner" /></div>
                            )}
                            {messages.map(m => (
                                <MessageBubble key={m.id} message={m} />
                            ))}
                            {/* [054A-3] Typing indicator desde WebSocket */}
                            {activeSessionId && ws.typingMap[activeSessionId] && (
                                <div className="chatBurbuja chatBurbujaTyping">
                                    <div className="chatBurbujaHeader">
                                        <span style={{color: SENDER_COLORS[ws.typingMap[activeSessionId].sender] || '#94a3b8'}}>
                                            {SENDER_LABELS[ws.typingMap[activeSessionId].sender] || 'Visitante'} está escribiendo...
                                        </span>
                                    </div>
                                    <div className="chatBurbujaContenido chatTypingDots">
                                        <span /><span /><span />
                                    </div>
                                </div>
                            )}
                            <div ref={messagesEndRef} />
                        </div>

                        <div className="chatInputArea">
                            <Textarea
                                className="chatInput"
                                value={input}
                                onChange={e => setInput(e.target.value)}
                                onKeyDown={handleKeyDown}
                                placeholder="Escribe un mensaje..."
                                rows={1}
                            />
                            <button
                                className="chatBtnEnviar"
                                onClick={handleSend}
                                disabled={!input.trim() || enviando}
                                type="button"
                            >
                                <Send size={16} />
                            </button>
                        </div>
                    </>
                ) : (
                    <div className="chatVacio chatVacioCentrado">
                        <MessageCircle size={48} strokeWidth={1.2} />
                        <p>Selecciona una conversación</p>
                    </div>
                )}
            </div>
        </div>
    );
};

/* ============================================================
   SUB-COMPONENTES
   ============================================================ */

function SessionItem({
    session,
    active,
    onClick,
}: {
    session: ChatSession;
    active: boolean;
    onClick: () => void;
}) {
    return (
        <button
            className={`chatSesionItem ${active ? 'chatSesionActiva' : ''}`}
            onClick={onClick}
            type="button"
        >
            <div className="chatSesionIcono">
                {session.ai_enabled ? <Bot size={18} /> : <User size={18} />}
            </div>
            <div className="chatSesionInfo">
                <div className="chatSesionTitulo">
                    {session.order_id ? `Orden` : 'Chat general'}
                </div>
                <div className="chatSesionPreview">
                    {session.last_message || 'Sin mensajes'}
                </div>
            </div>
            <div className="chatSesionMeta">
                <span className="chatSesionEstado">
                    {SESSION_STATUS_LABELS[session.status] || session.status}
                </span>
            </div>
        </button>
    );
}

function MessageBubble({message}: {message: ChatMessage}) {
    const isAi = message.sender_type === 'ai';
    const isOwn = message.sender_type === 'client';
    const color = SENDER_COLORS[message.sender_type] || '#94a3b8';
    const label = SENDER_LABELS[message.sender_type] || message.sender_type;

    return (
        <div className={`chatBurbuja ${isOwn ? 'chatBurbujaPropia' : ''} ${isAi ? 'chatBurbujaIA' : ''}`}>
            <div className="chatBurbujaHeader">
                <span style={{color}}>{label}</span>
                <span className="chatBurbujaHora">
                    {new Date(message.created_at).toLocaleTimeString('es', {
                        hour: '2-digit',
                        minute: '2-digit',
                    })}
                </span>
            </div>
            <div className="chatBurbujaContenido">{message.content}</div>
        </div>
    );
}

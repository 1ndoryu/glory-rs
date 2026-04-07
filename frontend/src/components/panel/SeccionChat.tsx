/* [044A-38 Fase 5] Sección "Mensajes" del panel.
 * Lista de conversaciones + chat activo. Soporta chat por orden y pre-venta.
 * Staff: ve todas las sesiones. Cliente: solo sus chats de órdenes. */

import React from 'react';
import {MessageCircle, Send, Bot, User, ChevronLeft, XCircle} from 'lucide-react';
import {SENDER_LABELS, SESSION_STATUS_LABELS, type ChatSession, type ChatMessage} from '../../api/chat';
import {useSeccionChat} from '../../hooks/useSeccionChat';
import {Button} from '../ui/Button';
import {Textarea} from '../ui/Textarea';
import './SeccionChat.css';

export const SeccionChat: React.FC = () => {
    const {
        activeSessionId,
        sessions,
        messages,
        cargandoSesiones,
        cargandoMensajes,
        enviando,
        input,
        messagesEndRef,
        typingMap,
        showingChat,
        setInput,
        selectSession,
        clearActiveSession,
        handleKeyDown,
        handleSend,
    } = useSeccionChat();

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
                            onClick={() => selectSession(s.id)}
                        />
                    ))
                )}
            </div>

            {/* Chat activo */}
            <div className={`chatAreaMensajes ${!showingChat ? 'chatAreaOculta' : ''}`}>
                {activeSessionId ? (
                    <>
                        <div className="chatAreaHeader">
                            <Button
                                className="chatBtnVolver"
                                onClick={clearActiveSession}
                                type="button"
                                variante="texto"
                                tamano="pequeno"
                            >
                                <ChevronLeft size={18} />
                            </Button>
                            <span className="chatAreaTitulo">
                                {(() => {
                                    const s = sessions.find(ses => ses.id === activeSessionId);
                                    if (s?.order_id) return `Chat de orden #${s.order_number ?? '...'}`;
                                    return 'Chat general';
                                })()}
                            </span>
                            {/* [064A-71] Botón para deseleccionar la conversación activa,
                             * sin cerrarla permanentemente. El cierre definitivo (close_session)
                             * requiere acción explícita desde un menú contextual futuro. */}
                            <Button
                                className="chatBtnCerrar"
                                onClick={clearActiveSession}
                                type="button"
                                title="Volver a la lista"
                                variante="texto"
                                tamano="pequeno"
                            >
                                <XCircle size={18} />
                            </Button>
                        </div>

                        <div className="chatMensajes">
                            {cargandoMensajes && (
                                <div className="chatLoading"><div className="chatSpinner" /></div>
                            )}
                            {messages.map(m => (
                                <MessageBubble key={m.id} message={m} />
                            ))}
                            {/* [054A-3] Typing indicator desde WebSocket */}
                            {activeSessionId && typingMap[activeSessionId] && (
                                <div className="chatBurbuja chatBurbujaTyping">
                                    <div className="chatBurbujaHeader">
                                        <span className={resolveSenderToneClass(typingMap[activeSessionId].sender)}>
                                            {SENDER_LABELS[typingMap[activeSessionId].sender] || 'Visitante'} está escribiendo...
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
                            <Button
                                className="chatBtnEnviar"
                                onClick={() => void handleSend()}
                                disabled={!input.trim() || enviando}
                                type="button"
                                variante="primario"
                                tamano="pequeno"
                            >
                                <Send size={16} />
                            </Button>
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
        <Button
            className={`chatSesionItem ${active ? 'chatSesionActiva' : ''}`}
            onClick={onClick}
            type="button"
            variante="texto"
            tamano="pequeno"
        >
            <div className="chatSesionIcono">
                {session.ai_enabled ? <Bot size={18} /> : <User size={18} />}
            </div>
            <div className="chatSesionInfo">
                <div className="chatSesionTitulo">
                    {session.order_id
                        ? `Orden #${session.order_number ?? '...'}`
                        : 'Chat general'}
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
        </Button>
    );
}

function MessageBubble({message}: {message: ChatMessage}) {
    const isAi = message.sender_type === 'ai';
    const isOwn = message.sender_type === 'client';
    const label = SENDER_LABELS[message.sender_type] || message.sender_type;

    return (
        <div className={`chatBurbuja ${isOwn ? 'chatBurbujaPropia' : ''} ${isAi ? 'chatBurbujaIA' : ''}`}>
            <div className="chatBurbujaHeader">
                <span className={resolveSenderToneClass(message.sender_type)}>{label}</span>
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

function resolveSenderToneClass(senderType: string) {
    switch (senderType) {
        case 'admin':
            return 'chatRemitente chatRemitente--admin';
        case 'employee':
            return 'chatRemitente chatRemitente--employee';
        case 'client':
            return 'chatRemitente chatRemitente--client';
        case 'ai':
            return 'chatRemitente chatRemitente--ai';
        case 'visitor':
            return 'chatRemitente chatRemitente--visitor';
        default:
            return 'chatRemitente chatRemitente--neutral';
    }
}

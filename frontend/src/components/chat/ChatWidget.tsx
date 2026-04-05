/* [054A-3] ChatWidget: burbuja flotante de chat para visitantes.
 * Aparece en todas las páginas públicas (no en /panel).
 * Usa WebSocket de visitante anónimo. Persiste visitor_id en localStorage. */

import React, {useState, useRef, useEffect} from 'react';
import {useLocation} from 'react-router-dom';
import {MessageCircle, X, Send, Bot, User, Minus} from 'lucide-react';
import {useChatWidget} from '../../hooks/useChatWidget';
import {SENDER_LABELS} from '../../api/chat';
import {Input} from '../ui/Input';
import {Button} from '../ui/Button';
import './ChatWidget.css';

export const ChatWidget: React.FC = () => {
    const location = useLocation();
    const [open, setOpen] = useState(false);
    const [input, setInput] = useState('');
    const [visitorName, setVisitorName] = useState('');
    const [nameSubmitted, setNameSubmitted] = useState(false);
    const messagesEndRef = useRef<HTMLDivElement>(null);

    const {
        connected,
        connecting,
        messages,
        typing,
        connect,
        sendMessage,
        sendTyping,
    } = useChatWidget();

    /* No mostrar en /panel */
    if (location.pathname.startsWith('/panel')) return null;

    const handleOpen = () => {
        setOpen(true);
        if (!connected && !connecting && nameSubmitted) {
            connect(visitorName || undefined);
        }
    };

    const handleClose = () => {
        setOpen(false);
    };

    const handleMinimize = () => {
        setOpen(false);
    };

    const handleNameSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        setNameSubmitted(true);
        connect(visitorName || undefined);
    };

    const handleSend = () => {
        const content = input.trim();
        if (!content || !connected) return;
        sendMessage(content);
        setInput('');
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            handleSend();
        }
    };

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setInput(e.target.value);
        if (e.target.value.trim()) {
            sendTyping(e.target.value);
        }
    };

    return (
        <>
            {/* Burbuja flotante */}
            {!open && (
                <Button
                    variante="texto"
                    className="chatWidgetBubble"
                    onClick={handleOpen}
                    aria-label="Abrir chat"
                    type="button"
                >
                    <MessageCircle size={24} />
                </Button>
            )}

            {/* Panel de chat expandido */}
            {open && (
                <div className="chatWidgetPanel">
                    <ChatWidgetHeader
                        connected={connected}
                        connecting={connecting}
                        onMinimize={handleMinimize}
                        onClose={handleClose}
                    />

                    {!nameSubmitted ? (
                        <ChatWidgetNameForm
                            visitorName={visitorName}
                            onNameChange={setVisitorName}
                            onSubmit={handleNameSubmit}
                        />
                    ) : (
                        <>
                            <ChatWidgetMessages
                                messages={messages}
                                typing={typing}
                                messagesEndRef={messagesEndRef}
                            />
                            <ChatWidgetInput
                                input={input}
                                connected={connected}
                                onInputChange={handleInputChange}
                                onKeyDown={handleKeyDown}
                                onSend={handleSend}
                            />
                        </>
                    )}
                </div>
            )}
        </>
    );
};

/* ============================================================
   SUB-COMPONENTES (SRP — cada uno con una responsabilidad)
   ============================================================ */

function ChatWidgetHeader({
    connected,
    connecting,
    onMinimize,
    onClose,
}: {
    connected: boolean;
    connecting: boolean;
    onMinimize: () => void;
    onClose: () => void;
}) {
    return (
        <div className="chatWidgetHeader">
            <div className="chatWidgetHeaderInfo">
                <Bot size={20} />
                <div>
                    <span className="chatWidgetTitle">Nakomi Studio</span>
                    <span className="chatWidgetStatus">
                        {connecting
                            ? 'Conectando...'
                            : connected
                                ? 'En línea'
                                : 'Desconectado'}
                    </span>
                </div>
            </div>
            <div className="chatWidgetHeaderActions">
                <Button
                    variante="texto"
                    type="button"
                    className="chatWidgetHeaderBtn"
                    onClick={onMinimize}
                    aria-label="Minimizar chat"
                >
                    <Minus size={16} />
                </Button>
                <Button
                    variante="texto"
                    type="button"
                    className="chatWidgetHeaderBtn"
                    onClick={onClose}
                    aria-label="Cerrar chat"
                >
                    <X size={16} />
                </Button>
            </div>
        </div>
    );
}

function ChatWidgetNameForm({
    visitorName,
    onNameChange,
    onSubmit,
}: {
    visitorName: string;
    onNameChange: (name: string) => void;
    onSubmit: (e: React.FormEvent) => void;
}) {
    return (
        <form className="chatWidgetNameForm" onSubmit={onSubmit}>
            <p className="chatWidgetWelcome">
                ¡Hola! ¿Cómo te podemos ayudar?
            </p>
            <Input
                type="text"
                placeholder="Tu nombre (opcional)"
                value={visitorName}
                onChange={(e) => onNameChange(e.target.value)}
                className="chatWidgetNameInput"
            />
            <Button type="submit" className="chatWidgetStartBtn">
                Iniciar conversación
            </Button>
        </form>
    );
}

function ChatWidgetMessages({
    messages,
    typing,
    messagesEndRef,
}: {
    messages: Array<{
        id: string;
        sender_type: string;
        sender_id: string | null;
        content: string;
        created_at: string;
    }>;
    typing: {sender: string; content: string} | null;
    messagesEndRef: React.RefObject<HTMLDivElement>;
}) {
    /* Auto-scroll al recibir mensajes */
    useEffect(() => {
        messagesEndRef.current?.scrollIntoView({behavior: 'smooth'});
    }, [messages, typing, messagesEndRef]);

    return (
        <div className="chatWidgetMessages">
            {messages.length === 0 && (
                <p className="chatWidgetEmpty">
                    Envía un mensaje para comenzar la conversación.
                    Un asistente te responderá pronto.
                </p>
            )}
            {messages.map((msg) => {
                const isOwn = msg.sender_type === 'visitor' || msg.sender_type === 'client';
                const label = SENDER_LABELS[msg.sender_type] || msg.sender_type;
                const icon = msg.sender_type === 'ai' ? <Bot size={14} /> : <User size={14} />;

                return (
                    <div
                        key={msg.id}
                        className={`chatWidgetMsg ${isOwn ? 'chatWidgetMsgOwn' : 'chatWidgetMsgOther'}`}
                    >
                        {!isOwn && (
                            <span className={`chatWidgetMsgSender chatWidgetSender--${msg.sender_type}`}>
                                {icon} {label}
                            </span>
                        )}
                        <div className={`chatWidgetMsgBubble ${isOwn ? 'chatWidgetMsgBubbleOwn' : 'chatWidgetMsgBubbleOther'}`}>
                            {msg.content}
                        </div>
                    </div>
                );
            })}

            {typing && (
                <div className="chatWidgetMsg chatWidgetMsgOther">
                    <span className={`chatWidgetMsgSender chatWidgetSender--${typing.sender}`}>
                        {typing.sender === 'ai' ? <Bot size={14} /> : <User size={14} />}
                        {' '}{SENDER_LABELS[typing.sender] || typing.sender}
                    </span>
                    <div className="chatWidgetMsgBubble chatWidgetMsgBubbleOther chatWidgetTyping">
                        <span className="chatWidgetTypingDots">
                            <span />
                            <span />
                            <span />
                        </span>
                    </div>
                </div>
            )}

            <div ref={messagesEndRef} />
        </div>
    );
}

function ChatWidgetInput({
    input,
    connected,
    onInputChange,
    onKeyDown,
    onSend,
}: {
    input: string;
    connected: boolean;
    onInputChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    onKeyDown: (e: React.KeyboardEvent) => void;
    onSend: () => void;
}) {
    return (
        <div className="chatWidgetInputArea">
            <Input
                type="text"
                placeholder={connected ? 'Escribe un mensaje...' : 'Conectando...'}
                value={input}
                onChange={onInputChange}
                onKeyDown={onKeyDown}
                disabled={!connected}
                className="chatWidgetInput"
            />
            <Button
                variante="texto"
                type="button"
                className="chatWidgetSendBtn"
                onClick={onSend}
                disabled={!connected || !input.trim()}
                aria-label="Enviar mensaje"
            >
                <Send size={18} />
            </Button>
        </div>
    );
}

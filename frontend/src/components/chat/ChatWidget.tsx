/* [064A-28] ChatWidget: redesign completo. Sin header, burbuja con avatar,
 * fondo semi-transparente rgba(245,242,239,0.8), sombra especifica del usuario,
 * animacion suave open/close via CSS transitions. Botones #e8e6e2/#e9e7e4.
 * Requisitos explicitos del usuario — no modificar sin instruccion. */

import React, {useState, useRef, useEffect} from 'react';
import {useLocation} from 'react-router-dom';
import {Send, Bot, User, Minus} from 'lucide-react';
import {useChatWidget} from '../../hooks/useChatWidget';
import {SENDER_LABELS} from '../../api/chat';
import {useChatStore} from '../../stores/chatStore';
import {Input} from '../ui/Input';
import {Button} from '../ui/Button';
import './ChatWidget.css';

const AVATAR_SRC = '/assets/random/85a51ba9a4233272662e744b48f97d67.jpg';

export const ChatWidget: React.FC = () => {
    const location = useLocation();
    const abierto = useChatStore(s => s.abierto);
    const abrir = useChatStore(s => s.abrir);
    const cerrar = useChatStore(s => s.cerrar);
    const [input, setInput] = useState('');
    const [visitorName, setVisitorName] = useState('');
    const messagesEndRef = useRef<HTMLDivElement>(null);

    const {
        connected,
        connecting,
        messages,
        typing,
        sessionId,
        connect,
        sendMessage,
        sendTyping,
    } = useChatWidget();

    /* [064A-29] Si hay sesion previa (sessionId en localStorage), saltar
     * formulario de nombre y reconectar automaticamente al abrir. */
    const [nameSubmitted, setNameSubmitted] = useState(() => !!sessionId);

    if (location.pathname.startsWith('/panel')) return null;

    const handleOpen = () => {
        abrir();
        if (!connected && !connecting) {
            if (nameSubmitted || sessionId) {
                setNameSubmitted(true);
                connect(visitorName || undefined);
            }
        }
    };

    const handleMinimize = () => {
        cerrar();
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
            {/* [064A-28] Burbuja flotante con avatar + texto "Chat" */}
            <Button
                variante="texto"
                tamano="pequeno"
                className={`chatWidgetBubble ${abierto ? 'chatWidgetBubbleOculta' : ''}`}
                onClick={handleOpen}
                aria-label="Abrir chat"
                type="button"
            >
                <img src={AVATAR_SRC} alt="" className="chatWidgetBubbleAvatar" />
                <span className="chatWidgetBubbleTexto">Chat</span>
            </Button>

            {/* [064A-28] Panel sin header, con minimize btn flotante */}
            <div className={`chatWidgetPanel ${abierto ? 'chatWidgetPanelAbierto' : ''}`}>
                <Button
                    variante="texto"
                    tamano="pequeno"
                    className="chatWidgetMinimizeBtn"
                    onClick={handleMinimize}
                    aria-label="Minimizar chat"
                    type="button"
                >
                    <Minus size={16} />
                </Button>

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
        </>
    );
};

/* Sub-componentes (SRP) */

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
            <img src={AVATAR_SRC} alt="" className="chatWidgetFormAvatar" />
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
            <Button type="submit" variante="texto" tamano="pequeno" className="chatWidgetStartBtn">
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
                tamano="pequeno"
                className="chatWidgetSendBtn"
                onClick={onSend}
                disabled={!connected || !input.trim()}
                aria-label="Enviar mensaje"
                type="button"
            >
                <Send size={18} />
            </Button>
        </div>
    );
}

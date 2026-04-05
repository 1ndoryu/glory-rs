/* [044A-43] Componente Textarea reutilizable — wrapper de <textarea> nativo. */
import React from 'react';
import './Textarea.css';

interface TextareaProps extends React.TextareaHTMLAttributes<HTMLTextAreaElement> {
    variante?: 'default' | 'outline';
}

export const Textarea: React.FC<TextareaProps> = ({className = '', variante = 'default', ...props}) => {
    const claseVariante = `textarea${variante.charAt(0).toUpperCase() + variante.slice(1)}`;
    return (
        <textarea className={`textareaBase ${claseVariante} ${className}`} {...props} />
    );
};

/* [044A-43] Componente Input reutilizable — wrapper de <input> nativo.
 * Mantiene acceso completo a InputHTMLAttributes. */
import React from 'react';
import './Input.css';

interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
    variante?: 'default' | 'outline';
}

export const Input: React.FC<InputProps> = ({className = '', variante = 'default', ...props}) => {
    const claseVariante = `input${variante.charAt(0).toUpperCase() + variante.slice(1)}`;
    return (
        <input className={`inputBase ${claseVariante} ${className}`} {...props} />
    );
};

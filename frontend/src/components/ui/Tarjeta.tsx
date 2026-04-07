/* [084A-6] Componente base Tarjeta — wrapper visual reutilizable.
 * Centraliza estilo de tarjeta (borde, fondo, radius, padding).
 * Se usa como contenedor en features, servicios, etc. */

import React from 'react';
import './Tarjeta.css';

interface TarjetaProps {
    children: React.ReactNode;
    className?: string;
    fondo?: string;
    onClick?: () => void;
}

export const Tarjeta: React.FC<TarjetaProps> = ({children, className, fondo, onClick}) => {
    const estiloInline = fondo ? {backgroundColor: fondo} : undefined;
    const Tag = onClick ? 'button' : 'div';

    return (
        <Tag
            className={`tarjetaBase ${className ?? ''}`}
            style={estiloInline}
            onClick={onClick}
            {...(onClick ? {type: 'button' as const} : {})}
        >
            {children}
        </Tag>
    );
};

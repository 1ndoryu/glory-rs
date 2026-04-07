import React from 'react';
import './Button.css';

/* [064A-1] Variantes semánticas: cubren estados (exito, peligro, advertencia, info)
 * y variantes suaves (Suave) para acciones secundarias. */
type ButtonVariante =
    | 'primario' | 'secundario' | 'outline' | 'texto'
    | 'exito' | 'exitoSuave'
    | 'peligro' | 'peligroSuave'
    | 'advertencia' | 'advertenciaSuave'
    | 'info' | 'infoSuave';

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
    variante?: ButtonVariante;
    tamano?: 'pequeno' | 'mediano' | 'grande';
}

/**
 * Componente Boton reutilizable
 * Soporta variantes y tamaños configurables via props.
 */
export const Button: React.FC<ButtonProps> = ({children, className = '', variante = 'primario', tamano = 'mediano', ...props}) => {
    const claseVariante = `boton${variante.charAt(0).toUpperCase() + variante.slice(1)}`;
    const claseTamano = `boton${tamano.charAt(0).toUpperCase() + tamano.slice(1)}`;

    return (
        <button className={`botonBase ${claseVariante} ${claseTamano} ${className}`} {...props}>
            {children}
        </button>
    );
};

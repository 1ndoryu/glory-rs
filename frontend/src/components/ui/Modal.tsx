/* [044A-40] Componente base Modal reutilizable.
 * Todos los modales del sistema deben usar este componente como contenedor.
 * Incluye: overlay con click-to-close, Escape listener, focus trap, scroll lock.
 * No incluye botón X (decisión de diseño del roadmap). */
import React, {useEffect, useRef} from 'react';
import {useFocusTrap} from '../../hooks/useFocusTrap';
import './Modal.css';

interface ModalProps {
    abierto: boolean;
    onCerrar: () => void;
    children: React.ReactNode;
    /** Clase CSS adicional para el contenedor interno */
    className?: string;
}

export const Modal: React.FC<ModalProps> = ({abierto, onCerrar, children, className}) => {
    const modalRef = useRef<HTMLDivElement>(null);
    useFocusTrap(modalRef, abierto);

    /* Cerrar con Escape y bloquear scroll del body */
    useEffect(() => {
        if (!abierto) return;

        const handleEscape = (e: KeyboardEvent) => {
            if (e.key === 'Escape') onCerrar();
        };

        document.addEventListener('keydown', handleEscape);
        document.body.style.overflow = 'hidden';

        return () => {
            document.removeEventListener('keydown', handleEscape);
            document.body.style.overflow = '';
        };
    }, [abierto, onCerrar]);

    if (!abierto) return null;

    const handleOverlayClick = (e: React.MouseEvent<HTMLDivElement>) => {
        if (e.target === e.currentTarget) onCerrar();
    };

    const claseContenedor = className
        ? `modalBaseContenedor ${className}`
        : 'modalBaseContenedor';

    return (
        <div className="modalBaseOverlay" onClick={handleOverlayClick}>
            <div className={claseContenedor} ref={modalRef} role="dialog" aria-modal="true">
                {children}
            </div>
        </div>
    );
};

/* [054A-5] Contenedor de toasts — se monta una vez en App.tsx.
 * Renderiza la lista de toasts activos con animación de entrada/salida. */
import React from 'react';
import { X, CheckCircle, AlertCircle, Info, AlertTriangle } from 'lucide-react';
import { useToastStore, type ToastType } from '../../stores/toastStore';
import { Button } from './Button';
import './ToastContainer.css';

const ICONS: Record<ToastType, React.ElementType> = {
    success: CheckCircle,
    error: AlertCircle,
    info: Info,
    warning: AlertTriangle,
};

export const ToastContainer: React.FC = () => {
    const toasts = useToastStore((s) => s.toasts);
    const removeToast = useToastStore((s) => s.removeToast);

    if (toasts.length === 0) return null;

    return (
        <div className="toastContenedor" aria-live="polite">
            {toasts.map((t) => {
                const Icon = ICONS[t.type];
                return (
                    <div key={t.id} className={`toastItem toastItem${t.type}`}>
                        <Icon size={18} className="toastIcono" />
                        <span className="toastMensaje">{t.message}</span>
                        <Button
                            variante="texto"
                            tamano="pequeno"
                            className="toastCerrar"
                            onClick={() => removeToast(t.id)}
                            aria-label="Cerrar"
                        >
                            <X size={14} />
                        </Button>
                    </div>
                );
            })}
        </div>
    );
};

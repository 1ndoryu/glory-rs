/* [044A-38 Fase 4] Sección "Disponibles" del panel (empleado/admin).
 * Muestra órdenes sin asignar con botón "Tomar" para que el empleado se auto-asigne.
 * Usa useDisponibles hook con React Query + invalidación de cache. */
import React, {useCallback, useState} from 'react';
import {FolderOpen, UserPlus} from 'lucide-react';
import {useDisponibles} from '../../hooks/useAsignaciones';
import {ORDER_STATUS_LABELS, PAYMENT_MODE_LABELS, formatPrice} from '../../api/orders';
import type {OrderResponse} from '../../api/orders';
import './SeccionDisponibles.css';

export const SeccionDisponibles: React.FC = () => {
    const {disponibles, cargando, tomarOrden, tomando} = useDisponibles();
    const [error, setError] = useState<string | null>(null);

    const handleTomar = useCallback(async (orderId: string) => {
        try {
            setError(null);
            await tomarOrden(orderId);
        } catch (err: unknown) {
            const msg = extraerError(err);
            setError(msg);
        }
    }, [tomarOrden]);

    if (cargando) {
        return (
            <div className="disponiblesLoading">
                <div className="disponiblesSpinner" />
                <p>Cargando órdenes disponibles...</p>
            </div>
        );
    }

    if (disponibles.length === 0) {
        return (
            <div className="disponiblesVacio">
                <FolderOpen size={48} strokeWidth={1.2} />
                <h3>Sin órdenes disponibles</h3>
                <p>No hay órdenes esperando asignación en este momento.</p>
            </div>
        );
    }

    return (
        <div className="disponiblesContenedor">
            {error && <div className="disponiblesError">{error}</div>}
            <div className="disponiblesLista">
                {disponibles.map(orden => (
                    <OrdenDisponibleCard
                        key={orden.id}
                        orden={orden}
                        onTomar={handleTomar}
                        tomando={tomando}
                    />
                ))}
            </div>
        </div>
    );
};

function OrdenDisponibleCard({
    orden,
    onTomar,
    tomando,
}: {
    orden: OrderResponse;
    onTomar: (id: string) => void;
    tomando: boolean;
}) {
    return (
        <div className="disponibleCard">
            <div className="disponibleCardHeader">
                <div className="disponibleCardInfo">
                    <span className="disponibleCardNumero">#{orden.order_number}</span>
                    <h3 className="disponibleCardTitulo">{orden.service_title}</h3>
                    <span className="disponibleCardPlan">{orden.plan_name}</span>
                </div>
                <span className="disponibleCardBadge">
                    {ORDER_STATUS_LABELS[orden.status]}
                </span>
            </div>

            <div className="disponibleCardDetalles">
                <span>{formatPrice(orden.final_price_cents, orden.currency)}</span>
                <span>{PAYMENT_MODE_LABELS[orden.payment_mode]}</span>
                <span>{orden.total_phases} fases</span>
                <span>{new Date(orden.created_at).toLocaleDateString('es')}</span>
            </div>

            {orden.client_notes && (
                <p className="disponibleCardNotas">{orden.client_notes}</p>
            )}

            <button
                className="disponibleBtnTomar"
                onClick={() => onTomar(orden.id)}
                disabled={tomando}
                type="button"
            >
                <UserPlus size={16} />
                {tomando ? 'Asignando...' : 'Tomar orden'}
            </button>
        </div>
    );
}

function extraerError(err: unknown): string {
    if (typeof err === 'object' && err !== null && 'response' in err) {
        const resp = (err as {response?: {data?: {message?: string; error?: string}}}).response;
        if (resp?.data?.message) return resp.data.message;
        if (resp?.data?.error) return resp.data.error;
    }
    return 'Error al tomar la orden';
}

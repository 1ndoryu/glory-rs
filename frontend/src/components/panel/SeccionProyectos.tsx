/* [044A-38 Fase 2] Sección "Mis Proyectos" del panel.
 * Muestra lista de órdenes con progreso visual + detalle con fases.
 * Vista lista ↔ detalle. Detalle extraído a OrdenDetalle.tsx (SRP). */
import React, {useCallback} from 'react';
import {FolderOpen} from 'lucide-react';
import {useAuthStore} from '../../stores/authStore';
import {useOrdenes} from '../../hooks/useOrdenes';
import {OrdenDetalle} from './OrdenDetalle';
import {
    ORDER_STATUS_LABELS,
    PAYMENT_MODE_LABELS,
    formatPrice,
    type OrderResponse,
} from '../../api/orders';
import './SeccionProyectos.css';

/* Mapa status → clase CSS (evita inline style) */
const STATUS_CLASS: Record<string, string> = {
    pending_payment: 'ordenBadge--pendingPayment',
    payment_held: 'ordenBadge--paymentHeld',
    awaiting_assignment: 'ordenBadge--awaiting',
    in_progress: 'ordenBadge--inProgress',
    under_review: 'ordenBadge--underReview',
    completed: 'ordenBadge--completed',
    cancelled: 'ordenBadge--cancelled',
    disputed: 'ordenBadge--disputed',
};

export const SeccionProyectos: React.FC = () => {
    const effectiveRole = useAuthStore(s => s.user?.effectiveRole) || 'client';
    const {
        ordenes, cargando, detalle, cargandoDetalle,
        ordenSeleccionada, error, seleccionarOrden, recargar,
        cancelarOrden, aprobarFase, solicitarRevision, cancelando,
    } = useOrdenes();

    const handleVolver = useCallback(() => seleccionarOrden(null), [seleccionarOrden]);
    const handleCancelar = useCallback(async (orderId: string) => {
        if (!window.confirm('¿Cancelar esta orden? Esta acción no se puede deshacer.')) return;
        await cancelarOrden(orderId);
    }, [cancelarOrden]);
    const handleAprobar = useCallback(async (orderId: string, phase: number) => {
        await aprobarFase({orderId, phase});
    }, [aprobarFase]);
    const handleRevision = useCallback(async (orderId: string, phase: number) => {
        await solicitarRevision({orderId, phase});
    }, [solicitarRevision]);
    if (cargando) {
        return <div className="proyectosLoading"><div className="proyectosSpinner" /><p>Cargando proyectos...</p></div>;
    }
    if (error) {
        return <div className="proyectosError">{error}</div>;
    }

    /* Vista detalle */
    if (ordenSeleccionada && detalle) {
        return (
            <OrdenDetalle
                order={detalle.order}
                phases={detalle.phases}
                effectiveRole={effectiveRole}
                onVolver={handleVolver}
                onCancelar={handleCancelar}
                onAprobar={handleAprobar}
                onRevision={handleRevision}
                onPagoExitoso={recargar}
                cancelando={cancelando}
            />
        );
    }
    if (ordenSeleccionada && cargandoDetalle) {
        return <div className="proyectosLoading"><div className="proyectosSpinner" /><p>Cargando detalle...</p></div>;
    }

    /* Lista vacía */
    if (ordenes.length === 0) {
        return (
            <div className="proyectosVacio">
                <FolderOpen size={48} className="proyectosVacioIcono" strokeWidth={1.2} />
                <h3 className="proyectosVacioTitulo">Sin proyectos aún</h3>
                <p className="proyectosVacioTexto">
                    Cuando contrates un servicio, aparecerá aquí con seguimiento en tiempo real.
                </p>
            </div>
        );
    }

    /* Lista de órdenes */
    return (
        <div className="proyectosLista">
            {ordenes.map(orden => (
                <OrdenCard key={orden.id} orden={orden} onClick={() => seleccionarOrden(orden.id)} />
            ))}
        </div>
    );
};

/* Sub-componente: card de orden en la lista */
function OrdenCard({orden, onClick}: {orden: OrderResponse; onClick: () => void}) {
    const progreso = orden.total_phases > 0 ? Math.round((orden.current_phase / orden.total_phases) * 100) : 0;

    return (
        <button className="ordenCard" onClick={onClick} type="button">
            <div className="ordenCardHeader">
                <div className="ordenCardInfo">
                    <span className="ordenCardNumero">#{orden.order_number}</span>
                    <h3 className="ordenCardTitulo">{orden.service_title}</h3>
                    <span className="ordenCardPlan">{orden.plan_name}</span>
                </div>
                <span className={`ordenCardBadge ${STATUS_CLASS[orden.status] || ''}`}>
                    {ORDER_STATUS_LABELS[orden.status]}
                </span>
            </div>
            {orden.total_phases > 0 && (
                <div className="ordenCardProgreso">
                    <div className="ordenCardProgresoBarra">
                        <div className="ordenCardProgresoLleno" style={{width: `${progreso}%`}} />
                    </div>
                    <span className="ordenCardProgresoTexto">
                        {orden.current_phase}/{orden.total_phases} fases · {progreso}%
                    </span>
                </div>
            )}
            <div className="ordenCardFooter">
                <span className="ordenCardPrecio">{formatPrice(orden.final_price_cents, orden.currency)}</span>
                <span className="ordenCardModo">{PAYMENT_MODE_LABELS[orden.payment_mode]}</span>
                <span className="ordenCardFecha">{new Date(orden.created_at).toLocaleDateString('es')}</span>
            </div>
        </button>
    );
}

/* [044A-38 Fase 2+3] Detalle de una orden con timeline de fases.
 * Muestra info general + fases con acciones según rol.
 * Fase 3: botón de pago Stripe integrado para pending_payment. */
import React, { useState } from 'react';
import {Check, RotateCcw, Package, CreditCard} from 'lucide-react';
import {
    ORDER_STATUS_LABELS,
    PHASE_STATUS_LABELS,
    PAYMENT_MODE_LABELS,
    formatPrice,
    type OrderResponse,
    type OrderPhaseResponse,
    type OrderStatus,
    type PhaseStatus,
} from '../../api/orders';
import CheckoutModal from './CheckoutModal';
import './SeccionProyectos.css';

/* Fases que el cliente puede cancelar */
const CANCELABLE_STATUSES: OrderStatus[] = ['pending_payment', 'payment_held', 'awaiting_assignment'];
const APPROVABLE_PHASE: PhaseStatus[] = ['delivered'];
const REVISABLE_PHASE: PhaseStatus[] = ['delivered'];

/* Mapa status → clase CSS para colores */
const STATUS_CLASS: Record<OrderStatus, string> = {
    pending_payment: 'ordenBadge--pendingPayment',
    payment_held: 'ordenBadge--paymentHeld',
    awaiting_assignment: 'ordenBadge--awaiting',
    in_progress: 'ordenBadge--inProgress',
    under_review: 'ordenBadge--underReview',
    completed: 'ordenBadge--completed',
    cancelled: 'ordenBadge--cancelled',
    disputed: 'ordenBadge--disputed',
};

interface OrdenDetalleProps {
    order: OrderResponse;
    phases: OrderPhaseResponse[];
    effectiveRole: string;
    onVolver: () => void;
    onCancelar: (orderId: string) => Promise<void>;
    onAprobar: (orderId: string, phase: number) => Promise<void>;
    onRevision: (orderId: string, phase: number) => Promise<void>;
    onEntregar: (orderId: string, phase: number) => Promise<void>;
    onPagoExitoso: () => void;
    cancelando: boolean;
}

export const OrdenDetalle: React.FC<OrdenDetalleProps> = ({
    order, phases, effectiveRole, onVolver, onCancelar, onAprobar, onRevision, onEntregar, onPagoExitoso, cancelando,
}) => {
    const [checkout, setCheckout] = useState<{
        amountCents: number;
        phaseNumber?: number;
    } | null>(null);

    const canCancel = CANCELABLE_STATUSES.includes(order.status);
    const needsPayment = order.status === 'pending_payment';
    const isPhased = order.payment_mode === 'phased';
    const isEmployee = effectiveRole === 'employee';
    const isClient = effectiveRole === 'client' || effectiveRole === 'admin';

    return (
        <div className="ordenDetalle">
            <div className="ordenDetalleHeader">
                <button className="ordenDetalleVolver" onClick={onVolver} type="button">
                    ← Volver
                </button>
                <div className="ordenDetalleResumen">
                    <h2 className="ordenDetalleTitulo">
                        #{order.order_number} — {order.service_title}
                    </h2>
                    <div className="ordenDetalleMetaRow">
                        <span className={`ordenDetalleBadge ${STATUS_CLASS[order.status]}`}>
                            {ORDER_STATUS_LABELS[order.status]}
                        </span>
                        <span className="ordenDetalleMeta">{order.plan_name}</span>
                        <span className="ordenDetalleMeta">{PAYMENT_MODE_LABELS[order.payment_mode]}</span>
                        <span className="ordenDetalleMeta">{formatPrice(order.final_price_cents, order.currency)}</span>
                    </div>
                </div>
                {canCancel && (
                    <button
                        className="ordenDetalleCancelar"
                        onClick={() => onCancelar(order.id)}
                        disabled={cancelando}
                        type="button"
                    >
                        {cancelando ? 'Cancelando...' : 'Cancelar orden'}
                    </button>
                )}
                {needsPayment && !isPhased && (
                    <button
                        className="ordenDetallePagar"
                        onClick={() => setCheckout({ amountCents: order.final_price_cents })}
                        type="button"
                    >
                        <CreditCard size={16} /> Pagar
                    </button>
                )}
            </div>

            {checkout && (
                <CheckoutModal
                    orderId={order.id}
                    orderNumber={order.order_number}
                    amountCents={checkout.amountCents}
                    currency={order.currency}
                    phaseNumber={checkout.phaseNumber}
                    onClose={() => setCheckout(null)}
                    onSuccess={() => {
                        setCheckout(null);
                        onPagoExitoso();
                    }}
                />
            )}

            {order.client_notes && (
                <div className="ordenDetalleNotas">
                    <strong>Notas:</strong> {order.client_notes}
                </div>
            )}

            <div className="ordenDetalleFases">
                <h3 className="ordenDetalleFasesTitulo">Fases del proyecto</h3>
                <div className="fasesTimeline">
                    {phases.map((phase, idx) => (
                        <FaseCard
                            key={phase.phase_number}
                            phase={phase}
                            orderId={order.id}
                            isLast={idx === phases.length - 1}
                            isClient={isClient}
                            isEmployee={isEmployee}
                            isPhased={isPhased}
                            onAprobar={onAprobar}
                            onRevision={onRevision}
                            onEntregar={onEntregar}
                            onPagarFase={(pn, amount) => setCheckout({ amountCents: amount, phaseNumber: pn })}
                        />
                    ))}
                </div>
            </div>
        </div>
    );
};

/* Componente privado: card de una fase dentro del timeline */
function FaseCard({phase, orderId, isLast, isClient, isEmployee, isPhased, onAprobar, onRevision, onEntregar, onPagarFase}: {
    phase: OrderPhaseResponse;
    orderId: string;
    isLast: boolean;
    isClient: boolean;
    isEmployee: boolean;
    isPhased: boolean;
    onAprobar: (orderId: string, phase: number) => Promise<void>;
    onRevision: (orderId: string, phase: number) => Promise<void>;
    onEntregar: (orderId: string, phase: number) => Promise<void>;
    onPagarFase: (phaseNumber: number, amountCents: number) => void;
}) {
    const canApprove = isClient && APPROVABLE_PHASE.includes(phase.status);
    const canRevise = isClient && REVISABLE_PHASE.includes(phase.status) && phase.revisions_used < phase.max_revisions;
    const canDeliver = isEmployee && (phase.status === 'in_progress' || phase.status === 'paid' || phase.status === 'revision_requested');
    const canPayPhase = isClient && isPhased && phase.status === 'pending_payment';
    const isActive = phase.status !== 'locked' && phase.status !== 'approved' && phase.status !== 'skipped';

    return (
        <div className={`faseCard ${isActive ? 'faseCardActiva' : ''} ${phase.status === 'approved' ? 'faseCardAprobada' : ''}`}>
            <div className="faseTimelineDot">
                <div className={`faseDot ${phase.status === 'approved' ? 'faseDotAprobada' : ''} ${isActive ? 'faseDotActiva' : ''}`} />
                {!isLast && <div className="faseTimelineLine" />}
            </div>

            <div className="faseCardContenido">
                <div className="faseCardHeader">
                    <div className="faseCardTitulo">
                        <span className="faseNumero">Fase {phase.phase_number}</span>
                        <h4 className="faseNombre">{phase.title}</h4>
                    </div>
                    <span className={`faseBadge faseBadge--${phase.status.replace('_', '-')}`}>
                        {PHASE_STATUS_LABELS[phase.status]}
                    </span>
                </div>

                {phase.description && <p className="faseDescripcion">{phase.description}</p>}

                <div className="faseMeta">
                    <span>{formatPrice(phase.price_cents)}</span>
                    <span>{phase.estimated_days} días est.</span>
                    <span>Revisiones: {phase.revisions_used}/{phase.max_revisions}</span>
                </div>

                {(canApprove || canRevise || canDeliver || canPayPhase) && (
                    <div className="faseAcciones">
                        {canPayPhase && (
                            <button
                                className="faseBtn faseBtnPagar"
                                onClick={() => onPagarFase(phase.phase_number, phase.price_cents)}
                                type="button"
                            >
                                <CreditCard size={14} /> Pagar fase
                            </button>
                        )}
                        {canApprove && (
                            <button
                                className="faseBtn faseBtnAprobar"
                                onClick={() => onAprobar(orderId, phase.phase_number)}
                                type="button"
                            >
                                <Check size={14} /> Aprobar
                            </button>
                        )}
                        {canRevise && (
                            <button
                                className="faseBtn faseBtnRevision"
                                onClick={() => onRevision(orderId, phase.phase_number)}
                                type="button"
                            >
                                <RotateCcw size={14} /> Revisión
                            </button>
                        )}
                        {canDeliver && (
                            <button
                                className="faseBtn faseBtnEntregar"
                                onClick={() => onEntregar(orderId, phase.phase_number)}
                                type="button"
                            >
                                <Package size={14} /> Entregar
                            </button>
                        )}
                    </div>
                )}
            </div>
        </div>
    );
}

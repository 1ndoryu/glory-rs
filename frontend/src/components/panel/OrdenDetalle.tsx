/* [044A-38 Fase 2+3+6] Detalle de una orden con timeline de fases.
 * Muestra info general + fases con acciones según rol.
 * Fase 3: botón de pago Stripe integrado para pending_payment.
 * Fase 6: upload multipart de entregables + listado + descarga por fase. */
import React, {useState} from 'react';
import {Check, RotateCcw, CreditCard, XCircle} from 'lucide-react';
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
import {EntregablesPanel} from './EntregablesPanel';
import {ReviewPanel} from './ReviewPanel';
import {Button} from '../ui/Button';
import {MenuContextual, type MenuContextualItem} from '../ui/ContextMenu';
import {Modal} from '../ui/Modal';
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
    onPagoExitoso: () => void;
    cancelando: boolean;
}

export const OrdenDetalle: React.FC<OrdenDetalleProps> = ({
    order, phases, effectiveRole, onVolver, onCancelar, onAprobar, onRevision, onPagoExitoso, cancelando,
}) => {
    const [checkout, setCheckout] = useState<{
        amountCents: number;
        phaseNumber?: number;
    } | null>(null);
    const [menuAbierto, setMenuAbierto] = useState(false);
    const [modalCancelarAbierto, setModalCancelarAbierto] = useState(false);

    const canCancel = CANCELABLE_STATUSES.includes(order.status);
    const needsPayment = order.status === 'pending_payment';
    const isPhased = order.payment_mode === 'phased';
    const shouldShowPhases = isPhased;
    const isEmployee = effectiveRole === 'employee';
    const isClient = effectiveRole === 'client' || effectiveRole === 'admin';

    const abrirCancelacion = () => {
        setModalCancelarAbierto(true);
    };

    const confirmarCancelacion = async () => {
        await onCancelar(order.id);
        setModalCancelarAbierto(false);
    };

    const menuItems: MenuContextualItem[] = canCancel
        ? [{
            id: 'cancel-order',
            label: cancelando ? 'Cancelando...' : 'Cancelar orden',
            onSelect: abrirCancelacion,
            disabled: cancelando,
            danger: true,
            icon: <XCircle size={16} />,
        }]
        : [];

    return (
        <div className="ordenDetalle">
            <div className="ordenDetalleTopbar">
                <Button className="ordenDetalleVolver" onClick={onVolver} type="button" variante="texto" tamano="pequeno">
                    ← Volver
                </Button>
            </div>

            <div className="ordenDetalleHeader">
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
                <div className="ordenDetalleAcciones">
                    {needsPayment && !isPhased && (
                        <Button
                            className="ordenDetallePagar"
                            onClick={() => setCheckout({amountCents: order.final_price_cents})}
                            type="button"
                            variante="exito"
                            tamano="pequeno"
                        >
                            <CreditCard size={16} /> Pagar
                        </Button>
                    )}

                    {canCancel && (
                        <MenuContextual
                            abierto={menuAbierto}
                            onToggle={() => setMenuAbierto(prev => !prev)}
                            onCerrar={() => setMenuAbierto(false)}
                            items={menuItems}
                            ariaLabel="Opciones de la orden"
                        />
                    )}
                </div>
            </div>

            <Modal
                abierto={modalCancelarAbierto}
                onCerrar={() => {
                    if (!cancelando) setModalCancelarAbierto(false);
                }}
                className="ordenDetalleModal"
            >
                <div className="ordenDetalleModalContenido">
                    <h3 className="ordenDetalleModalTitulo">Cancelar orden</h3>
                    <p className="ordenDetalleModalTexto">
                        Esta acción no se puede deshacer. La orden #{order.order_number} quedará cancelada.
                    </p>
                    <div className="ordenDetalleModalAcciones">
                        <Button
                            variante="outline"
                            tamano="pequeno"
                            type="button"
                            onClick={() => setModalCancelarAbierto(false)}
                            disabled={cancelando}
                        >
                            Volver
                        </Button>
                        <Button
                            variante="peligro"
                            tamano="pequeno"
                            type="button"
                            onClick={confirmarCancelacion}
                            disabled={cancelando}
                        >
                            {cancelando ? 'Cancelando...' : 'Sí, cancelar'}
                        </Button>
                    </div>
                </div>
            </Modal>

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

            {shouldShowPhases && (
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
                                onPagarFase={(pn, amount) => setCheckout({amountCents: amount, phaseNumber: pn})}
                            />
                        ))}
                    </div>
                </div>
            )}

            {/* [044A-38 Fase 8] Review panel para órdenes completadas */}
            {order.status === 'completed' && (
                <ReviewPanel orderId={order.id} effectiveRole={effectiveRole} />
            )}
        </div>
    );
};

/* Componente privado: card de una fase dentro del timeline.
 * [044A-38 Fase 6] Integra upload multipart de entregables y listado de archivos. */
function FaseCard({phase, orderId, isLast, isClient, isEmployee, isPhased, onAprobar, onRevision, onPagarFase}: {
    phase: OrderPhaseResponse;
    orderId: string;
    isLast: boolean;
    isClient: boolean;
    isEmployee: boolean;
    isPhased: boolean;
    onAprobar: (orderId: string, phase: number) => Promise<void>;
    onRevision: (orderId: string, phase: number) => Promise<void>;
    onPagarFase: (phaseNumber: number, amountCents: number) => void;
}) {
    const canApprove = isClient && APPROVABLE_PHASE.includes(phase.status);
    const canRevise = isClient && REVISABLE_PHASE.includes(phase.status) && phase.revisions_used < phase.max_revisions;
    const canDeliver = isEmployee && (phase.status === 'in_progress' || phase.status === 'paid' || phase.status === 'revision_requested');
    const canPayPhase = isClient && isPhased && phase.status === 'pending_payment';
    const isActive = phase.status !== 'locked' && phase.status !== 'approved' && phase.status !== 'skipped';

    /* Determinar si la fase tiene o puede tener entregables */
    const hasDeliverableHistory = ['delivered', 'approved', 'revision_requested', 'in_progress']
        .includes(phase.status) && phase.revisions_used > 0;
    const showDeliverables = canDeliver || hasDeliverableHistory;

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

                {/* [044A-38 Fase 6] Panel de entregables: upload + historial */}
                {showDeliverables && (
                    <EntregablesPanel
                        orderId={orderId}
                        phaseNumber={phase.phase_number}
                        canDeliver={canDeliver}
                    />
                )}

                {(canApprove || canRevise || canPayPhase) && (
                    <div className="faseAcciones">
                        {canPayPhase && (
                            <Button
                                className="faseBtn"
                                onClick={() => onPagarFase(phase.phase_number, phase.price_cents)}
                                type="button"
                                variante="exito"
                                tamano="pequeno"
                            >
                                <CreditCard size={14} /> Pagar fase
                            </Button>
                        )}
                        {canApprove && (
                            <Button
                                className="faseBtn"
                                onClick={() => onAprobar(orderId, phase.phase_number)}
                                type="button"
                                variante="exitoSuave"
                                tamano="pequeno"
                            >
                                <Check size={14} /> Aprobar
                            </Button>
                        )}
                        {canRevise && (
                            <Button
                                className="faseBtn"
                                onClick={() => onRevision(orderId, phase.phase_number)}
                                type="button"
                                variante="advertenciaSuave"
                                tamano="pequeno"
                            >
                                <RotateCcw size={14} /> Revisión
                            </Button>
                        )}
                    </div>
                )}
            </div>
        </div>
    );
}

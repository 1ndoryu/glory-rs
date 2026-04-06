/* [064A-30] FaseCard: card de una fase con entregas, revisiones y acciones.
 * Employee ve botón de extensión de tiempo si la fase está en progreso.
 * Cliente puede aprobar o pedir revisión en fases entregadas. */
import {Check, RotateCcw, CreditCard} from 'lucide-react';
import {
    PHASE_STATUS_LABELS,
    formatPrice,
    type OrderPhaseResponse,
    type PhaseStatus,
} from '../../api/orders';
import {EntregablesPanel} from './EntregablesPanel';
import {Button} from '../ui/Button';

const APPROVABLE_PHASE: PhaseStatus[] = ['delivered'];
const REVISABLE_PHASE: PhaseStatus[] = ['delivered'];

interface FaseCardProps {
    phase: OrderPhaseResponse;
    orderId: string;
    isLast: boolean;
    isClient: boolean;
    isEmployee: boolean;
    isPhased: boolean;
    onAprobar: (orderId: string, phase: number) => Promise<void>;
    onRevision: (orderId: string, phase: number) => Promise<void>;
    onPagarFase: (phaseNumber: number, amountCents: number) => void;
}

export function FaseCard({phase, orderId, isLast, isClient, isEmployee, isPhased, onAprobar, onRevision, onPagarFase}: FaseCardProps) {
    const canApprove = isClient && APPROVABLE_PHASE.includes(phase.status);
    const canRevise = isClient && REVISABLE_PHASE.includes(phase.status) && phase.revisions_used < phase.max_revisions;
    const canDeliver = isEmployee && (phase.status === 'in_progress' || phase.status === 'paid' || phase.status === 'revision_requested');
    const canPayPhase = isClient && isPhased && phase.status === 'pending_payment';
    const isActive = phase.status !== 'locked' && phase.status !== 'approved' && phase.status !== 'skipped';

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
                    <h4 className="faseNombre">{phase.title}</h4>
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

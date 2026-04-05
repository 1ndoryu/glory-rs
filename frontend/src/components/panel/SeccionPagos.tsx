/* [044A-38 Fase 3] Sección de historial de pagos en el panel.
 * Muestra pagos por orden con estado, monto y descripción.
 * [044A-38 Fase 7] Botón "Solicitar reembolso" para pagos held/released. */

import { useState } from 'react';
import { Loader2, CreditCard, AlertCircle, RotateCcw } from 'lucide-react';
import { useOrdenes } from '../../hooks/useOrdenes';
import { usePagos } from '../../hooks/usePagos';
import { useRefundModal } from '../../hooks/useRefundModal';
import {
    PAYMENT_STATUS_LABELS,
    PAYMENT_STATUS_CLASS,
} from '../../api/payments';
import { formatPrice } from '../../api/orders';
import { useAuthStore } from '../../stores/authStore';
import { Textarea } from '../ui/Textarea';
import './SeccionPagos.css';

export function SeccionPagos() {
    const { ordenes, cargando, error } = useOrdenes();
    const [ordenSeleccionada, setOrdenSeleccionada] = useState<string | null>(
        null
    );
    const { pagos, cargandoPagos, errorPagos } = usePagos(ordenSeleccionada);
    const {
        refundOrderId, refundRazon, refundEnCurso,
        setRefundRazon, abrirModal, cerrarModal, enviarSolicitud,
    } = useRefundModal();
    const effectiveRole = useAuthStore(s => s.user?.effectiveRole) || 'client';

    if (cargando) {
        return (
            <div className="pagosVacio">
                <Loader2 className="pagosSpinner" size={32} />
            </div>
        );
    }

    if (error) {
        return (
            <div className="pagosError">
                <AlertCircle size={20} />
                <span>{error}</span>
            </div>
        );
    }

    return (
        <div className="pagosContenedor">
            <h2 className="pagosTitulo">Historial de Pagos</h2>

            <div className="pagosOrdenesLista">
                {ordenes.map((o) => (
                    <button
                        key={o.id}
                        className={`pagosOrdenBtn ${
                            ordenSeleccionada === o.id
                                ? 'pagosOrdenBtn--activo'
                                : ''
                        }`}
                        onClick={() => setOrdenSeleccionada(o.id)}
                    >
                        <span className="pagosOrdenNumero">
                            #{o.order_number}
                        </span>
                        <span className="pagosOrdenTitulo">
                            {o.service_title}
                        </span>
                        <span className="pagosOrdenPrecio">
                            {formatPrice(o.final_price_cents, o.currency)}
                        </span>
                    </button>
                ))}
                {ordenes.length === 0 && (
                    <p className="pagosVacioTexto">No tienes órdenes</p>
                )}
            </div>

            {ordenSeleccionada && (
                <div className="pagosDatos">
                    {cargandoPagos && (
                        <Loader2 className="pagosSpinner" size={24} />
                    )}
                    {errorPagos && (
                        <p className="pagosError">
                            <AlertCircle size={16} /> {errorPagos}
                        </p>
                    )}
                    {!cargandoPagos && pagos.length === 0 && (
                        <div className="pagosVacioDetalle">
                            <CreditCard size={32} />
                            <p>No hay pagos registrados para esta orden</p>
                        </div>
                    )}
                    {pagos.map((p) => (
                        <div key={p.id} className="pagoCard">
                            <div className="pagoCardHeader">
                                <span
                                    className={`pagoEstado ${
                                        PAYMENT_STATUS_CLASS[p.status]
                                    }`}
                                >
                                    {PAYMENT_STATUS_LABELS[p.status]}
                                </span>
                                <span className="pagoMonto">
                                    {formatPrice(p.amount_cents, p.currency)}
                                </span>
                            </div>
                            {p.description && (
                                <p className="pagoDescripcion">
                                    {p.description}
                                </p>
                            )}
                            <p className="pagoFecha">
                                {new Date(p.created_at).toLocaleDateString(
                                    'es-ES',
                                    {
                                        day: 'numeric',
                                        month: 'short',
                                        year: 'numeric',
                                    }
                                )}
                            </p>
                            {/* [044A-38 Fase 7] Botón reembolso si el pago es reembolsable */}
                            {effectiveRole === 'client' &&
                                (p.status === 'held' || p.status === 'released') && (
                                <button
                                    className="pagoBotonReembolso"
                                    onClick={() => abrirModal(ordenSeleccionada!)}
                                >
                                    <RotateCcw size={14} />
                                    Solicitar reembolso
                                </button>
                            )}
                        </div>
                    ))}
                </div>
            )}

            {/* [044A-38 Fase 7] Modal de solicitud de reembolso */}
            {refundOrderId && (
                <div className="pagoModalOverlay" onClick={cerrarModal}>
                    <div className="pagoModalContenido" onClick={(e) => e.stopPropagation()}>
                        <h3 className="pagoModalTitulo">Solicitar reembolso</h3>
                        <p className="pagoModalDescripcion">
                            Describe el motivo de tu solicitud de reembolso. Un administrador la revisará.
                        </p>
                        <Textarea
                            className="pagoModalTextarea"
                            placeholder="Motivo del reembolso..."
                            value={refundRazon}
                            onChange={(e) => setRefundRazon(e.target.value)}
                            rows={4}
                        />
                        <div className="pagoModalBotones">
                            <button
                                className="pagoBotonEnviarReembolso"
                                onClick={() => void enviarSolicitud()}
                                disabled={refundEnCurso || !refundRazon.trim()}
                            >
                                {refundEnCurso ? 'Enviando…' : 'Enviar solicitud'}
                            </button>
                            <button
                                className="pagoBotonCancelarReembolso"
                                onClick={cerrarModal}
                                disabled={refundEnCurso}
                            >
                                Cancelar
                            </button>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
}

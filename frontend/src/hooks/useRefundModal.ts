/* [044A-38 Fase 7] Hook para solicitud de reembolso desde SeccionPagos.
 * Encapsula estado del modal y lógica de envío. */

import { useState, useCallback } from 'react';
import { useRefunds } from './useRefunds';

export function useRefundModal() {
    const { solicitarReembolso } = useRefunds();
    const [refundOrderId, setRefundOrderId] = useState<string | null>(null);
    const [refundRazon, setRefundRazon] = useState('');
    const [refundEnCurso, setRefundEnCurso] = useState(false);

    const abrirModal = useCallback((orderId: string) => {
        setRefundOrderId(orderId);
        setRefundRazon('');
    }, []);

    const cerrarModal = useCallback(() => {
        setRefundOrderId(null);
        setRefundRazon('');
    }, []);

    const enviarSolicitud = useCallback(async () => {
        if (!refundOrderId || !refundRazon.trim()) return;
        setRefundEnCurso(true);
        try {
            await solicitarReembolso.mutateAsync({
                orderId: refundOrderId,
                reason: refundRazon.trim(),
            });
            cerrarModal();
        } finally {
            setRefundEnCurso(false);
        }
    }, [refundOrderId, refundRazon, solicitarReembolso, cerrarModal]);

    return {
        refundOrderId,
        refundRazon,
        refundEnCurso,
        setRefundRazon,
        abrirModal,
        cerrarModal,
        enviarSolicitud,
    };
}

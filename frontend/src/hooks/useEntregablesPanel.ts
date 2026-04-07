/* [044A-38 Fase 6] Hook para el panel de entregables.
 * [074A-51] Simplificado: sin upload de archivos. Entrega marca la fase como delivered.
 * Los archivos se comparten por chat. */

import { useState, useCallback } from 'react';
import { useDeliverables } from './useDeliverables';
import type { PhaseDeliverable } from '../api/deliverables';

export function useEntregablesPanel(orderId: string, phaseNumber: number) {
    const {deliverables, cargando, entregarFase, entregando, descargar} = useDeliverables(orderId, phaseNumber);
    const [error, setError] = useState('');

    /* [074A-51] Entregar sin archivos — solo marca la fase como entregada */
    const handleDeliver = useCallback(async () => {
        try {
            setError('');
            await entregarFase({});
        } catch {
            setError('Error al entregar. Intenta de nuevo.');
        }
    }, [entregarFase]);

    const handleDownload = useCallback(async (deliverableId: string, fileName: string) => {
        try {
            const blobUrl = await descargar(deliverableId);
            const a = document.createElement('a');
            a.href = blobUrl;
            a.download = fileName;
            a.click();
            URL.revokeObjectURL(blobUrl);
        } catch {
            setError('Error al descargar archivo');
        }
    }, [descargar]);

    /* Agrupar entregables por revisión (descendente) */
    const revisionGroups = deliverables.reduce<Record<number, PhaseDeliverable[]>>((acc, d) => {
        if (!acc[d.revision_number]) acc[d.revision_number] = [];
        acc[d.revision_number].push(d);
        return acc;
    }, {});
    const sortedRevisions = Object.keys(revisionGroups).map(Number).sort((a, b) => b - a);

    return {
        error,
        cargando,
        entregando,
        handleDeliver,
        handleDownload,
        sortedRevisions,
        revisionGroups,
    };
}

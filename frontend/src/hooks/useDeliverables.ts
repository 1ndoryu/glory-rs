/* [044A-38 Fase 6] Hook para entregables: listar, entregar, descargar.
 * Usa React Query para cache + refetch automático. */

import {useMutation, useQuery, useQueryClient} from '@tanstack/react-query';
import {
    apiDeliverPhase,
    apiDownloadDeliverable,
    apiListDeliverables,
} from '../api/deliverables';

/** Hook para listar entregables de una fase */
export function useDeliverables(orderId: string, phaseNumber: number | null) {
    const queryClient = useQueryClient();

    const {data, isLoading: cargando} = useQuery({
        queryKey: ['deliverables', orderId, phaseNumber],
        queryFn: () => apiListDeliverables(orderId, phaseNumber!),
        enabled: phaseNumber !== null,
    });

    const {mutateAsync: entregarFase, isPending: entregando} = useMutation({
        mutationFn: ({files, notes}: {files: File[]; notes?: string}) =>
            apiDeliverPhase(orderId, phaseNumber!, files, notes),
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: ['deliverables', orderId, phaseNumber]});
            /* También invalidar la orden para que se actualice el status de la fase */
            queryClient.invalidateQueries({queryKey: ['order', orderId]});
        },
    });

    const {mutateAsync: descargar} = useMutation({
        mutationFn: (deliverableId: string) => apiDownloadDeliverable(deliverableId),
    });

    return {
        deliverables: data?.deliverables ?? [],
        approvalStatus: data?.approval_status ?? '',
        revisionsUsed: data?.revisions_used ?? 0,
        maxRevisions: data?.max_revisions ?? 0,
        cargando,
        entregarFase,
        entregando,
        descargar,
    };
}

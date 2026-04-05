/* [044A-38 Fase 7] Hook de reembolsos: React Query para listar + mutations.
 * Cliente: solicitarReembolso. Admin: aprobar/rechazar. Ambos: listar. */

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import {
    apiRequestRefund,
    apiReviewRefund,
    apiListRefunds,
    apiGetOrderRefund,
} from '../api/refunds';
import type { RequestRefundBody, ReviewRefundBody } from '../api/refunds';
import { isAxiosError } from 'axios';

function extraerError(err: unknown): string {
    if (isAxiosError(err) && err.response?.data) {
        const d = err.response.data as Record<string, unknown>;
        if (typeof d.message === 'string') return d.message;
    }
    return err instanceof Error ? err.message : 'Error desconocido';
}

/* Lista global de reembolsos (admin: pendientes, otros: propios) */
export function useRefunds() {
    const queryClient = useQueryClient();

    const { data, isLoading, error } = useQuery({
        queryKey: ['refunds'],
        queryFn: apiListRefunds,
    });

    const solicitarReembolso = useMutation({
        mutationFn: (req: { orderId: string } & RequestRefundBody) =>
            apiRequestRefund(req.orderId, { reason: req.reason }),
        onSuccess: (_data, vars) => {
            void queryClient.invalidateQueries({ queryKey: ['refunds'] });
            void queryClient.invalidateQueries({ queryKey: ['refund-order', vars.orderId] });
            void queryClient.invalidateQueries({ queryKey: ['payments', vars.orderId] });
            void queryClient.invalidateQueries({ queryKey: ['orders'] });
        },
    });

    const revisarReembolso = useMutation({
        mutationFn: (req: { refundId: string } & ReviewRefundBody) =>
            apiReviewRefund(req.refundId, { action: req.action, admin_response: req.admin_response }),
        onSuccess: () => {
            void queryClient.invalidateQueries({ queryKey: ['refunds'] });
            void queryClient.invalidateQueries({ queryKey: ['orders'] });
        },
    });

    return {
        reembolsos: data ?? [],
        cargando: isLoading,
        error: error ? extraerError(error) : null,
        solicitarReembolso,
        revisarReembolso,
    };
}

/* Reembolso de una orden específica (para cliente en SeccionPagos) */
export function useOrderRefund(orderId: string | null) {
    const { data, isLoading, error } = useQuery({
        queryKey: ['refund-order', orderId],
        queryFn: () => apiGetOrderRefund(orderId!),
        enabled: !!orderId,
        retry: false,
    });

    return {
        reembolso: data ?? null,
        cargando: isLoading,
        error: error ? extraerError(error) : null,
    };
}

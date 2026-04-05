/* [044A-38 Fase 8] Hook de reviews: React Query para listar + mutations.
 * Cliente: crear review. Empleado: responder. Admin/Employee: listar. */

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import {
    apiCreateReview,
    apiRespondReview,
    apiListReviews,
    apiGetOrderReview,
} from '../api/reviews';
import type { CreateReviewBody, RespondReviewBody } from '../api/reviews';
import { isAxiosError } from 'axios';

function extraerError(err: unknown): string {
    if (isAxiosError(err) && err.response?.data) {
        const d = err.response.data as Record<string, unknown>;
        if (typeof d.message === 'string') return d.message;
    }
    return err instanceof Error ? err.message : 'Error desconocido';
}

/* Lista global de reviews (admin: todas, employee: propias) */
export function useReviews() {
    const queryClient = useQueryClient();

    const { data, isLoading, error } = useQuery({
        queryKey: ['reviews'],
        queryFn: apiListReviews,
    });

    const crearReview = useMutation({
        mutationFn: (req: { orderId: string } & CreateReviewBody) =>
            apiCreateReview(req.orderId, { rating: req.rating, comment: req.comment }),
        onSuccess: (_data, vars) => {
            void queryClient.invalidateQueries({ queryKey: ['reviews'] });
            void queryClient.invalidateQueries({ queryKey: ['review-order', vars.orderId] });
            void queryClient.invalidateQueries({ queryKey: ['orders'] });
        },
    });

    const responderReview = useMutation({
        mutationFn: (req: { reviewId: string } & RespondReviewBody) =>
            apiRespondReview(req.reviewId, { response: req.response }),
        onSuccess: () => {
            void queryClient.invalidateQueries({ queryKey: ['reviews'] });
        },
    });

    return {
        reviews: data ?? [],
        cargando: isLoading,
        error: error ? extraerError(error) : null,
        crearReview,
        responderReview,
    };
}

/* Review de una orden específica */
export function useOrderReview(orderId: string | null) {
    const { data, isLoading, error } = useQuery({
        queryKey: ['review-order', orderId],
        queryFn: () => apiGetOrderReview(orderId!),
        enabled: !!orderId,
        retry: false,
    });

    return {
        review: data ?? null,
        cargando: isLoading,
        error: error ? extraerError(error) : null,
    };
}

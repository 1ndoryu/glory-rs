/* [044A-38 Fase 7] API client para reembolsos.
 * Conecta con POST /orders/:id/refund, PATCH /refunds/:id, GET /refunds.
 * Tipos alineados con RefundResponse del backend Rust. */
import instance from './axios-instance';

export type RefundStatus = 'requested' | 'under_review' | 'approved' | 'completed' | 'rejected';

export interface RefundResponse {
    id: string;
    order_id: string;
    payment_id: string;
    requested_by: string;
    reviewed_by: string | null;
    amount_cents: number;
    reason: string;
    admin_response: string | null;
    status: RefundStatus;
    stripe_refund_id: string | null;
    requested_at: string;
    reviewed_at: string | null;
    completed_at: string | null;
}

export interface RequestRefundBody {
    reason: string;
}

export type ReviewAction = 'approve' | 'reject';

export interface ReviewRefundBody {
    action: ReviewAction;
    admin_response?: string;
}

export async function apiRequestRefund(
    orderId: string,
    body: RequestRefundBody
): Promise<RefundResponse> {
    const { data } = await instance.post<RefundResponse>(
        `/orders/${orderId}/refund`,
        body
    );
    return data;
}

export async function apiReviewRefund(
    refundId: string,
    body: ReviewRefundBody
): Promise<RefundResponse> {
    const { data } = await instance.patch<RefundResponse>(
        `/refunds/${refundId}`,
        body
    );
    return data;
}

export async function apiListRefunds(): Promise<RefundResponse[]> {
    const { data } = await instance.get<RefundResponse[]>('/refunds');
    return data;
}

export async function apiGetOrderRefund(orderId: string): Promise<RefundResponse> {
    const { data } = await instance.get<RefundResponse>(
        `/orders/${orderId}/refund`
    );
    return data;
}

export const REFUND_STATUS_LABELS: Record<RefundStatus, string> = {
    requested: 'Solicitado',
    under_review: 'En revisión',
    approved: 'Aprobado',
    completed: 'Completado',
    rejected: 'Rechazado',
};

export const REFUND_STATUS_CLASS: Record<RefundStatus, string> = {
    requested: 'reembolsoEstado--solicitado',
    under_review: 'reembolsoEstado--revision',
    approved: 'reembolsoEstado--aprobado',
    completed: 'reembolsoEstado--completado',
    rejected: 'reembolsoEstado--rechazado',
};

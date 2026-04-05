/* [044A-38 Fase 8] API client para reviews de órdenes.
 * Rating 1-5 + comentario, respuesta de empleado.
 * Endpoints: POST /orders/:id/review, POST /reviews/:id/respond, GET /reviews. */
import instance from './axios-instance';

export interface ReviewResponse {
    id: string;
    order_id: string;
    client_id: string;
    employee_id: string;
    rating: number;
    comment: string | null;
    employee_response: string | null;
    employee_responded_at: string | null;
    created_at: string;
}

export interface CreateReviewBody {
    rating: number;
    comment?: string;
}

export interface RespondReviewBody {
    response: string;
}

export async function apiCreateReview(
    orderId: string,
    body: CreateReviewBody
): Promise<ReviewResponse> {
    const { data } = await instance.post<ReviewResponse>(
        `/orders/${orderId}/review`,
        body
    );
    return data;
}

export async function apiRespondReview(
    reviewId: string,
    body: RespondReviewBody
): Promise<ReviewResponse> {
    const { data } = await instance.post<ReviewResponse>(
        `/reviews/${reviewId}/respond`,
        body
    );
    return data;
}

export async function apiGetOrderReview(orderId: string): Promise<ReviewResponse> {
    const { data } = await instance.get<ReviewResponse>(
        `/orders/${orderId}/review`
    );
    return data;
}

export async function apiListReviews(): Promise<ReviewResponse[]> {
    const { data } = await instance.get<ReviewResponse[]>('/reviews');
    return data;
}

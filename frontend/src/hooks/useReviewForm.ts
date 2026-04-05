/* [044A-38 Fase 8] Hook para el formulario de crear review.
 * Encapsula estado del rating/comentario y lógica de envío. */

import { useState, useCallback } from 'react';
import { useReviews } from './useReviews';

export function useReviewForm(orderId: string) {
    const { crearReview } = useReviews();
    const [rating, setRating] = useState(0);
    const [comment, setComment] = useState('');
    const [enviando, setEnviando] = useState(false);

    const enviarReview = useCallback(async () => {
        if (rating < 1 || rating > 5) return;
        setEnviando(true);
        try {
            await crearReview.mutateAsync({
                orderId,
                rating,
                comment: comment.trim() || undefined,
            });
        } finally {
            setEnviando(false);
        }
    }, [orderId, rating, comment, crearReview]);

    return {
        rating,
        setRating,
        comment,
        setComment,
        enviando,
        enviarReview,
    };
}

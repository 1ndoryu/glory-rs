/* [044A-38 Fase 2] Hook para gestión de órdenes en el panel.
 * Usa React Query para fetching + cache + invalidación.
 * Encapsula toda la lógica de estado de órdenes (SRP).
 * Max 3 useState → estado de UI mínimo, el resto en React Query. */
import {useState, useCallback} from 'react';
import {useQuery, useMutation, useQueryClient} from '@tanstack/react-query';
import {
    apiListOrders,
    apiGetOrder,
    apiCreateOrder,
    apiCancelOrder,
    apiApprovePhase,
    apiRequestRevision,
    apiDeliverPhase,
    type OrderResponse,
    type OrderDetailResponse,
    type CreateOrderRequest,
} from '../api/orders';

const ORDERS_KEY = ['orders'] as const;
const orderDetailKey = (id: string) => ['order', id] as const;

export function useOrdenes() {
    const queryClient = useQueryClient();
    const [ordenSeleccionada, setOrdenSeleccionada] = useState<string | null>(null);
    const [error, setError] = useState<string | null>(null);

    /* Lista de órdenes del usuario */
    const {
        data: ordenes = [],
        isLoading: cargando,
        refetch: recargar,
    } = useQuery<OrderResponse[]>({
        queryKey: ORDERS_KEY,
        queryFn: apiListOrders,
    });

    /* Detalle de orden seleccionada */
    const {
        data: detalle,
        isLoading: cargandoDetalle,
    } = useQuery<OrderDetailResponse>({
        queryKey: ordenSeleccionada ? orderDetailKey(ordenSeleccionada) : ['order', 'none'],
        queryFn: () => apiGetOrder(ordenSeleccionada!),
        enabled: !!ordenSeleccionada,
    });

    /* Invalidar cache tras mutaciones */
    const invalidar = useCallback(() => {
        queryClient.invalidateQueries({queryKey: ORDERS_KEY});
        if (ordenSeleccionada) {
            queryClient.invalidateQueries({queryKey: orderDetailKey(ordenSeleccionada)});
        }
    }, [queryClient, ordenSeleccionada]);

    /* Crear orden */
    const crearMutation = useMutation({
        mutationFn: (req: CreateOrderRequest) => apiCreateOrder(req),
        onSuccess: () => {
            invalidar();
            setError(null);
        },
        onError: (err: unknown) => {
            setError(extraerError(err));
        },
    });

    /* Cancelar orden */
    const cancelarMutation = useMutation({
        mutationFn: (orderId: string) => apiCancelOrder(orderId),
        onSuccess: () => {
            invalidar();
            setError(null);
        },
        onError: (err: unknown) => {
            setError(extraerError(err));
        },
    });

    /* Aprobar fase (cliente) */
    const aprobarFaseMutation = useMutation({
        mutationFn: ({orderId, phase}: {orderId: string; phase: number}) =>
            apiApprovePhase(orderId, phase),
        onSuccess: () => {
            invalidar();
            setError(null);
        },
        onError: (err: unknown) => {
            setError(extraerError(err));
        },
    });

    /* Solicitar revisión (cliente) */
    const revisionMutation = useMutation({
        mutationFn: ({orderId, phase}: {orderId: string; phase: number}) =>
            apiRequestRevision(orderId, phase),
        onSuccess: () => {
            invalidar();
            setError(null);
        },
        onError: (err: unknown) => {
            setError(extraerError(err));
        },
    });

    /* Entregar fase (empleado) */
    const entregarFaseMutation = useMutation({
        mutationFn: ({orderId, phase}: {orderId: string; phase: number}) =>
            apiDeliverPhase(orderId, phase),
        onSuccess: () => {
            invalidar();
            setError(null);
        },
        onError: (err: unknown) => {
            setError(extraerError(err));
        },
    });

    return {
        ordenes,
        cargando,
        detalle,
        cargandoDetalle,
        ordenSeleccionada,
        error,
        seleccionarOrden: setOrdenSeleccionada,
        recargar,
        crearOrden: crearMutation.mutateAsync,
        cancelarOrden: cancelarMutation.mutateAsync,
        aprobarFase: aprobarFaseMutation.mutateAsync,
        solicitarRevision: revisionMutation.mutateAsync,
        entregarFase: entregarFaseMutation.mutateAsync,
        creando: crearMutation.isPending,
        cancelando: cancelarMutation.isPending,
    };
}

/* Extrae mensaje de error del response axios */
function extraerError(err: unknown): string {
    if (typeof err === 'object' && err !== null && 'response' in err) {
        const resp = (err as {response?: {data?: {message?: string; error?: string}}}).response;
        if (resp?.data?.message) return resp.data.message;
        if (resp?.data?.error) return resp.data.error;
    }
    return 'Error de conexión';
}

/* [044A-38 Fase 4] Hooks para asignación, delegación y empleados.
 * React Query para fetching + invalidación de cache.
 * useDisponibles: órdenes sin asignar + acción tomar.
 * useDelegaciones: lista + crear + responder delegaciones.
 * useEmpleados: lista de empleados (admin). */
import {useQuery, useMutation, useQueryClient} from '@tanstack/react-query';
import {
    apiListUnassigned,
    apiTakeOrder,
    apiListDelegations,
    apiCreateDelegation,
    apiCreateHelpRequest,
    apiRespondDelegation,
    apiListEmployees,
    type DelegationResponse,
    type EmployeeListItem,
} from '../api/assignment';
import type {OrderResponse} from '../api/orders';

const UNASSIGNED_KEY = ['orders-unassigned'] as const;
const DELEGATIONS_KEY = ['delegations'] as const;
const EMPLOYEES_KEY = ['employees'] as const;

/*    ÓRDENES SIN ASIGNAR (employee + admin) */

export function useDisponibles() {
    const queryClient = useQueryClient();

    const {
        data: disponibles = [],
        isLoading: cargando,
    } = useQuery<OrderResponse[]>({
        queryKey: UNASSIGNED_KEY,
        queryFn: apiListUnassigned,
    });

    const tomarMutation = useMutation({
        mutationFn: (orderId: string) => apiTakeOrder(orderId),
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: UNASSIGNED_KEY});
            queryClient.invalidateQueries({queryKey: ['orders']});
        },
    });

    return {
        disponibles,
        cargando,
        tomarOrden: tomarMutation.mutateAsync,
        tomando: tomarMutation.isPending,
    };
}

/*    DELEGACIONES (employee + admin) */

export function useDelegaciones() {
    const queryClient = useQueryClient();

    const {
        data: delegaciones = [],
        isLoading: cargando,
    } = useQuery<DelegationResponse[]>({
        queryKey: DELEGATIONS_KEY,
        queryFn: apiListDelegations,
    });

    const crearDelegacionMutation = useMutation({
        mutationFn: ({orderId, reason}: {orderId: string; reason: string}) =>
            apiCreateDelegation(orderId, {reason}),
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: DELEGATIONS_KEY});
        },
    });

    const pedirAyudaMutation = useMutation({
        mutationFn: ({orderId, reason}: {orderId: string; reason: string}) =>
            apiCreateHelpRequest(orderId, {reason}),
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: DELEGATIONS_KEY});
        },
    });

    const responderMutation = useMutation({
        mutationFn: ({delegationId, accept}: {delegationId: string; accept: boolean}) =>
            apiRespondDelegation(delegationId, accept),
        onSuccess: () => {
            queryClient.invalidateQueries({queryKey: DELEGATIONS_KEY});
            queryClient.invalidateQueries({queryKey: UNASSIGNED_KEY});
            queryClient.invalidateQueries({queryKey: ['orders']});
        },
    });

    return {
        delegaciones,
        cargando,
        crearDelegacion: crearDelegacionMutation.mutateAsync,
        pedirAyuda: pedirAyudaMutation.mutateAsync,
        responder: responderMutation.mutateAsync,
        creandoDelegacion: crearDelegacionMutation.isPending,
        respondiendo: responderMutation.isPending,
    };
}

/*    EMPLEADOS (admin) */

export function useEmpleados() {
    const {
        data: empleados = [],
        isLoading: cargando,
    } = useQuery<EmployeeListItem[]>({
        queryKey: EMPLOYEES_KEY,
        queryFn: apiListEmployees,
    });

    return {empleados, cargando};
}

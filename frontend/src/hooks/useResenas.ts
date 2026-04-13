/* [134A-3] Hook para gestión de reseñas del restaurante.
 * Listado admin paginado + solicitar reseña a un cliente. */

import { useState } from 'react';
import { toast } from 'sonner';
import {
  useListarResenas,
  useSolicitarResena,
} from '../api/generated/reseñas/reseñas';
import type { ResenaAdmin } from '../api/generated/gestionRestauranteAPI.schemas';

function extraerError(err: unknown): string {
  const e = err as { response?: { data?: { message?: string } } };
  return e?.response?.data?.message || 'Error inesperado';
}

export function useResenas() {
  const [page, setPage] = useState(1);

  const { data, isLoading, refetch } = useListarResenas(
    { page, per_page: 20 },
    { query: { queryKey: ['resenas', page] } }
  );

  const solicitarMut = useSolicitarResena({
    mutation: {
      onSuccess: () => {
        toast.success('Reseña solicitada — enlace generado');
        refetch();
      },
      onError: (err: unknown) => toast.error(extraerError(err)),
    },
  });

  const resenas: ResenaAdmin[] = data?.data?.data ?? [];
  const total = data?.data?.total ?? 0;
  const totalPages = Math.ceil(total / 20);

  return {
    resenas,
    total,
    page,
    totalPages,
    isLoading,
    setPage,
    solicitarResena: (reservaId?: string, clienteId?: string) =>
      solicitarMut.mutate({ params: { reserva_id: reservaId, cliente_id: clienteId } }),
    solicitando: solicitarMut.isPending,
    refetch,
  };
}

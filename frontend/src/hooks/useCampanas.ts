/* [263A-23] Hook para gestión de campañas de marketing.
 * Maneja creación, envío y navegación a formulario.
 * [024A-9] Toast de error con mensaje del backend al enviar campaña. */

import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { toast } from 'sonner';
import {
  useListarCampanas,
  useCrearCampana,
  useEliminarCampana,
  useEnviarCampana,
} from '../api/generated';
import type { Campana } from '../api/generated/gestionRestauranteAPI.schemas';

/* [024A-9] Extrae el mensaje de error del backend */
function extraerMensajeError(err: unknown): string {
  const axiosErr = err as { response?: { data?: { message?: string } } };
  return axiosErr?.response?.data?.message || 'Error al enviar campaña';
}

export function useCampanas() {
  const [page, setPage] = useState(1);
  const [filtroEstado, setFiltroEstado] = useState('');
  const navigate = useNavigate();

  const { data, isLoading, refetch } = useListarCampanas(
    { page, per_page: 20, estado: filtroEstado || undefined },
    { query: { queryKey: ['campanas', page, filtroEstado] } }
  );

  const crearMutation = useCrearCampana({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const eliminarMutation = useEliminarCampana({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const enviarMutation = useEnviarCampana({
    mutation: {
      onSuccess: () => {
        toast.success('Campaña enviada');
        refetch();
      },
      onError: (err: unknown) => {
        toast.error(extraerMensajeError(err));
      },
    },
  });

  const campanas: Campana[] = data?.data?.items ?? [];
  const total = data?.data?.total ?? 0;
  const totalPages = Math.ceil(total / 20);

  return {
    campanas,
    total,
    page,
    totalPages,
    filtroEstado,
    isLoading,
    setPage,
    setFiltroEstado,
    crearCampana: crearMutation.mutateAsync,
    eliminarCampana: eliminarMutation.mutateAsync,
    /* [024A-9] mutate en vez de mutateAsync para que onError maneje sin exception */
    enviarCampana: enviarMutation.mutate,
    irANuevaCampana: () => navigate('/marketing/campanas/nueva'),
    refetch,
  };
}

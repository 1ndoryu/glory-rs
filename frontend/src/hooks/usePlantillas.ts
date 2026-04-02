/* [263A-24] Hook para gestión de plantillas WhatsApp.
 * Maneja paginación, filtro por estado, CRUD y envío a Meta.
 * [024A-6] Toast de error con mensaje del backend al enviar a Meta. */

import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { toast } from 'sonner';
import {
  useListarPlantillas,
  useCrearPlantilla,
  useEliminarPlantilla,
  useEnviarAMeta,
} from '../api/generated';

/* [024A-6] Extrae el mensaje de error del backend desde la respuesta Axios/fetch */
function extraerMensajeError(err: unknown): string {
  const axiosErr = err as { response?: { data?: { message?: string } } };
  return axiosErr?.response?.data?.message || 'Error desconocido';
}

export function usePlantillas() {
  const [page, setPage] = useState(1);
  const [filtroEstado, setFiltroEstado] = useState('');
  const navigate = useNavigate();

  const { data, isLoading, refetch } = useListarPlantillas(
    { page, per_page: 20, estado: filtroEstado || undefined },
    { query: { queryKey: ['plantillas-wa', page, filtroEstado] } },
  );

  const crearMutation = useCrearPlantilla({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const eliminarMutation = useEliminarPlantilla({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const enviarMutation = useEnviarAMeta({
    mutation: {
      onSuccess: () => {
        toast.success('Plantilla enviada a Meta para aprobación');
        refetch();
      },
      onError: (err: unknown) => {
        toast.error(extraerMensajeError(err));
      },
    },
  });

  const plantillas = data?.data?.items ?? [];
  const total = data?.data?.total ?? 0;
  const totalPages = Math.ceil(total / 20);

  return {
    plantillas,
    total,
    page,
    totalPages,
    filtroEstado,
    isLoading,
    setPage,
    setFiltroEstado,
    crearPlantilla: crearMutation.mutateAsync,
    eliminarPlantilla: eliminarMutation.mutateAsync,
    /* [024A-6] Usar mutate (no mutateAsync) para que errors sean manejados por onError sin propagarse */
    enviarAMeta: enviarMutation.mutate,
    enviandoAMeta: enviarMutation.isPending,
    irANuevaPlantilla: () => navigate('/marketing/plantillas/nueva'),
    refetch,
  };
}

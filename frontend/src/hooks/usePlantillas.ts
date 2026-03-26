/* [263A-24] Hook para gestión de plantillas WhatsApp.
 * Maneja paginación, filtro por estado, CRUD y envío a Meta. */

import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  useListarPlantillas,
  useCrearPlantilla,
  useEliminarPlantilla,
  useEnviarAMeta,
} from '../api/generated';

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
    mutation: { onSuccess: () => { refetch(); } },
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
    enviarAMeta: enviarMutation.mutateAsync,
    irANuevaPlantilla: () => navigate('/marketing/plantillas/nueva'),
    refetch,
  };
}

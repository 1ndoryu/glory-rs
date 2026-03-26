/* [263A-23] Hook para gestión de campañas de marketing.
 * Maneja creación, envío y navegación a formulario. */

import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  useListarCampanas,
  useCrearCampana,
  useEliminarCampana,
  useEnviarCampana,
} from '../api/generated';

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
    mutation: { onSuccess: () => { refetch(); } },
  });

  const campanas = data?.data?.items ?? [];
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
    enviarCampana: enviarMutation.mutateAsync,
    irANuevaCampana: () => navigate('/marketing/campanas/nueva'),
    refetch,
  };
}

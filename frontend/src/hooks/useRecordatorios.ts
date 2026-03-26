/* [263A-25] Hook para gestión de reglas de recordatorio.
 * Maneja CRUD de reglas, toggle activa/inactiva, y paginación. */

import { useState } from 'react';
import {
  useListarReglas,
  useCrearRegla,
  useActualizarRegla,
  useEliminarRegla,
} from '../api/generated';

export function useRecordatorios() {
  const [page, setPage] = useState(1);

  const { data, isLoading, refetch } = useListarReglas(
    { page, per_page: 20 },
    { query: { queryKey: ['recordatorios-reglas', page] } },
  );

  const crearMutation = useCrearRegla({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const actualizarMutation = useActualizarRegla({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const eliminarMutation = useEliminarRegla({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const reglas = data?.data?.items ?? [];
  const total = data?.data?.total ?? 0;
  const totalPages = Math.ceil(total / 20);

  const toggleActiva = async (id: string, activa: boolean) => {
    await actualizarMutation.mutateAsync({ id, data: { activa: !activa } });
  };

  return {
    reglas,
    total,
    page,
    totalPages,
    isLoading,
    setPage,
    crearRegla: crearMutation.mutateAsync,
    actualizarRegla: actualizarMutation.mutateAsync,
    eliminarRegla: eliminarMutation.mutateAsync,
    toggleActiva,
    refetch,
  };
}

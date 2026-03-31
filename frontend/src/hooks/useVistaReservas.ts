/* 263A-6: Hook para vista de reservas por día.
   Maneja filtros (fecha, turno, estado), paginación y modal.
   Lee ?fecha= de la URL cuando se navega desde el calendario.
   303A-15: Soporte rango de fechas (fecha_desde/fecha_hasta). */

import { useState, useCallback } from 'react';
import { useSearchParams } from 'react-router-dom';
import { useQueryClient } from '@tanstack/react-query';
import { useListarReservas, useEliminarReserva, getObtenerOcupacionQueryKey } from '../api/generated';

interface FiltrosReservas {
  fecha: string;
  fechaHasta: string;
  turno: string;
  estado: string;
  busqueda: string;
  pagina: number;
}

const POR_PAGINA = 20;

function useVistaReservas() {
  const queryClient = useQueryClient();
  const [searchParams] = useSearchParams();
  const fechaUrl = searchParams.get('fecha');

  const [filtros, setFiltros] = useState<FiltrosReservas>({
    fecha: fechaUrl || new Date().toISOString().split('T')[0],
    fechaHasta: '',
    turno: '',
    estado: '',
    busqueda: '',
    pagina: 1,
  });
  const [modalAbierto, setModalAbierto] = useState(false);

  /* [303A-15] Si fechaHasta tiene valor, se usa rango (fecha_desde/fecha_hasta).
   * Si no, se usa fecha exacta (compatibilidad con vista día). */
  const usaRango = !!filtros.fechaHasta;

  const { data, isLoading, refetch } = useListarReservas({
    page: filtros.pagina,
    per_page: POR_PAGINA,
    fecha: usaRango ? undefined : (filtros.fecha || undefined),
    fecha_desde: usaRango ? (filtros.fecha || undefined) : undefined,
    fecha_hasta: usaRango ? (filtros.fechaHasta || undefined) : undefined,
    estado: filtros.estado || undefined,
    turno: filtros.turno || undefined,
    busqueda: filtros.busqueda || undefined,
  });

  const eliminarMutation = useEliminarReserva({
    mutation: {
      onSuccess: () => {
        refetch();
        /* [313A-8] Invalidar plano de ocupación al eliminar reserva */
        queryClient.invalidateQueries({ queryKey: getObtenerOcupacionQueryKey() });
      },
    },
  });

  const reservas = data?.status === 200 ? data.data : null;

  const cambiarFiltro = useCallback(<K extends keyof FiltrosReservas>(
    campo: K,
    valor: FiltrosReservas[K],
  ) => {
    setFiltros(prev => ({ ...prev, [campo]: valor, pagina: campo === 'pagina' ? valor as number : 1 }));
  }, []);

  /* [313A-8] Al cerrar modal de nueva reserva, refrescar lista Y plano de ocupación.
   * Sin invalidar ocupación, las mesas no reflejaban la reserva recién creada. */
  const cerrarModalYRefrescar = useCallback(() => {
    setModalAbierto(false);
    refetch();
    queryClient.invalidateQueries({ queryKey: getObtenerOcupacionQueryKey() });
  }, [refetch, queryClient]);

  return {
    filtros,
    cambiarFiltro,
    modalAbierto,
    setModalAbierto,
    reservas,
    isLoading,
    refetch,
    eliminarMutation,
    cerrarModalYRefrescar,
    porPagina: POR_PAGINA,
  };
}

export default useVistaReservas;

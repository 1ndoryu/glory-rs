/* 263A-6: Hook para vista de reservas por día.
   Maneja filtros (fecha, turno, estado), paginación y modal.
   Lee ?fecha= de la URL cuando se navega desde el calendario.
   303A-15: Soporte rango de fechas (fecha_desde/fecha_hasta).
   [024A-2] Actualizar estado inline desde la lista.
   [024A-3] Invalidar TODAS las queries de reservas tras mutaciones (no solo la actual).
   [024A-5] Modal de edición con reserva seleccionada. */

import { useState, useCallback } from 'react';
import { useSearchParams } from 'react-router-dom';
import { useQueryClient } from '@tanstack/react-query';
import {
  useListarReservas,
  useEliminarReserva,
  useActualizarReserva,
  getObtenerOcupacionQueryKey,
  getListarReservasQueryKey,
  Reserva,
} from '../api/generated';

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
  /* [024A-5] Reserva seleccionada para edición en modal */
  const [reservaEditando, setReservaEditando] = useState<Reserva | null>(null);

  /* [303A-15] Si fechaHasta tiene valor, se usa rango (fecha_desde/fecha_hasta).
   * Si no, se usa fecha exacta (compatibilidad con vista día). */
  const usaRango = !!filtros.fechaHasta;

  const { data, isLoading } = useListarReservas({
    page: filtros.pagina,
    per_page: POR_PAGINA,
    fecha: usaRango ? undefined : (filtros.fecha || undefined),
    fecha_desde: usaRango ? (filtros.fecha || undefined) : undefined,
    fecha_hasta: usaRango ? (filtros.fechaHasta || undefined) : undefined,
    estado: filtros.estado || undefined,
    turno: filtros.turno || undefined,
    busqueda: filtros.busqueda || undefined,
  });

  /* [024A-3] Invalidar TODAS las variantes de queries de reservas y ocupación.
   * Antes solo se usaba refetch() que refrescaba la query actual, dejando stale
   * las demás combinaciones de filtros (ej: cambiar de cena → día completo). */
  const invalidarReservas = useCallback(() => {
    queryClient.invalidateQueries({ queryKey: getListarReservasQueryKey() });
    queryClient.invalidateQueries({ queryKey: getObtenerOcupacionQueryKey() });
  }, [queryClient]);

  const eliminarMutation = useEliminarReserva({
    mutation: {
      onSuccess: () => {
        /* [024A-3] Usar invalidación amplia en vez de refetch puntual */
        invalidarReservas();
      },
    },
  });

  /* [024A-2] Mutation para cambiar estado inline desde la lista */
  const actualizarMutation = useActualizarReserva({
    mutation: {
      onSuccess: () => {
        invalidarReservas();
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
   * [024A-3] Usar invalidación amplia para todas las variantes de query. */
  const cerrarModalYRefrescar = useCallback(() => {
    setModalAbierto(false);
    setReservaEditando(null);
    invalidarReservas();
  }, [invalidarReservas]);

  /* [024A-5] Abrir modal de edición con la reserva seleccionada */
  const abrirEdicion = useCallback((reserva: Reserva) => {
    setReservaEditando(reserva);
    setModalAbierto(true);
  }, []);

  return {
    filtros,
    cambiarFiltro,
    modalAbierto,
    setModalAbierto,
    reservas,
    isLoading,
    eliminarMutation,
    actualizarMutation,
    cerrarModalYRefrescar,
    porPagina: POR_PAGINA,
    reservaEditando,
    abrirEdicion,
  };
}

export default useVistaReservas;

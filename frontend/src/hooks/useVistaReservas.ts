/* 263A-6: Hook para vista de reservas por día.
   Maneja filtros (fecha, turno, estado), paginación y modal.
   Lee ?fecha= de la URL cuando se navega desde el calendario.
   303A-15: Soporte rango de fechas (fecha_desde/fecha_hasta). */

import { useState, useCallback } from 'react';
import { useSearchParams } from 'react-router-dom';
import { useListarReservas, useEliminarReserva } from '../api/generated';

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
    mutation: { onSuccess: () => { refetch(); } },
  });

  const reservas = data?.status === 200 ? data.data : null;

  const cambiarFiltro = useCallback(<K extends keyof FiltrosReservas>(
    campo: K,
    valor: FiltrosReservas[K],
  ) => {
    setFiltros(prev => ({ ...prev, [campo]: valor, pagina: campo === 'pagina' ? valor as number : 1 }));
  }, []);

  const cerrarModalYRefrescar = useCallback(() => {
    setModalAbierto(false);
    refetch();
  }, [refetch]);

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

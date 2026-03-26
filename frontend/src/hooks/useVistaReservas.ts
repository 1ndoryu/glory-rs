/* 263A-6: Hook para vista de reservas por día.
   Maneja filtros (fecha, turno, estado), paginación y modal.
   Lee ?fecha= de la URL cuando se navega desde el calendario. */

import { useState, useCallback } from 'react';
import { useSearchParams } from 'react-router-dom';
import { useListarReservas, useEliminarReserva } from '../api/generated';

interface FiltrosReservas {
  fecha: string;
  turno: string;
  estado: string;
  pagina: number;
}

const POR_PAGINA = 20;

function useVistaReservas() {
  const [searchParams] = useSearchParams();
  const fechaUrl = searchParams.get('fecha');

  const [filtros, setFiltros] = useState<FiltrosReservas>({
    fecha: fechaUrl || new Date().toISOString().split('T')[0],
    turno: '',
    estado: '',
    pagina: 1,
  });
  const [modalAbierto, setModalAbierto] = useState(false);

  const { data, isLoading, refetch } = useListarReservas({
    page: filtros.pagina,
    per_page: POR_PAGINA,
    fecha: filtros.fecha || null,
    estado: filtros.estado || null,
    turno: filtros.turno || null,
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

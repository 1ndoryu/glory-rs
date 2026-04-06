/* [283A-34] Hook para ListaGastos — filtros de fecha (desde/hasta) y paginación.
 * Reduce useState count en el componente.
 * [044A-8+9] Búsqueda y ordenamiento por columnas (sort_by/sort_order).
 * [064A-3] Filtros por columna: tipo_documento, metodo_pago (multi-valor). */

import { useState } from 'react';
import { useListarGastos, useEliminarGasto, Gasto } from '../api/generated';

interface FiltrosGastos {
  pagina: number;
  desde: string;
  hasta: string;
  busqueda: string;
  tipoDocumento: string[];
  metodoPago: string[];
  sortBy: string;
  sortOrder: 'asc' | 'desc';
}

const POR_PAGINA = 15;

function useListaGastos() {
  const [filtros, setFiltros] = useState<FiltrosGastos>({
    pagina: 1,
    desde: '',
    hasta: '',
    busqueda: '',
    tipoDocumento: [],
    metodoPago: [],
    sortBy: '',
    sortOrder: 'desc',
  });
  const [modalAbierto, setModalAbierto] = useState(false);
  const [gastoEditando, setGastoEditando] = useState<Gasto | null>(null);

  const { data, isLoading, refetch } = useListarGastos({
    page: filtros.pagina,
    per_page: POR_PAGINA,
    desde: filtros.desde || undefined,
    hasta: filtros.hasta || undefined,
    busqueda: filtros.busqueda || undefined,
    tipo_documento: filtros.tipoDocumento.length > 0 ? filtros.tipoDocumento.join(',') : undefined,
    metodo_pago: filtros.metodoPago.length > 0 ? filtros.metodoPago.join(',') : undefined,
    sort_by: filtros.sortBy || undefined,
    sort_order: filtros.sortOrder || undefined,
  });

  const eliminarMutation = useEliminarGasto({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const gastos = data?.status === 200 ? data.data : null;

  const cambiarFiltro = <K extends keyof FiltrosGastos>(
    campo: K,
    valor: FiltrosGastos[K],
  ) => {
    setFiltros(prev => ({
      ...prev,
      [campo]: valor,
      pagina: campo === 'pagina' ? valor as number : 1,
    }));
  };

  const cerrarModalYRefrescar = () => {
    setModalAbierto(false);
    refetch();
  };

  const cerrarEdicionYRefrescar = () => {
    setGastoEditando(null);
    refetch();
  };

  /* [044A-8] Alterna ordenamiento por columna */
  const toggleSort = (columna: string) => {
    setFiltros(prev => ({
      ...prev,
      sortBy: columna,
      sortOrder: prev.sortBy === columna && prev.sortOrder === 'asc' ? 'desc' : 'asc',
      pagina: 1,
    }));
  };

  /* [064A-3] Actualizar filtro de columna (array de valores seleccionados) */
  const cambiarFiltroColumna = (campo: 'tipoDocumento' | 'metodoPago', valores: string[]) => {
    setFiltros(prev => ({ ...prev, [campo]: valores, pagina: 1 }));
  };

  return {
    filtros,
    cambiarFiltro,
    toggleSort,
    cambiarFiltroColumna,
    modalAbierto,
    setModalAbierto,
    gastoEditando,
    setGastoEditando,
    porPagina: POR_PAGINA,
    gastos,
    isLoading,
    eliminarMutation,
    cerrarModalYRefrescar,
    cerrarEdicionYRefrescar,
  };
}

export default useListaGastos;

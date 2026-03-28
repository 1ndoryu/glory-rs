/* [283A-34] Hook para ListaGastos — filtros de fecha (desde/hasta) y paginación.
 * Reduce useState count en el componente. */

import { useState } from 'react';
import { useListarGastos, useEliminarGasto, Gasto } from '../api/generated';

interface FiltrosGastos {
  pagina: number;
  desde: string;
  hasta: string;
}

const POR_PAGINA = 15;

function useListaGastos() {
  const [filtros, setFiltros] = useState<FiltrosGastos>({
    pagina: 1,
    desde: '',
    hasta: '',
  });
  const [modalAbierto, setModalAbierto] = useState(false);
  const [gastoEditando, setGastoEditando] = useState<Gasto | null>(null);

  const { data, isLoading, refetch } = useListarGastos({
    page: filtros.pagina,
    per_page: POR_PAGINA,
    desde: filtros.desde || undefined,
    hasta: filtros.hasta || undefined,
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

  return {
    filtros,
    cambiarFiltro,
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

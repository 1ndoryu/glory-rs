/* [283A-28] Hook para ListaVentas — reduce useState count en el componente.
 * Maneja paginación, filtros de fecha (desde/hasta) y modales. */

import { useState } from 'react';
import { useListarVentas, useEliminarVenta, Venta } from '../api/generated';

interface FiltrosVentas {
  pagina: number;
  desde: string;
  hasta: string;
}

const POR_PAGINA = 15;

function useListaVentas() {
  const [filtros, setFiltros] = useState<FiltrosVentas>({
    pagina: 1,
    desde: '',
    hasta: '',
  });
  const [modalAbierto, setModalAbierto] = useState(false);
  const [ventaEditando, setVentaEditando] = useState<Venta | null>(null);

  const { data, isLoading, refetch } = useListarVentas({
    page: filtros.pagina,
    per_page: POR_PAGINA,
    desde: filtros.desde || undefined,
    hasta: filtros.hasta || undefined,
  });

  const eliminarMutation = useEliminarVenta({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const ventas = data?.status === 200 ? data.data : null;

  const cambiarFiltro = <K extends keyof FiltrosVentas>(
    campo: K,
    valor: FiltrosVentas[K],
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
    setVentaEditando(null);
    refetch();
  };

  return {
    filtros,
    cambiarFiltro,
    modalAbierto,
    setModalAbierto,
    ventaEditando,
    setVentaEditando,
    porPagina: POR_PAGINA,
    ventas,
    isLoading,
    eliminarMutation,
    cerrarModalYRefrescar,
    cerrarEdicionYRefrescar,
  };
}

export default useListaVentas;

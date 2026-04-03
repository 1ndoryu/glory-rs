/* [283A-28] Hook para ListaVentas — reduce useState count en el componente.
 * Maneja paginación, filtros de fecha (desde/hasta) y modales.
 * [034A-5] VentaConCliente con nombre_cliente + visor de reserva asociada. */

import { useState } from 'react';
import { useListarVentas, useEliminarVenta, useObtenerReserva, VentaConCliente } from '../api/generated';

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
  const [ventaEditando, setVentaEditando] = useState<VentaConCliente | null>(null);
  /* [034A-5] ID de reserva para el diálogo de detalle */
  const [reservaIdViewer, setReservaIdViewer] = useState<string | null>(null);

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

  /* [034A-5] Obtener detalle de reserva asociada (solo cuando se abre el viewer) */
  const { data: reservaData, isLoading: reservaCargando } = useObtenerReserva(reservaIdViewer ?? '', {
    query: { enabled: !!reservaIdViewer },
  });
  const reservaDetalle = reservaData?.status === 200 ? reservaData.data : null;

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
    reservaIdViewer,
    setReservaIdViewer,
    reservaDetalle,
    reservaCargando,
  };
}

export default useListaVentas;

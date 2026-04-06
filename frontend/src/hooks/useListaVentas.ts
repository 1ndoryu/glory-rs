/* [283A-28] Hook para ListaVentas — reduce useState count en el componente.
 * Maneja paginación, filtros de fecha (desde/hasta) y modales.
 * [034A-5] VentaConCliente con nombre_cliente + visor de reserva asociada.
 * [044A-8+9] Búsqueda y ordenamiento por columnas (sort_by/sort_order).
 * [064A-3] Filtros por columna: turno, canal, metodo_pago (multi-valor). */

import { useState } from 'react';
import { useListarVentas, useEliminarVenta, useObtenerReserva, VentaConCliente } from '../api/generated';

interface FiltrosVentas {
  pagina: number;
  desde: string;
  hasta: string;
  busqueda: string;
  turno: string[];
  canal: string[];
  metodoPago: string[];
  sortBy: string;
  sortOrder: 'asc' | 'desc';
}

const POR_PAGINA = 15;

function useListaVentas() {
  const [filtros, setFiltros] = useState<FiltrosVentas>({
    pagina: 1,
    desde: '',
    hasta: '',
    busqueda: '',
    turno: [],
    canal: [],
    metodoPago: [],
    sortBy: '',
    sortOrder: 'desc',
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
    busqueda: filtros.busqueda || undefined,
    turno: filtros.turno.length > 0 ? filtros.turno.join(',') : undefined,
    canal: filtros.canal.length > 0 ? filtros.canal.join(',') : undefined,
    metodo_pago: filtros.metodoPago.length > 0 ? filtros.metodoPago.join(',') : undefined,
    sort_by: filtros.sortBy || undefined,
    sort_order: filtros.sortOrder || undefined,
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

  /* [044A-8] Alterna ordenamiento por columna — click en la misma columna invierte dirección */
  const toggleSort = (columna: string) => {
    setFiltros(prev => ({
      ...prev,
      sortBy: columna,
      sortOrder: prev.sortBy === columna && prev.sortOrder === 'asc' ? 'desc' : 'asc',
      pagina: 1,
    }));
  };

  /* [064A-3] Actualizar filtro de columna (array de valores seleccionados) */
  const cambiarFiltroColumna = (campo: 'turno' | 'canal' | 'metodoPago', valores: string[]) => {
    setFiltros(prev => ({ ...prev, [campo]: valores, pagina: 1 }));
  };

  return {
    filtros,
    cambiarFiltro,
    toggleSort,
    cambiarFiltroColumna,
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

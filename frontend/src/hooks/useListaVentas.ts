/* [283A-28] Hook para ListaVentas — reduce useState count en el componente.
 * Maneja paginación, filtros de fecha (desde/hasta) y modales.
 * [034A-5] VentaConCliente con nombre_cliente + visor de reserva asociada.
 * [044A-8+9] Búsqueda y ordenamiento por columnas (sort_by/sort_order).
 * [064A-3] Filtros por columna: turno, canal, metodo_pago (multi-valor).
 * [064A-8] Bloqueo de eliminación: expone haddockSyncEnabled, toast 409. */

import { useState } from 'react';
import { toast } from 'sonner';
import { useListarVentas, useEliminarVenta, useObtenerReserva, useReintentarSyncHaddock, VentaConCliente } from '../api/generated';
import { useObtenerConfiguracion } from '../api/generated/configuracion/configuracion';

interface FiltrosVentas {
  pagina: number;
  desde: string;
  hasta: string;
  busqueda: string;
  turno: string[];
  canal: string[];
  metodoPago: string[];
  estadoHaddock: string[];
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
    estadoHaddock: [],
    sortBy: '',
    sortOrder: 'desc',
  });
  const [modalAbierto, setModalAbierto] = useState(false);
  const [ventaEditando, setVentaEditando] = useState<VentaConCliente | null>(null);
  /* [064A-9] Venta pendiente de confirmación antes de editar (cuando ya está sincronizada) */
  const [ventaPendienteEdicion, setVentaPendienteEdicion] = useState<VentaConCliente | null>(null);
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
    estado_haddock: filtros.estadoHaddock.length > 0 ? filtros.estadoHaddock.join(',') : undefined,
    sort_by: filtros.sortBy || undefined,
    sort_order: filtros.sortOrder || undefined,
  });

  const eliminarMutation = useEliminarVenta({
    mutation: {
      onSuccess: () => { refetch(); },
      /* [064A-8] Manejo de error 409 — Haddock sync activo bloquea eliminación */
      onError: (err: unknown) => {
        const status = (err as { status?: number })?.status
          ?? (err as { response?: { status?: number } })?.response?.status;
        if (status === 409) {
          toast.error('Eliminación bloqueada', {
            description: 'La sincronización con Haddock está activa. Desactívela en Configuración para poder eliminar ventas.',
          });
        } else {
          toast.error('Error al eliminar la venta');
        }
      },
    },
  });

  /* [064A-8] Leer config para saber si sync Haddock está habilitado */
  const { data: configData } = useObtenerConfiguracion();
  const haddockSyncEnabled = configData?.status === 200
    ? configData.data.haddock_sync_enabled
    : false;

  /* [064A-10] Retry manual de sincronización Haddock */
  const retryHaddockMutation = useReintentarSyncHaddock({
    mutation: {
      onSuccess: () => {
        toast.success('Sincronización Haddock completada');
        refetch();
      },
      onError: () => {
        toast.error('Error al reintentar sincronización Haddock');
        refetch();
      },
    },
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

  /* [064A-9] Inicia edición de venta. Si está sincronizada con Haddock,
   * muestra diálogo de confirmación antes de abrir el formulario. */
  const iniciarEdicion = (venta: VentaConCliente) => {
    if (haddockSyncEnabled && venta.haddock_synced) {
      setVentaPendienteEdicion(venta);
    } else {
      setVentaEditando(venta);
    }
  };

  const confirmarEdicion = () => {
    if (ventaPendienteEdicion) {
      setVentaEditando(ventaPendienteEdicion);
      setVentaPendienteEdicion(null);
    }
  };

  const cancelarEdicion = () => {
    setVentaPendienteEdicion(null);
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

  /* [064A-3] Actualizar filtro de columna (array de valores seleccionados)
   * [064A-12] Añadido estadoHaddock como filtro de columna. */
  const cambiarFiltroColumna = (campo: 'turno' | 'canal' | 'metodoPago' | 'estadoHaddock', valores: string[]) => {
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
    ventaPendienteEdicion,
    iniciarEdicion,
    confirmarEdicion,
    cancelarEdicion,
    porPagina: POR_PAGINA,
    ventas,
    isLoading,
    eliminarMutation,
    haddockSyncEnabled,
    retryHaddockMutation,
    cerrarModalYRefrescar,
    cerrarEdicionYRefrescar,
    reservaIdViewer,
    setReservaIdViewer,
    reservaDetalle,
    reservaCargando,
  };
}

export default useListaVentas;

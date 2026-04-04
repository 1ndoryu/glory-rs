/* 263A-1: Hook para ListaClientes — reduce useState count en el componente.
 * [263A-26] Agregado: selección múltiple para merge de clientes duplicados.
 * [044A-8] Ordenamiento por columna (sort_by/sort_order). */

import { useState } from 'react';
import { useListarClientes, useEliminarCliente, useMergeClientes, Cliente } from '../api/generated';

function useListaClientes() {
  const [pagina, setPagina] = useState(1);
  const [busqueda, setBusqueda] = useState('');
  const [sortBy, setSortBy] = useState('');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('asc');
  const [modalCrear, setModalCrear] = useState(false);
  const [clienteEditar, setClienteEditar] = useState<Cliente | null>(null);
  const [seleccionados, setSeleccionados] = useState<string[]>([]);
  const [modalMerge, setModalMerge] = useState(false);
  const porPagina = 25;

  const { data, isLoading, refetch } = useListarClientes({
    page: pagina,
    per_page: porPagina,
    busqueda: busqueda || undefined,
    sort_by: sortBy || undefined,
    sort_order: sortOrder || undefined,
  });

  const eliminarMut = useEliminarCliente({
    mutation: { onSuccess: () => { refetch(); } },
  });

  const mergeMut = useMergeClientes({
    mutation: {
      onSuccess: () => {
        setSeleccionados([]);
        setModalMerge(false);
        refetch();
      },
    },
  });

  const clientes = data?.status === 200 ? data.data : null;

  const cerrarModalYRefrescar = () => {
    setModalCrear(false);
    setClienteEditar(null);
    refetch();
  };

  const buscar = (valor: string) => {
    setBusqueda(valor);
    setPagina(1);
  };

  const toggleSeleccion = (id: string) => {
    setSeleccionados((prev) =>
      prev.includes(id) ? prev.filter((s) => s !== id) : prev.length < 2 ? [...prev, id] : prev
    );
  };

  /* [044A-8] Alterna ordenamiento por columna */
  const toggleSort = (columna: string) => {
    setSortBy(prev => {
      const newOrder = prev === columna && sortOrder === 'asc' ? 'desc' : 'asc';
      setSortOrder(newOrder);
      setPagina(1);
      return columna;
    });
  };

  return {
    pagina,
    setPagina,
    busqueda,
    buscar,
    sortBy,
    sortOrder,
    toggleSort,
    modalCrear,
    setModalCrear,
    clienteEditar,
    setClienteEditar,
    porPagina,
    clientes,
    isLoading,
    eliminarMut,
    cerrarModalYRefrescar,
    seleccionados,
    setSeleccionados,
    toggleSeleccion,
    modalMerge,
    setModalMerge,
    mergeMut,
  };
}

export default useListaClientes;

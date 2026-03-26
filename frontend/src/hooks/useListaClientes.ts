/* 263A-1: Hook para ListaClientes — reduce useState count en el componente */

import { useState } from 'react';
import { useListarClientes, useEliminarCliente, Cliente } from '../api/generated';

function useListaClientes() {
  const [pagina, setPagina] = useState(1);
  const [busqueda, setBusqueda] = useState('');
  const [modalCrear, setModalCrear] = useState(false);
  const [clienteEditar, setClienteEditar] = useState<Cliente | null>(null);
  const porPagina = 25;

  const { data, isLoading, refetch } = useListarClientes({
    page: pagina,
    per_page: porPagina,
    busqueda: busqueda || undefined,
  });

  const eliminarMut = useEliminarCliente({
    mutation: { onSuccess: () => { refetch(); } },
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

  return {
    pagina,
    setPagina,
    busqueda,
    buscar,
    modalCrear,
    setModalCrear,
    clienteEditar,
    setClienteEditar,
    porPagina,
    clientes,
    isLoading,
    eliminarMut,
    cerrarModalYRefrescar,
  };
}

export default useListaClientes;

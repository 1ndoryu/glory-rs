/* [134A-3] Hook para gestión de reglas de inactividad de clientes.
 * CRUD completo. Cada regla define: nombre, días sin actividad, canal de envío,
 * plantilla del mensaje, y toggle activa/inactiva. */

import { useState } from 'react';
import { toast } from 'sonner';
import {
  useListarInactividad,
  useCrearInactividad,
  useActualizarInactividad,
  useEliminarInactividad,
} from '../api/generated/inactividad/inactividad';
import type {
  ReglaInactividad,
  CrearReglaInactividadRequest,
  ActualizarReglaInactividadRequest,
} from '../api/generated/gestionRestauranteAPI.schemas';

function extraerError(err: unknown): string {
  const e = err as { response?: { data?: { message?: string } } };
  return e?.response?.data?.message || 'Error inesperado';
}

export function useInactividad() {
  const [modalCrear, setModalCrear] = useState(false);
  const [reglaEditar, setReglaEditar] = useState<ReglaInactividad | null>(null);

  const { data, isLoading, refetch } = useListarInactividad({
    query: { queryKey: ['inactividad'] },
  });

  const crearMut = useCrearInactividad({
    mutation: {
      onSuccess: () => {
        toast.success('Regla de inactividad creada');
        refetch();
        setModalCrear(false);
      },
      onError: (err: unknown) => toast.error(extraerError(err)),
    },
  });

  const actualizarMut = useActualizarInactividad({
    mutation: {
      onSuccess: () => {
        toast.success('Regla actualizada');
        refetch();
        setReglaEditar(null);
      },
      onError: (err: unknown) => toast.error(extraerError(err)),
    },
  });

  const eliminarMut = useEliminarInactividad({
    mutation: {
      onSuccess: () => {
        toast.success('Regla eliminada');
        refetch();
      },
      onError: (err: unknown) => toast.error(extraerError(err)),
    },
  });

  const reglas: ReglaInactividad[] = data?.data ?? [];

  return {
    reglas,
    isLoading,
    modalCrear,
    setModalCrear,
    reglaEditar,
    setReglaEditar,
    crearRegla: (req: CrearReglaInactividadRequest) => crearMut.mutate({ data: req }),
    actualizarRegla: (id: string, req: ActualizarReglaInactividadRequest) =>
      actualizarMut.mutate({ id, data: req }),
    eliminarRegla: (id: string) => eliminarMut.mutate({ id }),
    creando: crearMut.isPending,
    actualizando: actualizarMut.isPending,
    refetch,
  };
}

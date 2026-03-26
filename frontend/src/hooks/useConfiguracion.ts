/* [263A-17] Hook para el formulario de configuración del restaurante.
 * Gestiona estado local, carga inicial y guardado vía API. */

import { useState, useEffect, useCallback } from 'react';
import {
  useObtenerConfiguracion,
  useActualizarConfiguracion,
} from '../api/generated/configuracion/configuracion';
import type { ActualizarConfiguracionRequest } from '../api/generated/gestiNRestauranteAPI.schemas';

interface EstadoConfiguracion {
  reserva_email_obligatorio: boolean;
  reserva_telefono_obligatorio: boolean;
  reserva_nombre_obligatorio: boolean;
  reserva_apellidos_obligatorio: boolean;
  iva_por_defecto: number;
  nombre_restaurante: string;
}

const DEFAULTS: EstadoConfiguracion = {
  reserva_email_obligatorio: false,
  reserva_telefono_obligatorio: true,
  reserva_nombre_obligatorio: true,
  reserva_apellidos_obligatorio: false,
  iva_por_defecto: 10,
  nombre_restaurante: '',
};

export function useConfiguracion() {
  const [config, setConfig] = useState<EstadoConfiguracion>(DEFAULTS);
  const [mensaje, setMensaje] = useState('');

  const { data: datos, isLoading } = useObtenerConfiguracion();
  const mutacion = useActualizarConfiguracion();

  /* Sincronizar datos del servidor al estado local */
  useEffect(() => {
    if (datos && datos.status === 200) {
      const d = datos.data;
      setConfig({
        reserva_email_obligatorio: d.reserva_email_obligatorio,
        reserva_telefono_obligatorio: d.reserva_telefono_obligatorio,
        reserva_nombre_obligatorio: d.reserva_nombre_obligatorio,
        reserva_apellidos_obligatorio: d.reserva_apellidos_obligatorio,
        iva_por_defecto: Number(d.iva_por_defecto),
        nombre_restaurante: d.nombre_restaurante,
      });
    }
  }, [datos]);

  const cambiarCampo = useCallback(
    <K extends keyof EstadoConfiguracion>(campo: K, valor: EstadoConfiguracion[K]) => {
      setConfig((prev) => ({ ...prev, [campo]: valor }));
    },
    [],
  );

  const guardar = useCallback(async () => {
    setMensaje('');
    const body: ActualizarConfiguracionRequest = {
      reserva_email_obligatorio: config.reserva_email_obligatorio,
      reserva_telefono_obligatorio: config.reserva_telefono_obligatorio,
      reserva_nombre_obligatorio: config.reserva_nombre_obligatorio,
      reserva_apellidos_obligatorio: config.reserva_apellidos_obligatorio,
      iva_por_defecto: String(config.iva_por_defecto),
      nombre_restaurante: config.nombre_restaurante,
    };
    try {
      await mutacion.mutateAsync({ data: body });
      setMensaje('Configuración guardada');
    } catch {
      setMensaje('Error al guardar');
    }
  }, [config, mutacion]);

  return { config, cambiarCampo, guardar, mensaje, cargando: isLoading, guardando: mutacion.isPending };
}

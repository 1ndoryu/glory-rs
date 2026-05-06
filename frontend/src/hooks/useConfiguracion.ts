/* [263A-17] Hook para el formulario de configuración del restaurante.
 * Gestiona estado local, carga inicial y guardado vía API.
 * [283A-8] Añadido groq_api_key para digitalización de documentos.
 * [064A-5] Añadidos haddock_api_token y haddock_sync_enabled para sincronización con Haddock POS API.
 * [065A-2] Añadida configuración BDP/WebLink REST API. */

import { useState, useEffect, useCallback } from 'react';
import { useQueryClient } from '@tanstack/react-query';
import {
  useObtenerConfiguracion,
  useActualizarConfiguracion,
  getObtenerConfiguracionQueryKey,
} from '../api/generated/configuracion/configuracion';
import type { ActualizarConfiguracionRequest } from '../api/generated/gestionRestauranteAPI.schemas';

export interface EstadoConfiguracion {
  reserva_email_obligatorio: boolean;
  reserva_telefono_obligatorio: boolean;
  reserva_nombre_obligatorio: boolean;
  reserva_apellidos_obligatorio: boolean;
  iva_por_defecto: number;
  nombre_restaurante: string;
  groq_api_key: string;
  /* [014A-1] Auto-venta al completar reserva */
  auto_venta_reserva: boolean;
  /* [014A-4] Turnos configurables */
  hora_desayuno_inicio: string;
  hora_desayuno_fin: string;
  hora_comida_inicio: string;
  hora_comida_fin: string;
  hora_cena_inicio: string;
  hora_cena_fin: string;
  /* [034A-3] URL de Haddock */
  url_haddock: string;
  /* [064A-5] Token API y toggle de sincronización Haddock */
  haddock_api_token: string;
  haddock_sync_enabled: boolean;
  /* [065A-2] BDP WebLink REST API */
  bdp_base_url: string;
  bdp_login: string;
  bdp_password: string;
  bdp_integrator_code: string;
  bdp_sync_enabled: boolean;
  bdp_pos_id: number;
  bdp_employee_id: number;
  bdp_items_profile_id: number;
  /* [134A-4] Reseñas y CTA WhatsApp */
  google_review_url: string;
  telefono_restaurante: string;
  url_reservas: string;
}

const DEFAULTS: EstadoConfiguracion = {
  reserva_email_obligatorio: false,
  reserva_telefono_obligatorio: true,
  reserva_nombre_obligatorio: true,
  reserva_apellidos_obligatorio: false,
  iva_por_defecto: 10,
  nombre_restaurante: '',
  groq_api_key: '',
  auto_venta_reserva: false,
  hora_desayuno_inicio: '00:00:00',
  hora_desayuno_fin: '12:00:00',
  hora_comida_inicio: '12:00:00',
  hora_comida_fin: '18:00:00',
  hora_cena_inicio: '18:00:00',
  hora_cena_fin: '23:59:59',
  url_haddock: '',
  haddock_api_token: '',
  haddock_sync_enabled: false,
  bdp_base_url: '',
  bdp_login: '',
  bdp_password: '',
  bdp_integrator_code: '',
  bdp_sync_enabled: false,
  bdp_pos_id: 1,
  bdp_employee_id: 1,
  bdp_items_profile_id: 1,
  google_review_url: '',
  telefono_restaurante: '',
  url_reservas: '',
};

export function useConfiguracion() {
  const [config, setConfig] = useState<EstadoConfiguracion>(DEFAULTS);
  const [mensaje, setMensaje] = useState('');
  const queryClient = useQueryClient();

  const { data: datos, isLoading } = useObtenerConfiguracion();
  const mutacion = useActualizarConfiguracion();

  /* Sincronizar datos del servidor al estado local */
  useEffect(() => {
    if (datos && datos.status === 200) {
      const d = datos.data;
      const raw = d as unknown as Record<string, string | number | boolean>;
      setConfig({
        reserva_email_obligatorio: d.reserva_email_obligatorio,
        reserva_telefono_obligatorio: d.reserva_telefono_obligatorio,
        reserva_nombre_obligatorio: d.reserva_nombre_obligatorio,
        reserva_apellidos_obligatorio: d.reserva_apellidos_obligatorio,
        iva_por_defecto: Number(d.iva_por_defecto),
        nombre_restaurante: d.nombre_restaurante,
        /* [283A-8] groq_api_key no viene en la respuesta (skip_serializing),
         * mantener valor local si ya fue editado */
        groq_api_key: config.groq_api_key || '',
        /* [014A-1] */
        auto_venta_reserva: d.auto_venta_reserva,
        /* [014A-4] Turnos — formato HH:MM:SS desde el servidor */
        hora_desayuno_inicio: d.hora_desayuno_inicio,
        hora_desayuno_fin: d.hora_desayuno_fin,
        hora_comida_inicio: d.hora_comida_inicio,
        hora_comida_fin: d.hora_comida_fin,
        hora_cena_inicio: d.hora_cena_inicio,
        hora_cena_fin: d.hora_cena_fin,
        url_haddock: d.url_haddock ?? '',
        /* [064A-5] haddock_api_token no viene en la respuesta (skip_serializing),
         * mantener valor local si ya fue editado */
        haddock_api_token: config.haddock_api_token || '',
        haddock_sync_enabled: d.haddock_sync_enabled,
        bdp_base_url: String(raw.bdp_base_url ?? ''),
        bdp_login: config.bdp_login || '',
        bdp_password: config.bdp_password || '',
        bdp_integrator_code: config.bdp_integrator_code || '',
        bdp_sync_enabled: Boolean(raw.bdp_sync_enabled ?? false),
        bdp_pos_id: Number(raw.bdp_pos_id ?? 1),
        bdp_employee_id: Number(raw.bdp_employee_id ?? 1),
        bdp_items_profile_id: Number(raw.bdp_items_profile_id ?? 1),
        google_review_url: String(raw.google_review_url ?? ''),
        telefono_restaurante: String(raw.telefono_restaurante ?? ''),
        url_reservas: String(raw.url_reservas ?? ''),
      });
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
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
      /* [283A-8] Solo enviar groq_api_key si el usuario la ha editado */
      ...(config.groq_api_key ? { groq_api_key: config.groq_api_key } : {}),
      /* [014A-1] Auto-venta */
      auto_venta_reserva: config.auto_venta_reserva,
      /* [014A-4] Turnos configurables */
      hora_desayuno_inicio: config.hora_desayuno_inicio,
      hora_desayuno_fin: config.hora_desayuno_fin,
      hora_comida_inicio: config.hora_comida_inicio,
      hora_comida_fin: config.hora_comida_fin,
      hora_cena_inicio: config.hora_cena_inicio,
      hora_cena_fin: config.hora_cena_fin,
      /* [034A-3] URL de Haddock */
      url_haddock: config.url_haddock || undefined,
      /* [064A-5] Haddock POS API sync */
      ...(config.haddock_api_token ? { haddock_api_token: config.haddock_api_token } : {}),
      haddock_sync_enabled: config.haddock_sync_enabled,
      /* [065A-2] BDP WebLink REST API */
      bdp_base_url: config.bdp_base_url || undefined,
      ...(config.bdp_login ? { bdp_login: config.bdp_login } : {}),
      ...(config.bdp_password ? { bdp_password: config.bdp_password } : {}),
      ...(config.bdp_integrator_code ? { bdp_integrator_code: config.bdp_integrator_code } : {}),
      bdp_sync_enabled: config.bdp_sync_enabled,
      bdp_pos_id: config.bdp_pos_id,
      bdp_employee_id: config.bdp_employee_id,
      bdp_items_profile_id: config.bdp_items_profile_id,
      /* [134A-4] Reseñas y CTA WhatsApp */
      google_review_url: config.google_review_url || undefined,
      telefono_restaurante: config.telefono_restaurante || undefined,
      url_reservas: config.url_reservas || undefined,
    };
    try {
      await mutacion.mutateAsync({ data: body });
      /* [014A-6] Invalidar cache para que la UI refleje el valor actualizado del servidor */
      await queryClient.invalidateQueries({ queryKey: getObtenerConfiguracionQueryKey() });
      setMensaje('Configuración guardada');
    } catch {
      setMensaje('Error al guardar');
    }
  }, [config, mutacion]);

  return { config, cambiarCampo, guardar, mensaje, cargando: isLoading, guardando: mutacion.isPending };
}

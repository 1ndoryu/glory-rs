/* [283A-23] Hook para el formulario de integraciones de marketing.
 * Gestiona SMTP, Twilio y Meta credentials vía el endpoint /api/configuracion/integraciones.
 * Los campos de contraseña nunca vuelven del servidor (skip_serializing). */

import { useState, useEffect, useCallback } from 'react';
import {
  useObtenerIntegraciones,
  useActualizarIntegraciones,
} from '../api/generated/configuracion/configuracion';
import type { ActualizarIntegracionesRequest } from '../api/generated/gestionRestauranteAPI.schemas';

interface EstadoIntegraciones {
  smtp_host: string;
  smtp_port: number;
  smtp_user: string;
  smtp_password: string;
  smtp_from_email: string;
  smtp_from_name: string;
  twilio_account_sid: string;
  twilio_auth_token: string;
  twilio_from_number: string;
  meta_waba_id: string;
  meta_business_app_id: string;
  meta_access_token: string;
  meta_phone_number_id: string;
}

const DEFAULTS: EstadoIntegraciones = {
  smtp_host: '',
  smtp_port: 587,
  smtp_user: '',
  smtp_password: '',
  smtp_from_email: '',
  smtp_from_name: '',
  twilio_account_sid: '',
  twilio_auth_token: '',
  twilio_from_number: '',
  meta_waba_id: '',
  meta_business_app_id: '',
  meta_access_token: '',
  meta_phone_number_id: '',
};

export function useIntegraciones() {
  const [form, setForm] = useState<EstadoIntegraciones>(DEFAULTS);
  const [mensaje, setMensaje] = useState('');

  const { data: datos, isLoading } = useObtenerIntegraciones();
  const mutacion = useActualizarIntegraciones();

  /* Estado del servidor: campos públicos (sin passwords) */
  const smtpConfigurado = datos?.status === 200 ? datos.data.smtp_configurado : false;
  const twilioConfigurado = datos?.status === 200 ? datos.data.twilio_configurado : false;
  const metaConfigurado = datos?.status === 200 ? datos.data.meta_configurado : false;

  /* Sincronizar campos públicos visibles */
  useEffect(() => {
    if (datos && datos.status === 200) {
      const d = datos.data;
      setForm((prev) => ({
        ...prev,
        smtp_from_email: d.smtp_from_email ?? '',
        smtp_from_name: d.smtp_from_name ?? '',
        twilio_from_number: d.twilio_from_number ?? '',
        meta_waba_id: d.meta_waba_id ?? '',
        meta_phone_number_id: d.meta_phone_number_id ?? '',
      }));
    }
  }, [datos]);

  const cambiarCampo = useCallback(
    <K extends keyof EstadoIntegraciones>(campo: K, valor: EstadoIntegraciones[K]) => {
      setForm((p) => ({ ...p, [campo]: valor }));
    },
    [],
  );

  const guardar = useCallback(async () => {
    setMensaje('');

    /* Solo enviar campos que tengan valor (no vacíos) */
    const payload: ActualizarIntegracionesRequest = {};
    if (form.smtp_host) payload.smtp_host = form.smtp_host;
    if (form.smtp_port) payload.smtp_port = form.smtp_port;
    if (form.smtp_user) payload.smtp_user = form.smtp_user;
    if (form.smtp_password) payload.smtp_password = form.smtp_password;
    if (form.smtp_from_email) payload.smtp_from_email = form.smtp_from_email;
    if (form.smtp_from_name) payload.smtp_from_name = form.smtp_from_name;
    if (form.twilio_account_sid) payload.twilio_account_sid = form.twilio_account_sid;
    if (form.twilio_auth_token) payload.twilio_auth_token = form.twilio_auth_token;
    if (form.twilio_from_number) payload.twilio_from_number = form.twilio_from_number;
    if (form.meta_waba_id) payload.meta_waba_id = form.meta_waba_id;
    if (form.meta_business_app_id) payload.meta_business_app_id = form.meta_business_app_id;
    if (form.meta_access_token) payload.meta_access_token = form.meta_access_token;
    if (form.meta_phone_number_id) payload.meta_phone_number_id = form.meta_phone_number_id;

    try {
      await mutacion.mutateAsync({ data: payload });
      setMensaje('Integraciones guardadas');
      /* Limpiar campos sensibles después de guardar */
      setForm((p) => ({
        ...p,
        smtp_password: '',
        twilio_auth_token: '',
        meta_access_token: '',
      }));
    } catch {
      setMensaje('Error al guardar integraciones');
    }
  }, [form, mutacion]);

  return {
    form,
    cambiarCampo,
    guardar,
    mensaje,
    cargando: isLoading,
    guardando: mutacion.isPending,
    smtpConfigurado,
    twilioConfigurado,
    metaConfigurado,
  };
}

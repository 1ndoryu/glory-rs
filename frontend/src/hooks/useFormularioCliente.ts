/* 263A-1: Hook para FormularioCliente — CRM crear/editar cliente */

import { useState, FormEvent } from 'react';
import { useCrearCliente, useActualizarCliente, Cliente } from '../api/generated';

interface CamposCliente {
  nombre: string;
  apellidos: string;
  telefono: string;
  prefijoTelefono: string;
  email: string;
  empresa: string;
  notas: string;
  alergias: string;
  preferenciasBebida: string;
  preferenciasUbicacion: string;
  consentimientoEmail: boolean;
  consentimientoSms: boolean;
  enviarEncuestas: boolean;
}

function camposIniciales(cliente?: Cliente): CamposCliente {
  return {
    nombre: cliente?.nombre ?? '',
    apellidos: cliente?.apellidos ?? '',
    telefono: cliente?.telefono ?? '',
    prefijoTelefono: cliente?.prefijo_telefono ?? '+34',
    email: cliente?.email ?? '',
    empresa: cliente?.empresa ?? '',
    notas: cliente?.notas ?? '',
    alergias: cliente?.alergias ?? '',
    preferenciasBebida: cliente?.preferencias_bebida ?? '',
    preferenciasUbicacion: cliente?.preferencias_ubicacion ?? '',
    consentimientoEmail: cliente?.consentimiento_comercial_email ?? false,
    consentimientoSms: cliente?.consentimiento_comercial_sms ?? false,
    enviarEncuestas: cliente?.enviar_encuestas ?? false,
  };
}

function useFormularioCliente(onExito?: () => void, clienteEditar?: Cliente) {
  const [error, setError] = useState('');
  const [campos, setCampos] = useState<CamposCliente>(camposIniciales(clienteEditar));

  const cambiarCampo = <K extends keyof CamposCliente>(campo: K, valor: CamposCliente[K]) => {
    setCampos(prev => ({ ...prev, [campo]: valor }));
  };

  const crearMut = useCrearCliente({
    mutation: {
      onSuccess: (res) => { if (res.status === 201 && onExito) onExito(); },
      onError: () => setError('Error al crear el cliente'),
    },
  });

  const actualizarMut = useActualizarCliente({
    mutation: {
      onSuccess: (res) => { if (res.status === 200 && onExito) onExito(); },
      onError: () => setError('Error al actualizar el cliente'),
    },
  });

  const cargando = crearMut.isPending || actualizarMut.isPending;

  const manejarEnvio = (e: FormEvent) => {
    e.preventDefault();
    setError('');
    if (!campos.nombre.trim()) {
      setError('El nombre es obligatorio');
      return;
    }

    const payload = {
      nombre: campos.nombre,
      apellidos: campos.apellidos || null,
      telefono: campos.telefono || null,
      prefijo_telefono: campos.prefijoTelefono || null,
      email: campos.email || null,
      empresa: campos.empresa || null,
      notas: campos.notas || null,
      alergias: campos.alergias || null,
      preferencias_bebida: campos.preferenciasBebida || null,
      preferencias_ubicacion: campos.preferenciasUbicacion || null,
      consentimiento_comercial_email: campos.consentimientoEmail,
      consentimiento_comercial_sms: campos.consentimientoSms,
      enviar_encuestas: campos.enviarEncuestas,
    };

    if (clienteEditar) {
      actualizarMut.mutate({ id: clienteEditar.id, data: payload });
    } else {
      crearMut.mutate({ data: payload });
    }
  };

  return { campos, cambiarCampo, error, manejarEnvio, cargando, esEdicion: !!clienteEditar };
}

export default useFormularioCliente;

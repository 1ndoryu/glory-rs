/* 253A-10: Hook para FormularioReserva
   263A-6: Añade num_mesa y apellidos_cliente */

import { useState, FormEvent } from 'react';
import { useNavigate } from 'react-router-dom';
import { useCrearReserva, EstadoReserva } from '../api/generated';

interface CamposReserva {
  fecha: string;
  hora: string;
  nombreCliente: string;
  apellidosCliente: string;
  numPersonas: string;
  numMesa: string;
  telefono: string;
  notas: string;
  estado: EstadoReserva;
}

function useFormularioReserva(onExito?: () => void) {
  const navigate = useNavigate();
  const [error, setError] = useState('');
  const [campos, setCampos] = useState<CamposReserva>({
    fecha: new Date().toISOString().split('T')[0],
    hora: '20:00',
    nombreCliente: '',
    apellidosCliente: '',
    numPersonas: '2',
    numMesa: '',
    telefono: '',
    notas: '',
    estado: EstadoReserva.pendiente,
  });

  const cambiarCampo = <K extends keyof CamposReserva>(campo: K, valor: CamposReserva[K]) => {
    setCampos(prev => ({ ...prev, [campo]: valor }));
  };

  const mutation = useCrearReserva({
    mutation: {
      onSuccess: (res) => {
        if (res.status === 201) {
          if (onExito) onExito();
          else navigate('/reservas');
        }
      },
      onError: () => setError('Error al crear la reserva'),
    },
  });

  const manejarEnvio = (e: FormEvent) => {
    e.preventDefault();
    setError('');
    if (!campos.fecha || !campos.hora || !campos.nombreCliente || !campos.numPersonas) {
      setError('Completa los campos obligatorios');
      return;
    }
    mutation.mutate({
      data: {
        fecha: campos.fecha,
        hora: campos.hora,
        nombre_cliente: campos.nombreCliente,
        num_personas: parseInt(campos.numPersonas, 10),
        telefono: campos.telefono || null,
        notas: campos.notas || null,
        estado: campos.estado,
        num_mesa: campos.numMesa ? parseInt(campos.numMesa, 10) : null,
        apellidos_cliente: campos.apellidosCliente || null,
      },
    });
  };

  return { campos, cambiarCampo, error, manejarEnvio, cargando: mutation.isPending };
}

export default useFormularioReserva;

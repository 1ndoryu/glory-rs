/* [134A-29] Hook para Login — soporta propietario, registro y trabajador.
 * El modo trabajador llama a /api/auth/login-trabajador con el JWT que incluye tid+permisos. */

import { useState, FormEvent } from 'react';
import { useNavigate } from 'react-router-dom';
import { useLogin, useRegister } from '../api/generated';
import { useLoginTrabajador } from '../api/generated/trabajadores/trabajadores';
import { useAuthStore } from '../stores/authStore';

function useLoginForm() {
  const navigate = useNavigate();
  const iniciarSesion = useAuthStore((s) => s.iniciarSesion);
  const [credenciales, setCredenciales] = useState({ email: '', password: '' });
  const [modoRegistro, setModoRegistro] = useState(false);
  const [esTrabajador, setEsTrabajador] = useState(false);
  const [error, setError] = useState('');

  const cambiarCredencial = (campo: 'email' | 'password', valor: string) => {
    setCredenciales(prev => ({ ...prev, [campo]: valor }));
  };

  const loginMutation = useLogin({
    mutation: {
      onSuccess: (respuesta) => {
        if (respuesta.status === 200) {
          iniciarSesion(respuesta.data.token);
          navigate('/');
        }
      },
      onError: () => setError('Credenciales incorrectas'),
    },
  });

  const loginTrabajadorMut = useLoginTrabajador({
    mutation: {
      onSuccess: (respuesta) => {
        if (respuesta.status === 200) {
          iniciarSesion(respuesta.data.token);
          navigate('/');
        }
      },
      onError: () => setError('Credenciales incorrectas'),
    },
  });

  const registerMutation = useRegister({
    mutation: {
      onSuccess: (respuesta) => {
        if (respuesta.status === 201) {
          iniciarSesion(respuesta.data.token);
          navigate('/');
        }
      },
      onError: () => setError('No se pudo crear la cuenta. Verifica los datos.'),
    },
  });

  const manejarEnvio = (e: FormEvent) => {
    e.preventDefault();
    setError('');
    if (!credenciales.email || !credenciales.password) {
      setError('Completa todos los campos');
      return;
    }
    if (modoRegistro) {
      registerMutation.mutate({ data: credenciales });
    } else if (esTrabajador) {
      loginTrabajadorMut.mutate({ data: { email: credenciales.email, password: credenciales.password } });
    } else {
      loginMutation.mutate({ data: credenciales });
    }
  };

  const alternarModo = () => {
    setModoRegistro(!modoRegistro);
    setEsTrabajador(false);
    setError('');
  };

  const alternarTrabajador = () => {
    setEsTrabajador(!esTrabajador);
    setModoRegistro(false);
    setError('');
  };

  const cargando = loginMutation.isPending || registerMutation.isPending || loginTrabajadorMut.isPending;

  return {
    credenciales, cambiarCredencial,
    modoRegistro, alternarModo,
    esTrabajador, alternarTrabajador,
    error, manejarEnvio, cargando,
  };
}

export default useLoginForm;

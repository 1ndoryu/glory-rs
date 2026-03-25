/* 253A-10: Hook para Login — reduce 4 useState a 3 (regla usestate-excesivo, max 3) */

import { useState, FormEvent } from 'react';
import { useNavigate } from 'react-router-dom';
import { useLogin, useRegister } from '../api/generated';
import { useAuthStore } from '../stores/authStore';

function useLoginForm() {
  const navigate = useNavigate();
  const iniciarSesion = useAuthStore((s) => s.iniciarSesion);
  const [credenciales, setCredenciales] = useState({ email: '', password: '' });
  const [modoRegistro, setModoRegistro] = useState(false);
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
    } else {
      loginMutation.mutate({ data: credenciales });
    }
  };

  const alternarModo = () => {
    setModoRegistro(!modoRegistro);
    setError('');
  };

  const cargando = loginMutation.isPending || registerMutation.isPending;

  return { credenciales, cambiarCredencial, modoRegistro, alternarModo, error, manejarEnvio, cargando };
}

export default useLoginForm;

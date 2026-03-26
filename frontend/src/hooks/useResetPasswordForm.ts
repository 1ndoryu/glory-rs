/* [263A-15] Hook para ResetPassword — lógica de formulario de nueva contraseña.
 * Consolida los 4 useState del componente original en un solo estado + lógica de mutación. */

import { useState, FormEvent } from 'react';
import { useSearchParams, useNavigate } from 'react-router-dom';
import { useResetPassword as useResetPasswordMutation } from '../api/generated';

interface EstadoFormulario {
  password: string;
  confirmar: string;
  error: string;
  exito: boolean;
}

function useResetPasswordForm() {
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const token = searchParams.get('token') || '';

  const [form, setForm] = useState<EstadoFormulario>({
    password: '',
    confirmar: '',
    error: '',
    exito: false,
  });

  const set = (campo: Partial<EstadoFormulario>) =>
    setForm(prev => ({ ...prev, ...campo }));

  const mutation = useResetPasswordMutation({
    mutation: {
      onSuccess: () => {
        set({ exito: true });
        setTimeout(() => navigate('/login'), 3000);
      },
      onError: () => set({ error: 'El enlace ha expirado o no es válido. Solicita uno nuevo.' }),
    },
  });

  const manejarEnvio = (e: FormEvent) => {
    e.preventDefault();
    set({ error: '' });

    if (!token) {
      set({ error: 'Token de recuperación no encontrado. Usa el enlace del email.' });
      return;
    }
    if (form.password.length < 8) {
      set({ error: 'La contraseña debe tener al menos 8 caracteres.' });
      return;
    }
    if (form.password !== form.confirmar) {
      set({ error: 'Las contraseñas no coinciden.' });
      return;
    }

    mutation.mutate({ data: { token, new_password: form.password } });
  };

  return {
    form,
    set,
    manejarEnvio,
    cargando: mutation.isPending,
  };
}

export default useResetPasswordForm;

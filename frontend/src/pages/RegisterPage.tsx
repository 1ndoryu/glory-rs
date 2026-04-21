import { useEffect } from 'react';
import { Navigate } from 'react-router-dom';
import { useAuthModalStore } from '../app/stores/authModalStore';

export default function RegisterPage() {
  const abrirAuth = useAuthModalStore((state) => state.abrir);

  useEffect(() => {
    abrirAuth('registro');
  }, [abrirAuth]);

  return <Navigate replace to="/" />;
}

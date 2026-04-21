import { useEffect } from 'react';
import { Navigate } from 'react-router-dom';
import { useAuthModalStore } from '../app/stores/authModalStore';

export default function LoginPage() {
  const abrirAuth = useAuthModalStore((state) => state.abrir);

  useEffect(() => {
    abrirAuth('login');
  }, [abrirAuth]);

  return <Navigate replace to="/" />;
}

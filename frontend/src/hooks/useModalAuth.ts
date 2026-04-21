import { useCallback } from 'react';
import { useAuthModalStore } from '../app/stores/authModalStore';

export function useModalAuth() {
  const abierto = useAuthModalStore((state) => state.abierto);
  const vista = useAuthModalStore((state) => state.vista);
  const cerrarStore = useAuthModalStore((state) => state.cerrar);
  const cambiarVista = useAuthModalStore((state) => state.cambiarVista);

  const cerrar = useCallback(() => {
    cerrarStore();
  }, [cerrarStore]);

  const cambiarALogin = useCallback(() => {
    cambiarVista('login');
  }, [cambiarVista]);

  const cambiarARegistro = useCallback(() => {
    cambiarVista('registro');
  }, [cambiarVista]);

  return {
    abierto,
    vista,
    cerrar,
    cambiarALogin,
    cambiarARegistro,
    puedesCerrar: true,
  };
}

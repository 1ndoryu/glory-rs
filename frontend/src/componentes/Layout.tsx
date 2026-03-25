/* 253A-7: Layout — envuelve las páginas autenticadas con sidebar */

import { Navigate, Outlet } from 'react-router-dom';
import { useAuthStore } from '../stores/authStore';
import BarraLateral from './BarraLateral';
import '../estilos/Layout.css';

function Layout() {
  const autenticado = useAuthStore((s) => s.estaAutenticado)();

  if (!autenticado) {
    return <Navigate to="/login" replace />;
  }

  return (
    <div className="layoutPrincipal">
      <BarraLateral />
      <main className="contenidoPrincipal">
        <Outlet />
      </main>
    </div>
  );
}

export default Layout;

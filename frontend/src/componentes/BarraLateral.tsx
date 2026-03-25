/* 253A-7: Barra lateral — navegación principal */

import { NavLink, useNavigate } from 'react-router-dom';
import { useAuthStore } from '../stores/authStore';
import '../estilos/BarraLateral.css';

function BarraLateral() {
  const cerrarSesion = useAuthStore((s) => s.cerrarSesion);
  const navigate = useNavigate();

  const salir = () => {
    cerrarSesion();
    navigate('/login');
  };

  return (
    <aside className="barraLateral">
      <div className="logoBarraLateral">🍽️ Restaurante</div>

      <nav className="navegacionLateral">
        <NavLink to="/" end className={({ isActive }) => `enlaceLateral ${isActive ? 'activo' : ''}`}>
          <span className="iconoLateral">🏠</span> Inicio
        </NavLink>
        <NavLink to="/ventas" className={({ isActive }) => `enlaceLateral ${isActive ? 'activo' : ''}`}>
          <span className="iconoLateral">💰</span> Ventas
        </NavLink>
        <NavLink to="/gastos" className={({ isActive }) => `enlaceLateral ${isActive ? 'activo' : ''}`}>
          <span className="iconoLateral">📊</span> Gastos
        </NavLink>
        <NavLink to="/reservas" className={({ isActive }) => `enlaceLateral ${isActive ? 'activo' : ''}`}>
          <span className="iconoLateral">📋</span> Reservas
        </NavLink>
      </nav>

      <div className="pieBarraLateral">
        <button className="botonCerrarSesion" onClick={salir}>
          Cerrar sesión
        </button>
      </div>
    </aside>
  );
}

export default BarraLateral;

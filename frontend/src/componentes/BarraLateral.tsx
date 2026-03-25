/* 253A-7: Barra lateral — navegación principal
   253A-9: emojis reemplazados por SVG inline (regla 18: emojis prohibidos) */

import { NavLink, useNavigate } from 'react-router-dom';
import { useAuthStore } from '../stores/authStore';
import { Boton } from './ui';
import '../estilos/BarraLateral.css';

/* SVG icons para la barra lateral — evita emojis Unicode (regla 18) */
const IconoInicio = () => (
  <svg className="iconoLateral" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
    <path d="M3 12l9-9 9 9M5 10v10a1 1 0 001 1h3m10-11v10a1 1 0 01-1 1h-3m-4 0v-6a1 1 0 011-1h2a1 1 0 011 1v6" />
  </svg>
);
const IconoVentas = () => (
  <svg className="iconoLateral" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
    <path d="M12 2v20M17 5H9.5a3.5 3.5 0 000 7h5a3.5 3.5 0 010 7H6" />
  </svg>
);
const IconoGastos = () => (
  <svg className="iconoLateral" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
    <path d="M18 20V10M12 20V4M6 20v-6" />
  </svg>
);
const IconoReservas = () => (
  <svg className="iconoLateral" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
    <path d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
  </svg>
);

function BarraLateral() {
  const cerrarSesion = useAuthStore((s) => s.cerrarSesion);
  const navigate = useNavigate();

  const salir = () => {
    cerrarSesion();
    navigate('/login');
  };

  return (
    <aside className="barraLateral">
      <div className="logoBarraLateral">Restaurante</div>

      <nav className="navegacionLateral">
        <NavLink to="/" end className={({ isActive }) => `enlaceLateral ${isActive ? 'activo' : ''}`}>
          <IconoInicio /> Inicio
        </NavLink>
        <NavLink to="/ventas" className={({ isActive }) => `enlaceLateral ${isActive ? 'activo' : ''}`}>
          <IconoVentas /> Ventas
        </NavLink>
        <NavLink to="/gastos" className={({ isActive }) => `enlaceLateral ${isActive ? 'activo' : ''}`}>
          <IconoGastos /> Gastos
        </NavLink>
        <NavLink to="/reservas" className={({ isActive }) => `enlaceLateral ${isActive ? 'activo' : ''}`}>
          <IconoReservas /> Reservas
        </NavLink>
      </nav>

      <div className="pieBarraLateral">
        <Boton variante="fantasma" tamano="sm" onClick={salir}>
          Cerrar sesión
        </Boton>
      </div>
    </aside>
  );
}

export default BarraLateral;

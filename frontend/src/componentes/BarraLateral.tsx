/* 253A-7: Barra lateral -- navegacion principal
   253A-14: SVGs manuales reemplazados por lucide-react, icono logout */

import { NavLink, useNavigate } from 'react-router-dom';
import { Home, DollarSign, BarChart3, ClipboardList, Calendar, Users, LogOut } from 'lucide-react';
import { useAuthStore } from '../stores/authStore';
import { Boton } from '@glory/componentes/ui';
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
      <div className="logoBarraLateral">Restaurante</div>

      <nav className="navegacionLateral">
        <NavLink to="/" end className={({ isActive }) => `enlaceLateral ${isActive ? 'activo' : ''}`}>
          <Home size={18} /> Inicio
        </NavLink>
        <NavLink to="/ventas" className={({ isActive }) => `enlaceLateral ${isActive ? 'activo' : ''}`}>
          <DollarSign size={18} /> Ventas
        </NavLink>
        <NavLink to="/gastos" className={({ isActive }) => `enlaceLateral ${isActive ? 'activo' : ''}`}>
          <BarChart3 size={18} /> Gastos
        </NavLink>
        <NavLink to="/reservas" className={({ isActive }) => `enlaceLateral ${isActive ? 'activo' : ''}`}>
          <ClipboardList size={18} /> Reservas
        </NavLink>
        <NavLink to="/reservas/calendario" className={({ isActive }) => `enlaceLateral ${isActive ? 'activo' : ''}`}>
          <Calendar size={18} /> Calendario
        </NavLink>
        <NavLink to="/clientes" className={({ isActive }) => `enlaceLateral ${isActive ? 'activo' : ''}`}>
          <Users size={18} /> Clientes
        </NavLink>
      </nav>

      <div className="pieBarraLateral">
        <Boton variante="fantasma" tamano="sm" onClick={salir}>
          <LogOut size={16} /> Cerrar sesion
        </Boton>
      </div>
    </aside>
  );
}

export default BarraLateral;

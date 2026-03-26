/* 253A-7: App principal -- rutas y providers
   253A-14: formularios eliminados como rutas (ahora son modales) */

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Routes, Route, Navigate } from 'react-router-dom';
import { useAuthStore } from './stores/authStore';
import Layout from './componentes/Layout';
import Login from './componentes/Login';
import Inicio from './componentes/Inicio';
import ListaVentas from './componentes/ListaVentas';
import ListaGastos from './componentes/ListaGastos';
import ListaReservas from './componentes/ListaReservas';
import CalendarioReservas from './componentes/CalendarioReservas';
import ListaClientes from './componentes/ListaClientes';
import ListaCanales from './componentes/ListaCanales';
import EstadisticasNoShows from './componentes/EstadisticasNoShows';
import DashboardReservas from './componentes/DashboardReservas';
import './estilos/global.css';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 5 * 60 * 1000,
      retry: 1,
    },
  },
});

function App() {
  const autenticado = useAuthStore((s) => s.estaAutenticado)();

  return (
    <QueryClientProvider client={queryClient}>
      <Routes>
        <Route path="/login" element={autenticado ? <Navigate to="/" replace /> : <Login />} />

        <Route element={<Layout />}>
          <Route path="/" element={<Inicio />} />
          <Route path="/ventas" element={<ListaVentas />} />
          <Route path="/gastos" element={<ListaGastos />} />
          <Route path="/reservas" element={<ListaReservas />} />
          <Route path="/reservas/calendario" element={<CalendarioReservas />} />
          <Route path="/clientes" element={<ListaClientes />} />
          <Route path="/canales" element={<ListaCanales />} />
          <Route path="/reservas/no-shows" element={<EstadisticasNoShows />} />
          <Route path="/reservas/dashboard" element={<DashboardReservas />} />
        </Route>

        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </QueryClientProvider>
  );
}

export default App;

/* 253A-7: App principal — rutas y providers */

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Routes, Route, Navigate } from 'react-router-dom';
import { useAuthStore } from './stores/authStore';
import Layout from './componentes/Layout';
import Login from './componentes/Login';
import Inicio from './componentes/Inicio';
import ListaVentas from './componentes/ListaVentas';
import FormularioVenta from './componentes/FormularioVenta';
import ListaGastos from './componentes/ListaGastos';
import FormularioGasto from './componentes/FormularioGasto';
import ListaReservas from './componentes/ListaReservas';
import FormularioReserva from './componentes/FormularioReserva';
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
          <Route path="/ventas/nueva" element={<FormularioVenta />} />
          <Route path="/gastos" element={<ListaGastos />} />
          <Route path="/gastos/nuevo" element={<FormularioGasto />} />
          <Route path="/reservas" element={<ListaReservas />} />
          <Route path="/reservas/nueva" element={<FormularioReserva />} />
        </Route>

        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </QueryClientProvider>
  );
}

export default App;

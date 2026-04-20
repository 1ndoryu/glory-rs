/* [174A-103] Router SPA placeholder.
 *
 * Define las rutas mínimas que la SPA usará una vez se vayan migrando las
 * features (174A-104..107). Por ahora todas las páginas son stubs que
 * solo confirman que la navegación client-side funciona; cada feature
 * irá reemplazando su página al sacar su carpeta del exclude del tsconfig.
 *
 * Las rutas espejean los paths que conoce el SeoKamples del backend
 * (/sample/{slug}, /perfil/{username}, /blog/{slug}) y `seo::metadata`. */

import { lazy, Suspense } from 'react';
import { NavLink, createBrowserRouter, Outlet, RouterProvider } from 'react-router-dom';
import Boton from './components/ui/Boton';
import { useAuth } from './hooks/useAuth';

const HomePage = lazy(() => import('./pages/HomePage'));
const DashboardPage = lazy(() => import('./pages/DashboardPage'));
const LoginPage = lazy(() => import('./pages/LoginPage'));
const RegisterPage = lazy(() => import('./pages/RegisterPage'));
const SamplePage = lazy(() => import('./pages/SamplePage'));
const PerfilPage = lazy(() => import('./pages/PerfilPage'));
const BlogPage = lazy(() => import('./pages/BlogPage'));
const NotFoundPage = lazy(() => import('./pages/NotFoundPage'));

function Layout() {
  const auth = useAuth();

  const linkClassName = ({ isActive }: { isActive: boolean }) => (
    isActive ? 'enlace enlaceActivo' : 'enlace'
  );

  return (
    <div className="aplicacion">
      <header className="cabecera">
        <NavLink to="/" className="logo">Kamples</NavLink>
        <nav className="navegacion">
          <NavLink to="/" className={linkClassName}>Inicio</NavLink>
          <NavLink to="/dashboard" className={linkClassName}>Dashboard</NavLink>
          <NavLink to="/blog" className={linkClassName}>Blog</NavLink>
          <a className="enlace" href="/swagger-ui/" target="_blank" rel="noopener noreferrer">API</a>
        </nav>
        <div className="accionesCabecera">
          {auth.isLoading ? (
            <span className="estadoNeutral">Sesión…</span>
          ) : auth.isAuthenticated && auth.user ? (
            <>
              <NavLink to={`/perfil/${auth.user.username}`} className="accionSecundaria">
                @{auth.user.username}
              </NavLink>
              <Boton className="accionFantasma" onClick={() => { void auth.logout(); }} type="button">
                Salir
              </Boton>
            </>
          ) : (
            <>
              <NavLink to="/auth/login" className="accionSecundaria">Entrar</NavLink>
              <NavLink to="/auth/registro" className="accionPrimaria">Crear cuenta</NavLink>
            </>
          )}
        </div>
      </header>
      <main className="contenido">
        <Suspense fallback={<div className="cargando">Cargando…</div>}>
          <Outlet />
        </Suspense>
      </main>
    </div>
  );
}

const router = createBrowserRouter([
  {
    path: '/',
    element: <Layout />,
    children: [
      { index: true, element: <HomePage /> },
      { path: 'auth/login', element: <LoginPage /> },
      { path: 'auth/registro', element: <RegisterPage /> },
      { path: 'dashboard', element: <DashboardPage /> },
      { path: 'sample/:slug', element: <SamplePage /> },
      { path: 'perfil/:username', element: <PerfilPage /> },
      { path: 'blog', element: <BlogPage /> },
      { path: 'blog/:slug', element: <BlogPage /> },
      { path: '*', element: <NotFoundPage /> },
    ],
  },
]);

export function AppRouter() {
  return <RouterProvider router={router} />;
}

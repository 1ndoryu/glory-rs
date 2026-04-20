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
import { createBrowserRouter, Outlet, RouterProvider, Link } from 'react-router-dom';

const HomePage = lazy(() => import('./pages/HomePage'));
const SamplePage = lazy(() => import('./pages/SamplePage'));
const PerfilPage = lazy(() => import('./pages/PerfilPage'));
const BlogPage = lazy(() => import('./pages/BlogPage'));
const NotFoundPage = lazy(() => import('./pages/NotFoundPage'));

function Layout() {
  return (
    <div className="aplicacion">
      <header className="cabecera">
        <Link to="/" className="logo">Kamples</Link>
        <nav className="navegacion">
          <Link to="/" className="enlace">Inicio</Link>
          <Link to="/blog" className="enlace">Blog</Link>
          <a className="enlace" href="/swagger-ui/" target="_blank" rel="noopener noreferrer">API</a>
        </nav>
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

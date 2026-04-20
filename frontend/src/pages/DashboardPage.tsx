/* [204A-1] Dashboard mínimo de la SPA.
 * Se apoya en `/api/users/me` y `/api/dashboard/stats` ya expuestos por Axum.
 * Si el creador todavía no tiene panel inicializado devolvemos un estado vacío útil,
 * en lugar de reciclar el dashboard legacy excluido del type-check actual. */

import axios from 'axios';
import { Link } from 'react-router-dom';
import { useStats } from '../api/generated/payments/payments';
import { useAuth } from '../hooks/useAuth';

const compactFormatter = new Intl.NumberFormat('es-ES', {
  maximumFractionDigits: 1,
  notation: 'compact',
});

const currencyFormatter = new Intl.NumberFormat('es-ES', {
  currency: 'EUR',
  maximumFractionDigits: 0,
  style: 'currency',
});

function getErrorMessage(error: unknown): string {
  if (axios.isAxiosError(error)) {
    const apiMessage = (error.response?.data as { message?: string } | undefined)?.message;
    return apiMessage ?? error.message;
  }

  if (error instanceof Error) {
    return error.message;
  }

  return 'No se pudieron cargar tus métricas.';
}

export default function DashboardPage() {
  const auth = useAuth();
  const statsQuery = useStats({
    query: {
      enabled: auth.isAuthenticated,
      refetchOnWindowFocus: false,
      retry: false,
    },
  });

  const stats = statsQuery.data?.status === 200 ? statsQuery.data.data : undefined;
  const isCreatorPending = axios.isAxiosError(statsQuery.error)
    && statsQuery.error.response?.status === 404;

  if (auth.isLoading) {
    return (
      <section className="dashboardPagina">
        <div className="panelEstado">
          <p className="estadoNeutral">Cargando tu panel…</p>
        </div>
      </section>
    );
  }

  if (!auth.isAuthenticated || !auth.user) {
    return (
      <section className="dashboardPagina">
        <div className="dashboardHero dashboardHeroVacio">
          <p className="heroEyebrow">Dashboard mínimo</p>
          <h1>Necesitas iniciar sesión para ver tu panel.</h1>
          <p>
            La migración ya soporta sesión real con JWT. Entra o crea una cuenta para
            habilitar tu vista personal y el feed autenticado.
          </p>
          <div className="heroAcciones">
            <Link className="accionPrimaria" to="/auth/login">Entrar</Link>
            <Link className="accionSecundaria" to="/auth/registro">Crear cuenta</Link>
          </div>
        </div>
      </section>
    );
  }

  return (
    <section className="dashboardPagina">
      <header className="dashboardHero">
        <div>
          <p className="heroEyebrow">Panel del creador</p>
          <h1>
            Hola, {auth.user.nombre_visible || auth.user.username}
          </h1>
          <p>
            @{auth.user.username} · plan {auth.user.plan} · estado {auth.user.estado}
          </p>
        </div>
        <div className="dashboardResumenUsuario">
          <span>Samples: {compactFormatter.format(auth.user.total_samples)}</span>
          <span>Seguidores: {compactFormatter.format(auth.user.total_seguidores)}</span>
          <span>Descargas: {compactFormatter.format(auth.user.total_descargas)}</span>
        </div>
      </header>

      {statsQuery.isLoading && (
        <div className="panelEstado">
          <p className="estadoNeutral">Consultando métricas del creador…</p>
        </div>
      )}

      {stats && (
        <div className="dashboardGrid">
          <article className="statCard">
            <span className="statLabel">Ingresos del mes</span>
            <strong>{currencyFormatter.format(stats.ingresosMes)}</strong>
          </article>
          <article className="statCard">
            <span className="statLabel">Descargas del mes</span>
            <strong>{compactFormatter.format(stats.descargasMes)}</strong>
          </article>
          <article className="statCard">
            <span className="statLabel">Reproducciones totales</span>
            <strong>{compactFormatter.format(stats.reproduccionesTotal)}</strong>
          </article>
          <article className="statCard">
            <span className="statLabel">Seguidores totales</span>
            <strong>{compactFormatter.format(stats.seguidoresTotal)}</strong>
          </article>
          <article className="statCard">
            <span className="statLabel">Ingresos totales</span>
            <strong>{currencyFormatter.format(stats.ingresosTotal)}</strong>
          </article>
          <article className="statCard">
            <span className="statLabel">Samples publicados</span>
            <strong>{compactFormatter.format(stats.samplesPublicados)}</strong>
          </article>
        </div>
      )}

      {isCreatorPending && (
        <div className="panelEstado">
          <p className="estadoNeutral">
            Tu cuenta ya está autenticada, pero el panel del creador todavía no tiene
            datos iniciales. Esta SPA ya dejó la ruta lista para cuando el backend los sirva.
          </p>
        </div>
      )}

      {statsQuery.error && !isCreatorPending && (
        <div className="panelEstado">
          <p className="estadoError">{getErrorMessage(statsQuery.error)}</p>
        </div>
      )}
    </section>
  );
}

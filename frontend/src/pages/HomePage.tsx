/* [204A-1] HomePage deja de ser un placeholder y pasa a ser la portada real de la SPA.
 * Reusa los contratos ya migrados a Orval (health, feed público, feed autenticado)
 * y reescribe la composición de UI porque las islands importadas siguen fuera del
 * type-check actual por depender de aliases legacy. */

import { Link } from 'react-router-dom';
import { useGetMeFeed } from '../api/generated/feed/feed';
import { useHealthCheck } from '../api/generated/health/health';
import type { RankedSample, SampleSummary } from '../api/generated/model';
import { useListSamples } from '../api/generated/sample-catalog/sample-catalog';
import { useAuth } from '../hooks/useAuth';

const compactFormatter = new Intl.NumberFormat('es-ES', {
  maximumFractionDigits: 1,
  notation: 'compact',
});

function formatDate(value?: string | null): string {
  if (!value) {
    return 'Publicación reciente';
  }

  const parsed = new Date(value);
  if (Number.isNaN(parsed.getTime())) {
    return 'Publicación reciente';
  }

  return parsed.toLocaleDateString('es-ES', {
    day: '2-digit',
    month: 'short',
    year: 'numeric',
  });
}

type SampleCardData = {
  bpm?: number | null;
  creatorLabel?: string;
  esNuevo: boolean;
  id: number;
  publicadoAt?: string | null;
  slug: string;
  tags: string[];
  tipo: string;
  titulo: string;
  tonalidad?: string | null;
  totalDescargas: number;
  totalLikes: number;
  totalReproducciones: number;
};

function normalizeFeedItem(sample: RankedSample): SampleCardData {
  return {
    bpm: sample.bpm,
    esNuevo: sample.es_nuevo,
    id: sample.id,
    publicadoAt: sample.publicado_at,
    slug: sample.slug,
    tags: sample.tags,
    tipo: sample.tipo,
    titulo: sample.titulo,
    tonalidad: sample.escala ?? sample.key,
    totalDescargas: sample.total_descargas,
    totalLikes: sample.total_likes,
    totalReproducciones: sample.total_reproducciones,
  };
}

function normalizeCatalogItem(sample: SampleSummary): SampleCardData {
  return {
    bpm: sample.bpm,
    creatorLabel: sample.creador.nombre_visible ?? sample.creador.username,
    esNuevo: false,
    id: sample.id,
    publicadoAt: sample.publicado_at,
    slug: sample.slug,
    tags: sample.tags,
    tipo: sample.tipo,
    titulo: sample.titulo,
    tonalidad: sample.escala ?? sample.key,
    totalDescargas: sample.total_descargas,
    totalLikes: sample.total_likes,
    totalReproducciones: sample.total_reproducciones,
  };
}

function SampleCard({ sample }: { sample: SampleCardData }) {
  return (
    <article className="sampleCard">
      <div className="sampleCabecera">
        <div>
          <p className="sampleTipo">{sample.tipo}</p>
          <h3>
            <Link className="sampleTitulo" to={`/sample/${sample.slug}`}>
              {sample.titulo}
            </Link>
          </h3>
        </div>
        {sample.esNuevo && <span className="badgeDestacado">Nuevo</span>}
      </div>

      <p className="sampleMeta">
        {formatDate(sample.publicadoAt)} · {sample.bpm ?? '---'} BPM · {sample.tonalidad ?? 'Sin tonalidad'}
      </p>

      {sample.creatorLabel && <p className="sampleMeta">por {sample.creatorLabel}</p>}

      <div className="sampleStats">
        <span>{compactFormatter.format(sample.totalReproducciones)} plays</span>
        <span>{compactFormatter.format(sample.totalLikes)} likes</span>
        <span>{compactFormatter.format(sample.totalDescargas)} descargas</span>
      </div>

      <div className="sampleEtiquetas">
        {sample.tags.slice(0, 4).map((tag) => (
          <span key={tag} className="sampleTag">#{tag}</span>
        ))}
      </div>
    </article>
  );
}

export default function HomePage() {
  const auth = useAuth();
  const health = useHealthCheck({
    query: { staleTime: 30_000, refetchOnWindowFocus: false },
  });
  const publicCatalog = useListSamples(
    { page: 1, per_page: 8 },
    { query: { staleTime: 60_000, refetchOnWindowFocus: false } },
  );
  const personalFeed = useGetMeFeed(
    { limit: 8, offset: 0 },
    {
      query: {
        enabled: auth.isAuthenticated,
        refetchOnWindowFocus: false,
        retry: false,
        staleTime: 60_000,
      },
    },
  );

  const version = (health.data as { data?: { version?: string } } | undefined)?.data?.version ?? '?';
  const samples: SampleCardData[] = auth.isAuthenticated
    ? (personalFeed.data?.status === 200 ? personalFeed.data.data.items.map(normalizeFeedItem) : [])
    : (publicCatalog.data?.status === 200 ? publicCatalog.data.data.data.map(normalizeCatalogItem) : []);
  const isFeedLoading = auth.isAuthenticated ? personalFeed.isLoading : publicCatalog.isLoading;
  const isFeedError = auth.isAuthenticated ? personalFeed.isError : publicCatalog.isError;

  return (
    <section className="paginaInicio">
      <div className="heroInicio">
        <div className="heroContenido">
          <p className="heroEyebrow">Kamples · Rust + React + Orval</p>
          <h1 className="heroTitulo">Una portada SPA real para descubrir, entrar y operar.</h1>
          <p className="heroDescripcion">
            El placeholder desaparece: ahora la home consume el feed real, enlaza a auth
            y deja listo un dashboard mínimo para cuentas autenticadas.
          </p>

          <div className="heroAcciones">
            {auth.isAuthenticated && auth.user ? (
              <>
                <Link className="accionPrimaria" to="/dashboard">Ir a mi dashboard</Link>
                <Link className="accionSecundaria" to={`/perfil/${auth.user.username}`}>
                  Ver mi perfil
                </Link>
              </>
            ) : (
              <>
                <Link className="accionPrimaria" to="/auth/registro">Crear cuenta</Link>
                <Link className="accionSecundaria" to="/auth/login">Iniciar sesión</Link>
              </>
            )}
          </div>
        </div>

        <aside className="heroPanel">
          <div className="panelEstado">
            <span className={health.isError ? 'estadoError' : health.data ? 'estadoOk' : 'estadoNeutral'}>
              {health.isLoading && 'Comprobando backend…'}
              {health.isError && 'Backend no disponible'}
              {health.data && `Backend OK · v${version}`}
            </span>
          </div>

          {auth.isAuthenticated && auth.user ? (
            <div className="bloqueResumen">
              <p className="bloqueResumenTitulo">Tu resumen</p>
              <strong>{auth.user.nombre_visible || auth.user.username}</strong>
              <span>@{auth.user.username}</span>
              <div className="resumenMetrico">
                <span>{compactFormatter.format(auth.user.total_samples)} samples</span>
                <span>{compactFormatter.format(auth.user.total_seguidores)} seguidores</span>
                <span>{compactFormatter.format(auth.user.total_descargas)} descargas</span>
              </div>
            </div>
          ) : (
            <div className="bloqueResumen">
              <p className="bloqueResumenTitulo">Qué entra en esta fase</p>
              <ul className="listaSimple">
                <li>Feed público ya conectado al backend Rust.</li>
                <li>Rutas nuevas de login y registro SPA.</li>
                <li>Dashboard mínimo listo para cuentas autenticadas.</li>
              </ul>
            </div>
          )}
        </aside>
      </div>

      <div className="feedSeccion">
        <div className="feedCabecera">
          <div>
            <p className="heroEyebrow">Feed</p>
            <h2>{auth.isAuthenticated ? 'Tu feed autenticado' : 'Catálogo público destacado'}</h2>
          </div>
          {auth.isAuthenticated ? (
            <Link className="accionSecundaria" to="/dashboard">Abrir panel</Link>
          ) : (
            <Link className="accionSecundaria" to="/auth/login">Personalizar mi feed</Link>
          )}
        </div>

        {isFeedLoading && (
          <div className="panelEstado">
            <p className="estadoNeutral">Cargando samples destacados…</p>
          </div>
        )}

        {isFeedError && (
          <div className="panelEstado">
            <p className="estadoError">No se pudo cargar el feed en este momento.</p>
          </div>
        )}

        {!isFeedLoading && !isFeedError && (
          <div className="feedGrid">
            {samples.map((sample) => (
              <SampleCard key={sample.id} sample={sample} />
            ))}
          </div>
        )}
      </div>
    </section>
  );
}

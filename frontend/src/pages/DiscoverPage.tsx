import { Link, useSearchParams } from 'react-router-dom';
import { useAuthModalStore } from '../app/stores/authModalStore';
import { useGetMeFeed } from '../api/generated/feed/feed';
import { useHealthCheck } from '../api/generated/health/health';
import type { RankedSample, SampleSummary } from '../api/generated/model';
import { useListSamples } from '../api/generated/sample-catalog/sample-catalog';
import Boton from '../components/ui/Boton';
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

export default function DiscoverPage() {
  const auth = useAuth();
  const abrirAuth = useAuthModalStore((state) => state.abrir);
  const [searchParams] = useSearchParams();
  const searchTerm = searchParams.get('buscar')?.trim() ?? '';
  const health = useHealthCheck({
    query: { staleTime: 30_000, refetchOnWindowFocus: false },
  });
  const publicCatalog = useListSamples(
    { page: 1, per_page: 8, search: searchTerm || undefined },
    { query: { staleTime: 60_000, refetchOnWindowFocus: false } },
  );
  const personalFeed = useGetMeFeed(
    { limit: 8, offset: 0 },
    {
      query: {
        enabled: auth.isAuthenticated && !searchTerm,
        refetchOnWindowFocus: false,
        retry: false,
        staleTime: 60_000,
      },
    },
  );

  const version = (health.data as { data?: { version?: string } } | undefined)?.data?.version ?? '?';
  const usePersonalFeed = auth.isAuthenticated && !searchTerm;
  const samples: SampleCardData[] = usePersonalFeed
    ? (personalFeed.data?.status === 200 ? personalFeed.data.data.items.map(normalizeFeedItem) : [])
    : (publicCatalog.data?.status === 200 ? publicCatalog.data.data.data.map(normalizeCatalogItem) : []);
  const isFeedLoading = usePersonalFeed ? personalFeed.isLoading : publicCatalog.isLoading;
  const isFeedError = usePersonalFeed ? personalFeed.isError : publicCatalog.isError;
  const title = searchTerm
    ? `Resultados para “${searchTerm}”`
    : usePersonalFeed
      ? 'Tu feed autenticado'
      : 'Catálogo público destacado';
  const description = searchTerm
    ? 'La portada pública original ahora vive en / y el catálogo queda separado para explorar y buscar.'
    : usePersonalFeed
      ? 'Tu inicio autenticado sigue mostrando el feed conectado al backend Rust.'
      : 'Explora el catálogo público sin mezclarlo con la portada principal deslogueada.';

  return (
    <section className="paginaInicio">
      <div className="heroInicio">
        <div className="heroContenido">
          <p className="heroEyebrow">Descubrir · Kamples</p>
          <h1 className="heroTitulo">{title}</h1>
          <p className="heroDescripcion">{description}</p>

          <div className="heroAcciones">
            {!auth.isAuthenticated && <Boton className="accionPrimaria" onClick={() => abrirAuth('registro')} type="button">Crear cuenta</Boton>}
            <Link className="accionSecundaria" to="/">Volver a portada</Link>
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
              <p className="bloqueResumenTitulo">Exploración pública</p>
              <ul className="listaSimple">
                <li>Busca por término desde la portada original.</li>
                <li>Explora el catálogo sin iniciar sesión.</li>
                <li>La cuenta desbloquea feed personalizado y dashboard.</li>
              </ul>
            </div>
          )}
        </aside>
      </div>

      <div className="feedSeccion">
        <div className="feedCabecera">
          <div>
            <p className="heroEyebrow">Catálogo</p>
            <h2>{title}</h2>
          </div>
          {!auth.isAuthenticated && <Boton className="accionSecundaria" onClick={() => abrirAuth('login')} type="button">Personalizar mi feed</Boton>}
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
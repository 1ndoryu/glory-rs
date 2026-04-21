import { useAuth } from '../hooks/useAuth';
import LandingPublica from '../components/public/LandingPublica';
import DiscoverPage from './DiscoverPage';

export default function HomePage() {
  const auth = useAuth();

  if (auth.isLoading) {
    return (
      <div className="panelEstado">
        <span className="estadoNeutral">Cargando inicio…</span>
      </div>
    );
  }

  if (auth.isAuthenticated) {
    return <DiscoverPage />;
  }

  return <LandingPublica />;
}

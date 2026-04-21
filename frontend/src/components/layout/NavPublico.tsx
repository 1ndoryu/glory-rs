import { Search, X } from 'lucide-react';
import { Link } from 'react-router-dom';
import { BotonBase } from '../../app/components/ui/BotonBase';
import SelectorIdioma from '../../app/components/ui/SelectorIdioma';
import { useAuthModalStore } from '../../app/stores/authModalStore';
import { useT } from '../../app/utils/i18n/useT';
import Input from '../../app/components/ui/Input';
import LogoKamples from '../ui/LogoKamples';
import { useNavPublico } from './useNavPublico';

export default function NavPublico(): JSX.Element {
  const { clearSearch, searchValue, showSearch, updateSearch } = useNavPublico();
  const abrirAuth = useAuthModalStore((state) => state.abrir);
  const { t } = useT();

  return (
    <nav className="navPublico">
      <div className="navPublicoIzquierda">
        <Link to="/" className="navPublicoLogo">
          <LogoKamples tamano={22} />
        </Link>
        <div className="navPublicoEnlaces">
          <Link to="/descubrir" className="navPublicoEnlace">{t('nav.explorar')}</Link>
          <Link to="/colecciones" className="navPublicoEnlace">{t('nav.colecciones')}</Link>
          <Link to="/musica" className="navPublicoEnlace">{t('nav.musica')}</Link>
          <Link to="/blog" className="navPublicoEnlace">{t('nav.blog')}</Link>
        </div>
      </div>
      <div className="navPublicoDerecha">
        {showSearch && (
          <div className="navPublicoBuscador">
            <Search size={14} className="navPublicoBuscadorIcono" />
            <Input
              type="text"
              className="navPublicoBuscadorInput"
              placeholder={t('nav.buscar')}
              value={searchValue}
              onChange={(event) => updateSearch(event.target.value)}
              aria-label={t('nav.buscar')}
            />
            {searchValue && (
              <BotonBase
                variante="ghost"
                tamano="sm"
                className="navPublicoBuscadorLimpiar"
                onClick={clearSearch}
                aria-label={t('nav.limpiarBusqueda')}
                type="button"
              >
                <X size={12} />
              </BotonBase>
            )}
          </div>
        )}
        <SelectorIdioma variante="select" className="navPublicoIdioma" />
        <BotonBase variante="ghost" tamano="md" type="button" onClick={() => abrirAuth('login')}>
          {t('nav.iniciarSesion')}
        </BotonBase>
        <BotonBase variante="primario" tamano="md" type="button" onClick={() => abrirAuth('registro')}>
          {t('nav.crearCuenta')}
        </BotonBase>
      </div>
    </nav>
  );
}

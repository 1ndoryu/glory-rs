import { Download, Search } from 'lucide-react';
import { type KeyboardEvent, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { BotonBase } from '../../app/components/ui/BotonBase';
import SelectorIdioma from '../../app/components/ui/SelectorIdioma';
import { useAuthModalStore } from '../../app/stores/authModalStore';
import { useT } from '../../app/utils/i18n/useT';
import Input from '../../app/components/ui/Input';

export default function LandingPublica(): JSX.Element {
  const navigate = useNavigate();
  const [searchValue, setSearchValue] = useState('');
  const abrirAuth = useAuthModalStore((state) => state.abrir);
  const { t } = useT();

  const sections = [
    {
      alt: t('landing.img.sync'),
      image: '/landing/Sync.svg',
      subtitle: t('landing.seccion.sync.subtitulo'),
      title: t('landing.seccion.sync.titulo'),
    },
    {
      alt: t('landing.img.daw'),
      image: '/landing/MiniDaw.svg',
      subtitle: t('landing.seccion.daw.subtitulo'),
      title: t('landing.seccion.daw.titulo'),
    },
    {
      alt: t('landing.img.catalogo'),
      image: '/landing/Rolas.svg',
      subtitle: t('landing.seccion.catalogo.subtitulo'),
      title: t('landing.seccion.catalogo.titulo'),
    },
  ];

  const goToDiscover = () => {
    const query = searchValue.trim();
    navigate(query ? `/descubrir?buscar=${encodeURIComponent(query)}` : '/descubrir');
  };

  const onKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') {
      goToDiscover();
    }
  };

  return (
    <div className="landingPublica" id="landingPublica">
      <section className="landingHero">
        <h1 className="landingHeroTitulo">
          {t('landing.titulo')}
          <span className="landingHeroResaltado">{t('landing.tituloResaltado')}</span>
        </h1>
        <p className="landingHeroDescripcion">
          {t('landing.descripcion')}
        </p>
        <div className="landingHeroBuscador">
          <Search size={18} className="landingHeroBuscadorIcono" />
          <Input
            type="text"
            className="landingHeroBuscadorInput"
            placeholder={t('landing.buscarPlaceholder')}
            value={searchValue}
            onChange={(event) => setSearchValue(event.target.value)}
            onKeyDown={onKeyDown}
            aria-label={t('landing.buscar')}
          />
          <BotonBase variante="primario" tamano="sm" onClick={goToDiscover} type="button">
            {t('landing.buscar')}
          </BotonBase>
        </div>
        <div className="landingHeroAcciones">
          <BotonBase variante="secundario" tamano="md" type="button" onClick={() => abrirAuth('registro')}>
            {t('auth.crearCuentaGratis')}
          </BotonBase>
          <a className="landingHeroDescargarEnlace" href="/descargar">
            <BotonBase variante="primario" tamano="md" type="button">
              <Download size={16} />
              {t('landing.descargarApp')}
            </BotonBase>
          </a>
        </div>
      </section>

      <section className="landingHeroVisual">
        <img
          src="/landing/Kamples.svg"
          alt={t('landing.img.kamples')}
          className="landingSeccionSync"
          width={1288}
          height={717}
          fetchPriority="high"
          decoding="async"
        />
      </section>

      {sections.map((section) => (
        <section key={section.title} className="seccionSync seccionEstandar">
          <div>
            <h2 className="titleSeccion">{section.title}</h2>
            <span className="subtitleSeccion">{section.subtitle}</span>
          </div>
          <img
            src={section.image}
            alt={section.alt}
            className="landingSeccionSync"
            width={1288}
            height={717}
            loading="lazy"
            decoding="async"
          />
        </section>
      ))}

      <footer className="landingFooter">
        <p className="landingFooterTexto">
          {t('landing.footer.producto')}{' '}
          <a href="https://nakomi.studio" target="_blank" rel="noopener noreferrer" className="landingFooterEnlace">
            Nakomi.studio
          </a>
        </p>
        <nav className="landingFooterNav">
          <a href="/privacy/" className="landingFooterNavEnlace">{t('landing.footer.privacidad')}</a>
          <a href="/terms/" className="landingFooterNavEnlace">{t('landing.footer.terminos')}</a>
        </nav>
        <SelectorIdioma variante="select" className="landingFooterIdioma" />
      </footer>
    </div>
  );
}

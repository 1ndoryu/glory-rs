/* [044A-28] Página 404 — Not Found.
 * Reemplaza el catch-all silencioso que renderizaba la home. */
import {useTranslation} from 'react-i18next';
import '../styles/variables.css';
import './NotFoundIsland.css';
import {LayoutPagina} from '../components/layout/LayoutPagina';
import {SEOHead} from '../components/seo/SEOHead';

export const NotFoundIsland = (): JSX.Element => {
    const {t} = useTranslation();

    return (
        <LayoutPagina className="notFoundMain">
            <SEOHead title="404" noindex />
            <section className="notFoundHero">
                <div className="notFoundContenido">
                    <span className="notFoundCodigo">404</span>
                    <h1 className="notFoundTitulo">{t('error.not_found_title', 'Página no encontrada')}</h1>
                    <p className="notFoundDescripcion">{t('error.not_found_desc', 'La página que buscas no existe o ha sido movida.')}</p>
                    <a href="/" className="notFoundEnlace">{t('error.go_home', 'Volver al inicio')}</a>
                </div>
            </section>
        </LayoutPagina>
    );
};

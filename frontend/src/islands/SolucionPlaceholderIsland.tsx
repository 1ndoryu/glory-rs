/**
 * Componente: SolucionPlaceholderIsland
 * Página placeholder "En construcción" para sub-páginas de soluciones.
 * Se reutiliza para /soluciones/hosting, /soluciones/vps, /soluciones/agentes-ia.
 */
import {useTranslation} from 'react-i18next';
import {spaClick} from '../navegacionSPA';
import '../styles/variables.css';
import './SolucionPlaceholderIsland.css';
import {LayoutPagina} from '../components/layout/LayoutPagina';
import {SEOHead} from '../components/seo/SEOHead';
import {SeccionContacto} from '../components/home/SeccionContacto';

interface SolucionPlaceholderIslandProps {
    titulo?: string;
    descripcion?: string;
}

export const SolucionPlaceholderIsland = ({
    titulo,
    descripcion
}: SolucionPlaceholderIslandProps): JSX.Element => {
    const {t} = useTranslation();

    return (
        <LayoutPagina className="placeholderMain" id="paginaPlaceholder">
            <SEOHead
                title={titulo}
                description={descripcion}
                path="/soluciones"
            />
            <section className="placeholderHero">
                <div className="placeholderContenido">
                    <span className="placeholderEtiqueta">{t('solutions_page.under_construction')}</span>
                    <h1 className="placeholderTitulo">{titulo || t('solutions_page.coming_soon')}</h1>
                    <p className="placeholderDescripcion">{descripcion || t('solutions_page.coming_soon_desc')}</p>
                    <div className="placeholderBotones">
                        <a href="/soluciones" className="placeholderBoton" onClick={e => spaClick(e, '/soluciones')}>
                            {t('solutions_page.back_solutions')}
                        </a>
                        <a href="#contacto" className="placeholderBotonContacto">
                            {t('solutions_page.contact_info')}
                        </a>
                    </div>
                </div>
            </section>
            <SeccionContacto />
        </LayoutPagina>
    );
};

export default SolucionPlaceholderIsland;

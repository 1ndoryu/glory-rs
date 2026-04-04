/**
 * Componente: ServiciosIsland
 * Página completa de Servicios con grid filtrable.
 * Categorías centralizadas en data/navegacion.ts (DRY).
 */
import '../styles/variables.css';
import {useTranslation} from 'react-i18next';
import './ServiciosIsland.css';
import {LayoutPagina} from '../components/layout/LayoutPagina';
import {SEOHead} from '../components/seo/SEOHead';
import {BarraFiltros} from '../components/servicios/BarraFiltros';
import {GridServicios} from '../components/servicios/GridServicios';
import {useServicios} from '../hooks/useServicios';
import {CATEGORIAS_SERVICIOS} from '../data/navegacion';

interface ServiciosIslandProps {
    titulo?: string;
}

export const ServiciosIsland = ({titulo}: ServiciosIslandProps): JSX.Element => {
    const {t} = useTranslation();
    const {categoriaActiva, setCategoriaActiva, busqueda, setBusqueda, serviciosFiltrados} = useServicios();

    return (
        <LayoutPagina className="serviciosMain" id="paginaServicios">
            <SEOHead
                title="Servicios"
                description="Descubre nuestros servicios de desarrollo web, diseño UI/UX y soluciones digitales a medida."
                path="/servicios"
            />
            <section className="serviciosHero">
                <div className="heroContenido">
                    <div>
                        <h1 className="heroTitulo">{titulo || t('services_page.title')}</h1>
                    </div>

                    <div className="heroDescripcion">
                        <p>{t('services_page.description')}</p>
                    </div>
                </div>
            </section>

            <section className="serviciosContenido">
                <div className="serviciosContenedor">
                    <BarraFiltros categorias={CATEGORIAS_SERVICIOS} categoriaActiva={categoriaActiva} busqueda={busqueda} onCategoriaChange={setCategoriaActiva} onBusquedaChange={setBusqueda} />
                    <GridServicios servicios={serviciosFiltrados} />
                </div>
            </section>
        </LayoutPagina>
    );
};

export default ServiciosIsland;

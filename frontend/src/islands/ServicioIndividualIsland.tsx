/**
 * Componente: ServicioIndividualIsland
 * Página de detalle de un servicio individual.
 * Estructura: Hero -> Galeria -> CTA -> Skills -> Relacionados -> Contacto -> Footer
 * Skills y datos centralizados en data/ (DRY).
 */
import {useTranslation} from 'react-i18next';
import '../styles/variables.css';
import './ServicioIndividualIsland.css';
import {LayoutPagina} from '../components/layout/LayoutPagina';
import {SeccionHeroServicio} from '../components/servicios/SeccionHeroServicio';
import {SeccionGaleriaServicio} from '../components/servicios/SeccionGaleriaServicio';
import {SeccionSkillsServicio} from '../components/servicios/SeccionSkillsServicio';
import {SeccionServiciosRelacionados} from '../components/servicios/SeccionServiciosRelacionados';
import {SeccionCta} from '../components/ui/SeccionCta';
import {SeccionPlanesServicio} from '../components/servicios/SeccionPlanesServicio';
import {SeccionContacto} from '../components/home/SeccionContacto';
import {SKILLS_POR_DEFECTO} from '../data/skills';
import {obtenerPlanesServicio} from '../data/planes/index';

interface ServicioIndividualIslandProps {
    titulo?: string;
    descripcion?: string;
    precio_desde?: string;
    slug?: string;
}

export const ServicioIndividualIsland = ({titulo, descripcion, precio_desde, slug}: ServicioIndividualIslandProps): JSX.Element => {
    const {t} = useTranslation();

    /* ID del servicio actual para excluirlo de relacionados */
    const servicioId = slug || titulo?.toLowerCase().replace(/\s+/g, '-') || '';

    /* [044A-11] Obtener precio mínimo real de los planes del servicio en lugar de hardcodear $997.
     * Si no hay planes o precio_desde viene por prop, usa ese. Sino busca el menor precio numérico. */
    const datosPlanes = obtenerPlanesServicio(servicioId);
    const precioMinimo = (() => {
        if (precio_desde) return precio_desde;
        if (!datosPlanes || datosPlanes.planes.length === 0) return '';
        const precios = datosPlanes.planes
            .filter(p => !p.esPersonalizado)
            .map(p => {
                const num = parseFloat(p.precio.replace(/[^0-9.]/g, ''));
                return isNaN(num) ? Infinity : num;
            })
            .filter(n => n !== Infinity);
        if (precios.length === 0) return '';
        return `$${Math.min(...precios)}`;
    })();

    return (
        <LayoutPagina className="servicioIndividualMain">
            <SeccionHeroServicio titulo={titulo} descripcion={descripcion} />
            <SeccionGaleriaServicio />
            <SeccionPlanesServicio slug={servicioId} />
            <SeccionCta descripcion={[t('service_detail.cta_1'), t('service_detail.cta_2')]} textoBotonPrimario={precioMinimo ? `${t('service_detail.cta_hire')} ${precioMinimo}` : t('service_detail.cta_hire')} linkBotonPrimario="/contacto/" textoBotonSecundario={t('service_detail.cta_contact')} linkBotonSecundario="/contacto/" />
            <SeccionSkillsServicio skills={SKILLS_POR_DEFECTO} />
            <SeccionServiciosRelacionados servicioActualId={servicioId} />
            <SeccionContacto />
        </LayoutPagina>
    );
};

export default ServicioIndividualIsland;

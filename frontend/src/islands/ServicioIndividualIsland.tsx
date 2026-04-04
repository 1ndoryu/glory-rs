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

    return (
        <LayoutPagina className="servicioIndividualMain">
            <SeccionHeroServicio titulo={titulo} descripcion={descripcion} />
            <SeccionGaleriaServicio />
            <SeccionPlanesServicio slug={servicioId} />
            <SeccionCta descripcion={[t('service_detail.cta_1'), t('service_detail.cta_2')]} textoBotonPrimario={`${t('service_detail.cta_hire')} ${precio_desde || '$997'}`} linkBotonPrimario="/contacto/" textoBotonSecundario={t('service_detail.cta_contact')} linkBotonSecundario="/contacto/" />
            <SeccionSkillsServicio skills={SKILLS_POR_DEFECTO} />
            <SeccionServiciosRelacionados servicioActualId={servicioId} />
            <SeccionContacto />
        </LayoutPagina>
    );
};

export default ServicioIndividualIsland;

/**
 * Componente: ServicioIndividualIsland
 * Página de detalle de un servicio individual.
 * Estructura: Hero -> Galeria -> CTA -> Skills -> Relacionados -> Contacto -> Footer
 * Skills y datos centralizados en data/ (DRY).
 * [044A-38 Fase 1] CTA "Contratar" apunta a /panel si logueado (Fase 2 conecta con API de órdenes).
 */
import {useTranslation} from 'react-i18next';
import {useAuthStore} from '../stores/authStore';
import {useChatStore} from '../stores/chatStore';
import '../styles/variables.css';
import './ServicioIndividualIsland.css';
import {LayoutPagina} from '../components/layout/LayoutPagina';
import {SEOHead} from '../components/seo/SEOHead';
import {serviceSchema} from '../components/seo/schemas';
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
    const logueado = useAuthStore(s => s.logueado);
    const abrirChat = useChatStore(s => s.abrir);

    /* [064A-5] Si logueado, CTA lleva al panel. Si no, abre el chat. */
    const ctaLink = logueado ? '/panel' : undefined;

    /* ID del servicio actual para excluirlo de relacionados */
    const servicioId = slug || titulo?.toLowerCase().replace(/\s+/g, '-') || '';

    /* [044A-40] Obtener precio mínimo real de los planes del servicio. */
    const datosPlanes = obtenerPlanesServicio(servicioId);
    const precioMinimo = (() => {
        if (precio_desde) return precio_desde;
        if (!datosPlanes || datosPlanes.planes.length === 0) return '';
        const precios = datosPlanes.planes
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
            <SEOHead
                title={titulo}
                description={descripcion}
                path={`/servicios/${slug || ''}`}
                jsonLd={titulo && descripcion && slug ? serviceSchema(titulo, descripcion, slug) : undefined}
            />
            <SeccionHeroServicio titulo={titulo} descripcion={descripcion} />
            <SeccionGaleriaServicio />
            <SeccionPlanesServicio slug={servicioId} />
            <SeccionCta descripcion={[t('service_detail.cta_1'), t('service_detail.cta_2')]} textoBotonPrimario={precioMinimo ? `${t('service_detail.cta_hire')} ${precioMinimo}` : t('service_detail.cta_hire')} linkBotonPrimario={ctaLink} onBotonPrimarioClick={logueado ? undefined : abrirChat} textoBotonSecundario={t('service_detail.cta_contact')} onBotonSecundarioClick={abrirChat} />
            <SeccionSkillsServicio skills={SKILLS_POR_DEFECTO} />
            <SeccionServiciosRelacionados servicioActualId={servicioId} />
            <SeccionContacto />
        </LayoutPagina>
    );
};

export default ServicioIndividualIsland;

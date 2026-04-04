import {Servicio} from '../types/servicios';
import {obtenerImagen} from '../hooks/useImagenes';

/* [044A-5] Textos revisados para coherencia con el perfil de Nakomi Studio. */

const SERVICIOS_FALLBACK: Servicio[] = [
    {
        id: '1',
        titulo: 'Diseño de Sitios Web',
        descripcion: 'Sitios web a medida con diseño original, rendimiento optimizado y código limpio. Desde landing pages hasta plataformas complejas.',
        imagen: obtenerImagen(0),
        categorias: ['web', 'branding'],
        link: '/servicios/diseno-web',
        skills: [
            {id: 1, titulo: 'Responsive Design', descripcion: 'Diseño adaptativo para todos los dispositivos.'},
            {id: 2, titulo: 'Performance', descripcion: 'Optimización de velocidad de carga y Core Web Vitals.'},
            {id: 3, titulo: 'SEO On-page', descripcion: 'Estructura semántica y meta tags optimizados.'},
            {id: 4, titulo: 'CMS Integration', descripcion: 'Integración con WordPress, headless CMS o soluciones propias.'},
        ]
    },
    {
        id: '2',
        titulo: 'Desarrollo de Aplicaciones',
        descripcion: 'Software a medida: apps web, móviles y de escritorio. Arquitectura sólida, APIs robustas y experiencia de usuario cuidada.',
        imagen: obtenerImagen(5),
        categorias: ['software'],
        link: '/servicios/desarrollo-apps',
        skills: [
            {id: 1, titulo: 'React / React Native', descripcion: 'Frontend web y apps móviles cross-platform.'},
            {id: 2, titulo: 'API Development', descripcion: 'APIs REST con Rust (Axum) o Node.js.'},
            {id: 3, titulo: 'Cloud Architecture', descripcion: 'Infraestructura escalable con deploy automatizado.'},
            {id: 4, titulo: 'Testing & QA', descripcion: 'Suite de tests automatizados para confiabilidad.'},
        ]
    },
    {
        id: '3',
        titulo: 'Agentes de IA',
        descripcion: 'Asistentes inteligentes, chatbots contextuales y automatización con modelos de lenguaje. Integración con tu flujo de trabajo existente.',
        imagen: obtenerImagen(10),
        categorias: ['ai', 'software'],
        link: '/servicios/agentes-ia'
    },
    {
        id: '4',
        titulo: 'Identidad de Marca',
        descripcion: 'Identidad visual completa: logo, paleta, tipografía y guidelines. Una marca coherente que comunica tu esencia.',
        imagen: obtenerImagen(15),
        categorias: ['branding'],
        link: '/servicios/branding'
    },
    {
        id: '5',
        titulo: 'E-commerce',
        descripcion: 'Tiendas online con catálogo, checkout optimizado, pasarelas de pago y panel de gestión. Enfocadas en conversión.',
        imagen: obtenerImagen(20),
        categorias: ['web', 'software'],
        link: '/servicios/ecommerce'
    },
    {
        id: '6',
        titulo: 'Chatbots Personalizados',
        descripcion: 'Bots conversacionales con IA que entienden contexto, escalan a agentes humanos y operan 24/7.',
        imagen: obtenerImagen(25),
        categorias: ['ai'],
        link: '/servicios/chatbots'
    },
    {
        id: '7',
        titulo: 'Diseño UX/UI',
        descripcion: 'Investigación de usuarios, wireframes, prototipos y diseño final. Interfaces que se sienten naturales desde el primer click.',
        imagen: obtenerImagen(30),
        categorias: ['web', 'software'],
        link: '/servicios/diseno-ux-ui'
    },
    {
        id: '8',
        titulo: 'Automatización de Procesos',
        descripcion: 'Pipelines, integraciones y herramientas que eliminan tareas repetitivas. Más tiempo para lo que importa.',
        imagen: obtenerImagen(35),
        categorias: ['software', 'ai'],
        link: '/servicios/automatizacion'
    },
    {
        id: '9',
        titulo: 'Consultoría Digital',
        descripcion: 'Análisis técnico, auditorías de rendimiento y estrategia digital. Decisiones informadas para tu próximo paso.',
        imagen: obtenerImagen(40),
        categorias: ['branding'],
        link: '/servicios/consultoria'
    }
];

/* [044A-1] Sin GLORY_CONTEXT, datos directos */
export const SERVICIOS_DATA: Servicio[] = SERVICIOS_FALLBACK;

/* Subconjuntos derivados para uso en diferentes secciones */

/* Para la Home (primeros 6) */
export const SERVICIOS_PRINCIPALES = SERVICIOS_DATA.slice(0, 6);

/*
 * Obtener servicios relacionados excluyendo el servicio actual.
 * Si no se pasa ID, retorna 3 servicios aleatorios.
 */
export const obtenerServiciosRelacionados = (servicioActualId?: string, cantidad: number = 3): Servicio[] => {
    const filtrados = servicioActualId
        ? SERVICIOS_DATA.filter(s => String(s.id) !== String(servicioActualId))
        : SERVICIOS_DATA;

    /* Mezcla simple para variedad */
    const mezclados = [...filtrados].sort(() => Math.random() - 0.5);
    return mezclados.slice(0, cantidad);
};

/* Alias retrocompatible (sin filtro) */
export const SERVICIOS_RELACIONADOS = SERVICIOS_DATA.slice(0, 3);

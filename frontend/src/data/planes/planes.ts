/**
 * [044A-32] Planes de precios unificados.
 * [044A-40] Eliminados tiers "A medida" — ahora solo 2 tiers por servicio.
 * El botón "Conversar" reemplaza la funcionalidad personalizada.
 * Servicios activos: Web, Apps, IA, Branding, E-commerce, SEO, Marketing.
 */
import {type PlanesDeServicio, incluida, noIncluida} from './tipos';

/* ── Diseño Web ─────────────────────────────────────────── */
export const PLANES_WEB: PlanesDeServicio = {
    servicioSlug: 'diseno-web',
    servicioTitulo: 'Diseno de Sitios Web',
    planes: [
        {
            id: 'web-basico',
            nombre: 'Basico',
            precio: '$100',
            descripcion: 'Ideal para landing pages y presencia online basica.',
            ctaTexto: 'Comenzar',
            ctaLink: '/contacto/',
            caracteristicas: [
                incluida('1 pagina principal'),
                incluida('Diseno responsive'),
                incluida('Formulario de contacto'),
                incluida('Hosting 3 meses incluido'),
                noIncluida('Blog integrado'),
                noIncluida('SEO avanzado'),
                noIncluida('Panel administrable'),
            ]
        },
        {
            id: 'web-avanzado',
            nombre: 'Avanzado',
            precio: '$250',
            descripcion: 'Sitio completo con CMS, blog y optimizacion SEO.',
            ctaTexto: 'Elegir plan',
            ctaLink: '/contacto/',
            destacado: true,
            caracteristicas: [
                incluida('Hasta 5 paginas'),
                incluida('Diseno responsive premium'),
                incluida('Formulario de contacto'),
                incluida('Hosting 6 meses incluido'),
                incluida('Blog integrado'),
                incluida('SEO on-page completo'),
                incluida('Panel administrable (CMS)'),
            ]
        }
    ]
};

/* ── Desarrollo de Aplicaciones ─────────────────────────── */
export const PLANES_APPS: PlanesDeServicio = {
    servicioSlug: 'desarrollo-apps',
    servicioTitulo: 'Desarrollo de Aplicaciones',
    planes: [
        {
            id: 'apps-basico',
            nombre: 'Basico',
            precio: '$200',
            descripcion: 'MVP o aplicacion sencilla con funcionalidades core.',
            ctaTexto: 'Comenzar',
            ctaLink: '/contacto/',
            caracteristicas: [
                incluida('1 plataforma (web o movil)'),
                incluida('Hasta 3 funcionalidades'),
                incluida('Autenticacion basica'),
                incluida('API REST simple'),
                noIncluida('Panel de administracion'),
                noIncluida('Integraciones externas'),
                noIncluida('Soporte post-lanzamiento'),
            ]
        },
        {
            id: 'apps-avanzado',
            nombre: 'Avanzado',
            precio: '$500',
            descripcion: 'App completa multi-plataforma con backend robusto.',
            ctaTexto: 'Elegir plan',
            ctaLink: '/contacto/',
            destacado: true,
            caracteristicas: [
                incluida('Web + Movil (React Native)'),
                incluida('Hasta 8 funcionalidades'),
                incluida('Auth + roles de usuario'),
                incluida('API REST + GraphQL'),
                incluida('Panel de administracion'),
                incluida('2 integraciones externas'),
                incluida('1 mes soporte post-lanzamiento'),
            ]
        }
    ]
};

/* ── Agentes de IA ──────────────────────────────────────── */
export const PLANES_IA: PlanesDeServicio = {
    servicioSlug: 'agentes-ia',
    servicioTitulo: 'Agentes de IA',
    planes: [
        {
            id: 'ia-basico',
            nombre: 'Basico',
            precio: '$60',
            periodo: '/mes',
            descripcion: 'Agente simple para tareas de automatizacion basica.',
            ctaTexto: 'Comenzar',
            ctaLink: '/contacto/',
            caracteristicas: [
                incluida('1 agente configurado'),
                incluida('Hasta 1,000 interacciones/mes'),
                incluida('Integracion con 1 plataforma'),
                incluida('Respuestas predefinidas'),
                noIncluida('Aprendizaje continuo'),
                noIncluida('Analisis de datos'),
                noIncluida('API personalizada'),
            ]
        },
        {
            id: 'ia-avanzado',
            nombre: 'Avanzado',
            precio: '$300',
            periodo: '/mes',
            descripcion: 'Agente inteligente con IA generativa y analisis.',
            ctaTexto: 'Elegir plan',
            ctaLink: '/contacto/',
            destacado: true,
            caracteristicas: [
                incluida('Hasta 3 agentes'),
                incluida('Interacciones ilimitadas'),
                incluida('Multi-plataforma'),
                incluida('IA generativa (GPT/Gemini)'),
                incluida('Aprendizaje continuo'),
                incluida('Dashboard de analisis'),
                incluida('API personalizada'),
            ]
        }
    ]
};

/* ── Identidad de Marca ─────────────────────────────────── */
export const PLANES_BRANDING: PlanesDeServicio = {
    servicioSlug: 'branding',
    servicioTitulo: 'Identidad de Marca',
    planes: [
        {
            id: 'branding-basico',
            nombre: 'Basico',
            precio: '$150',
            descripcion: 'Logo y paleta de colores para tu marca.',
            ctaTexto: 'Comenzar',
            ctaLink: '/contacto/',
            caracteristicas: [
                incluida('Diseno de logo (3 propuestas)'),
                incluida('Paleta de colores'),
                incluida('Tipografia principal'),
                incluida('Archivo en formatos digitales'),
                noIncluida('Manual de marca'),
                noIncluida('Papeleria corporativa'),
                noIncluida('Redes sociales kit'),
            ]
        },
        {
            id: 'branding-avanzado',
            nombre: 'Avanzado',
            precio: '$400',
            descripcion: 'Identidad visual completa con manual de marca.',
            ctaTexto: 'Elegir plan',
            ctaLink: '/contacto/',
            destacado: true,
            caracteristicas: [
                incluida('Diseno de logo (5 propuestas)'),
                incluida('Paleta de colores extendida'),
                incluida('Sistema tipografico completo'),
                incluida('Manual de marca (PDF)'),
                incluida('Papeleria corporativa'),
                incluida('Kit redes sociales'),
                incluida('Iconografia personalizada'),
            ]
        }
    ]
};

/* ── E-commerce ─────────────────────────────────────────── */
export const PLANES_ECOMMERCE: PlanesDeServicio = {
    servicioSlug: 'ecommerce',
    servicioTitulo: 'E-commerce',
    planes: [
        {
            id: 'ecommerce-basico',
            nombre: 'Basico',
            precio: '$200',
            descripcion: 'Tienda online funcional con lo esencial para vender.',
            ctaTexto: 'Comenzar',
            ctaLink: '/contacto/',
            caracteristicas: [
                incluida('Hasta 50 productos'),
                incluida('Pasarela de pago (Stripe)'),
                incluida('Carrito de compras'),
                incluida('Diseno responsive'),
                noIncluida('Gestion de inventario'),
                noIncluida('Integraciones avanzadas'),
                noIncluida('Marketing automatizado'),
            ]
        },
        {
            id: 'ecommerce-avanzado',
            nombre: 'Avanzado',
            precio: '$500',
            descripcion: 'Tienda completa con inventario y marketing integrado.',
            ctaTexto: 'Elegir plan',
            ctaLink: '/contacto/',
            destacado: true,
            caracteristicas: [
                incluida('Productos ilimitados'),
                incluida('Multi-pasarela de pago'),
                incluida('Gestion de inventario'),
                incluida('Panel de analytics'),
                incluida('Email marketing integrado'),
                incluida('Cupones y descuentos'),
                noIncluida('Marketplace multi-vendor'),
            ]
        }
    ]
};

/* ── SEO ────────────────────────────────────────────────── */
export const PLANES_SEO: PlanesDeServicio = {
    servicioSlug: 'seo',
    servicioTitulo: 'SEO',
    planes: [
        {
            id: 'seo-basico',
            nombre: 'Basico',
            precio: '$150',
            descripcion: 'Auditoria y optimizacion SEO inicial.',
            ctaTexto: 'Comenzar',
            ctaLink: '/contacto/',
            caracteristicas: [
                incluida('Auditoria SEO completa'),
                incluida('Optimizacion de 5 paginas'),
                incluida('Configuracion Search Console'),
                incluida('Reporte mensual basico'),
                noIncluida('Link building'),
                noIncluida('Contenido optimizado'),
                noIncluida('SEO local'),
            ]
        },
        {
            id: 'seo-avanzado',
            nombre: 'Avanzado',
            precio: '$400',
            descripcion: 'Estrategia SEO completa con contenido y links.',
            ctaTexto: 'Elegir plan',
            ctaLink: '/contacto/',
            destacado: true,
            caracteristicas: [
                incluida('Estrategia SEO integral'),
                incluida('Optimizacion paginas ilimitadas'),
                incluida('4 articulos SEO/mes'),
                incluida('Link building activo'),
                incluida('SEO local + Google My Business'),
                incluida('Reportes semanales'),
                incluida('Analisis de competencia'),
            ]
        }
    ]
};

/* ── Marketing Digital ──────────────────────────────────── */
export const PLANES_MARKETING: PlanesDeServicio = {
    servicioSlug: 'marketing-digital',
    servicioTitulo: 'Marketing Digital',
    planes: [
        {
            id: 'mkt-basico',
            nombre: 'Basico',
            precio: '$150',
            descripcion: 'Gestion basica de redes sociales y ads.',
            ctaTexto: 'Comenzar',
            ctaLink: '/contacto/',
            caracteristicas: [
                incluida('Gestion de 2 redes sociales'),
                incluida('8 publicaciones/mes'),
                incluida('1 campana de ads/mes'),
                incluida('Reporte mensual'),
                noIncluida('Email marketing'),
                noIncluida('Estrategia de contenidos'),
                noIncluida('Influencer marketing'),
            ]
        },
        {
            id: 'mkt-avanzado',
            nombre: 'Avanzado',
            precio: '$400',
            descripcion: 'Estrategia de marketing digital integral.',
            ctaTexto: 'Elegir plan',
            ctaLink: '/contacto/',
            destacado: true,
            caracteristicas: [
                incluida('Gestion de 4 redes sociales'),
                incluida('20 publicaciones/mes'),
                incluida('Campanas ads ilimitadas'),
                incluida('Email marketing (newsletters)'),
                incluida('Estrategia de contenidos'),
                incluida('Reportes semanales'),
                incluida('A/B testing campanas'),
            ]
        }
    ]
};

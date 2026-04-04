/*
 * Datos de showcase/proyectos centralizados.
 * [044A-3] Proyectos reales reemplazando placeholders.
 */
import {Proyecto, CategoriaShowcase} from '../types/contenido';

const PROYECTOS_FALLBACK: Proyecto[] = [
    {
        id: 'kamples',
        titulo: 'KAMPLES',
        cliente: 'Open Source Platform',
        categorias: ['web', 'software'],
        imagen: '/assets/Proyectos portadas/Kamples portada.jpg',
        descripcion: 'Plataforma de samples musicales con algoritmo de recomendación, DAW integrado y funcionalidades de red social. Código abierto. Organiza, descubre y comparte colecciones como en Pinterest — con la profundidad de WhoSampled.',
        link: '/proyectos/kamples',
        skills: [
            {id: 1, titulo: 'Full-Stack Development', descripcion: 'Arquitectura completa: API, frontend SPA y procesamiento de audio.'},
            {id: 2, titulo: 'Algoritmo de Recomendación', descripcion: 'Motor de descubrimiento basado en samples, géneros y uso.'},
            {id: 3, titulo: 'DAW Integrado', descripcion: 'Workstation de audio embebida para previsualizar y mezclar samples.'},
            {id: 4, titulo: 'Red Social', descripcion: 'Colecciones, tableros, "ver más ideas" y perfiles de usuario.'}
        ],
        galeria: ['/assets/Kamples/1.jpg', '/assets/Kamples/2.jpg', '/assets/Kamples/3.jpg']
    },
    {
        id: 'mabuhay',
        titulo: 'MABUHAY',
        cliente: 'Agencia de Viajes',
        categorias: ['web', 'branding'],
        imagen: '/assets/Proyectos portadas/Mabuhay.jpg',
        descripcion: 'Web y branding para agencia de viajes en España especializada en destinos asiáticos. Diseño cálido y visual que transmite la esencia de cada destino.',
        link: '/proyectos/mabuhay',
        skills: [
            {id: 1, titulo: 'Web Design', descripcion: 'Sitio web responsive con catálogo de destinos y reservas.'},
            {id: 2, titulo: 'Branding', descripcion: 'Identidad visual inspirada en la hospitalidad filipina.'},
            {id: 3, titulo: 'Content Strategy', descripcion: 'Fotografía y textos que invitan a explorar.'}
        ],
        galeria: ['/assets/Mabuhay/1.png', '/assets/Mabuhay/2.jpg', '/assets/Mabuhay/3.jpg']
    },
    {
        id: 'guillermochatbot',
        titulo: 'GUILLERMOCHATBOT.ES',
        cliente: 'AI Portfolio',
        categorias: ['ai', 'web'],
        imagen: '/assets/Proyectos portadas/Guillermochatbot.es.jpg',
        descripcion: 'Portfolio interactivo con chatbot IA que responde sobre experiencia profesional, proyectos y skills técnicos.',
        link: '/proyectos/guillermochatbot',
        skills: [
            {id: 1, titulo: 'AI Chatbot', descripcion: 'Asistente conversacional entrenado con contexto personal.'},
            {id: 2, titulo: 'Web Design', descripcion: 'Portfolio moderno con interacciones fluidas.'}
        ]
    },
    {
        id: 'task',
        titulo: 'TASK',
        cliente: 'Productivity App',
        categorias: ['app', 'software'],
        imagen: '/assets/Proyectos portadas/Task portada.jpg',
        descripcion: 'Aplicación de gestión de tareas con enfoque en simplicidad y flujos de trabajo ágiles.',
        link: '/proyectos/task',
        skills: [
            {id: 1, titulo: 'Product Design', descripcion: 'UX centrada en flujos de productividad real.'},
            {id: 2, titulo: 'App Development', descripcion: 'Desarrollo mobile-first con sincronización en tiempo real.'}
        ]
    },
    {
        id: 'material-de-padel',
        titulo: 'MATERIAL DE PÁDEL',
        cliente: 'E-commerce Deportivo',
        categorias: ['web', 'branding'],
        imagen: '/assets/Proyectos portadas/material de padel.jpg',
        descripcion: 'Tienda online de equipamiento de pádel con catálogo, comparador de palas y blog de contenido deportivo.',
        link: '/proyectos/material-de-padel',
        skills: [
            {id: 1, titulo: 'E-commerce', descripcion: 'Tienda con catálogo filtrable y checkout optimizado.'},
            {id: 2, titulo: 'SEO & Content', descripcion: 'Estrategia de contenido para posicionamiento orgánico.'}
        ]
    }
];

/* [044A-1] Sin GLORY_CONTEXT, datos directos */
export const PROYECTOS_DATA: Proyecto[] = PROYECTOS_FALLBACK;

/*
 * Agrupar proyectos en categorías para la sección showcase del home.
 * Se dividen en 2 grupos. Los proyectos NO se repiten entre categorías:
 * una vez asignado a un grupo, se excluye del siguiente.
 */
const buildCategoriasShowcase = (): CategoriaShowcase[] => {
    const usados = new Set<string | number>();

    const filtrarSinRepetir = (filtro: (cats: string[]) => boolean, max: number): Proyecto[] => {
        const resultado: Proyecto[] = [];
        for (const p of PROYECTOS_DATA) {
            if (usados.has(p.id)) continue;
            const cats = Array.isArray(p.categorias) ? p.categorias : [p.categorias];
            if (filtro(cats)) {
                resultado.push(p);
                usados.add(p.id);
                if (resultado.length >= max) break;
            }
        }
        return resultado;
    };

    return [
        {
            titulo: 'Website & Digital Experiences',
            proyectos: filtrarSinRepetir(cats => cats.includes('web') || cats.includes('app'), 3)
        },
        {
            titulo: 'Brand Identity & Strategy',
            proyectos: filtrarSinRepetir(cats => cats.includes('branding'), 3)
        }
    ];
};

export const CATEGORIAS_SHOWCASE: CategoriaShowcase[] = buildCategoriasShowcase();

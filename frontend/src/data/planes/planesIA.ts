/**
 * Planes: Agentes de IA
 * [044A-31] Eliminados planes de Chatbots (servicio eliminado).
 */
import {type PlanesDeServicio, incluida, noIncluida} from './tipos';

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
        },
        {
            id: 'ia-personalizado',
            nombre: 'Personalizado',
            precio: 'A medida',
            descripcion: 'Solucion de IA a la medida de tu negocio.',
            ctaTexto: 'Hablar con nosotros',
            ctaLink: '/contacto/',
            esPersonalizado: true,
            caracteristicas: [
                incluida('Agentes ilimitados'),
                incluida('Modelo fine-tuned'),
                incluida('Integracion total'),
                incluida('Datos propietarios'),
                incluida('SLA garantizado'),
                incluida('Soporte dedicado 24/7'),
                incluida('Consultoria IA estrategica'),
            ]
        }
    ]
};

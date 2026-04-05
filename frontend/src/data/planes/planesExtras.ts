/**
 * Planes: E-commerce
 * [044A-31] Eliminados planes de UX/UI, Automatización y Consultoría (servicios eliminados).
 */
import {type PlanesDeServicio, incluida, noIncluida} from './tipos';

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
        },
        {
            id: 'ecommerce-personalizado',
            nombre: 'Personalizado',
            precio: 'A medida',
            descripcion: 'Solucion e-commerce a medida para operaciones complejas.',
            ctaTexto: 'Hablar con nosotros',
            ctaLink: '/contacto/',
            esPersonalizado: true,
            caracteristicas: [
                incluida('Arquitectura personalizada'),
                incluida('Integraciones ERP/CRM'),
                incluida('Multi-idioma y multi-moneda'),
                incluida('Marketplace multi-vendor'),
                incluida('Analytics avanzado'),
                incluida('Soporte prioritario 24/7'),
                incluida('Migracion de datos'),
            ]
        }
    ]
};

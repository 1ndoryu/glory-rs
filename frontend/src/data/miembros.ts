/* [044A-1] Datos de miembros del equipo centralizados.
 * Imágenes servidas desde /assets/equipo/ (public folder). */
import {Miembro} from '../types/contenido';

const wanImg = '/assets/equipo/wan.jpg';
const anthonyImg = '/assets/equipo/anthony.jpg';
const misaelImg = '/assets/equipo/misael.jpg';

const MIEMBROS_FALLBACK: Miembro[] = [
    {
        id: 'wan',
        nombre: 'Wan',
        bio: 'Fundadora y directora creativa con visión estratégica para soluciones digitales de alto impacto.',
        cargo: 'CEO & Founder',
        avatar: wanImg,
        linkedin: '#',
        github: '#'
    },
    {
        id: 'anthony',
        nombre: 'Anthony',
        bio: 'Ingeniero de software principal, especializado en arquitecturas escalables y rendimiento.',
        cargo: 'Lead Developer',
        avatar: anthonyImg,
        linkedin: '#',
        github: '#'
    },
    {
        id: 'misael',
        nombre: 'Misael',
        bio: 'Ingeniero DevOps enfocado en la automatización, despliegue continuo y estabilidad de infraestructura.',
        cargo: 'DevOps Engineer',
        avatar: misaelImg,
        linkedin: '#',
        github: '#'
    }
];

/* [044A-1] Sin GLORY_CONTEXT, datos directos */
export const MIEMBROS_DATA: Miembro[] = MIEMBROS_FALLBACK;

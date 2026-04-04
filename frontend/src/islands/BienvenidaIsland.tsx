/**
 * Componente: BienvenidaIsland
 * Página principal (Home) del sitio.
 */

import '../styles/variables.css';
import '../styles/bienvenida.css';
import {LayoutPagina} from '../components/layout/LayoutPagina';
import {SeccionHero} from '../components/home/SeccionHero';
import {SeccionServicios} from '../components/home/SeccionServicios';
import {SeccionBlog} from '../components/home/SeccionBlog';
import {SeccionContacto} from '../components/home/SeccionContacto';
import {SeccionShowcase} from '../components/home/SeccionShowcase';

/* [044A-6] Se eliminaron SeccionClientes y SeccionTestimonios del home.
 * Los componentes siguen existiendo en components/home/ por si se reactivan. */
export const BienvenidaIsland = (): JSX.Element => {
    return (
        <LayoutPagina className="mainContainer">
            <SeccionHero />
            <SeccionShowcase />
            <SeccionServicios />
            <SeccionContacto compacto />
            <SeccionBlog />
        </LayoutPagina>
    );
};

export default BienvenidaIsland;

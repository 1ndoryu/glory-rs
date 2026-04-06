/**
 * Componente: NosotrosIsland
 * Página "Sobre Nosotros" con misión, equipo, marcas y testimonios.
 */
import React from 'react';
import {useTranslation} from 'react-i18next';
import '../styles/variables.css';
import './NosotrosIsland.css';
import {LayoutPagina} from '../components/layout/LayoutPagina';
import {SEOHead} from '../components/seo/SEOHead';
import {SeccionHeader} from '../components/ui/SeccionHeader';
import {SeccionContacto} from '../components/home/SeccionContacto';
import {MIEMBROS_DATA} from '../data/miembros';
import {Miembro} from '../types/contenido';

interface NosotrosIslandProps {
    titulo?: string;
}

/* Tarjeta de miembro del equipo */
const TarjetaMiembro: React.FC<{miembro: Miembro}> = ({miembro}) => (
    <article className="tarjetaMiembro">
        <div className="miembroAvatar">
            <img src={miembro.avatar} alt={miembro.nombre} loading="lazy" />
        </div>
        <div className="miembroInfo">
            <h3 className="miembroNombre">{miembro.nombre}</h3>
            <span className="miembroCargo">{miembro.cargo}</span>
            <p className="miembroBio">{miembro.bio}</p>
            <div className="miembroRedes">
                {miembro.linkedin && (
                    <a href={miembro.linkedin} target="_blank" rel="noopener noreferrer" aria-label="LinkedIn">
                        LinkedIn
                    </a>
                )}
                {miembro.github && (
                    <a href={miembro.github} target="_blank" rel="noopener noreferrer" aria-label="GitHub">
                        GitHub
                    </a>
                )}
            </div>
        </div>
    </article>
);

export const NosotrosIsland = ({titulo}: NosotrosIslandProps): JSX.Element => {
    const {t} = useTranslation();

    return (
        <LayoutPagina className="nosotrosMain" id="paginaNosotros">
            <SEOHead
                title="Nosotros"
                description="Conoce al equipo de Nakomi Studio, nuestra misión y valores."
                path="/nosotros"
            />
            {/* Hero */}
            <section className="nosotrosHero">
                <div className="nosotrosHeroContenido">
                    <div className="nosotrosHeroTexto">
                        <h1 className="nosotrosHeroTitulo">{titulo || t('about.title')}</h1>
                    </div>
                    <div className="nosotrosHeroDescripcion">
                        <p>{t('about.description')}</p>
                    </div>
                </div>
            </section>

            {/* Misión / Valores */}
            <section className="nosotrosMision">
                <div className="nosotrosMisionContenedor">
                    <div className="misionGrid">
                        <div className="misionItem">
                            <h3 className="misionItemTitulo">{t('about.mission_title')}</h3>
                            <p className="misionItemTexto">{t('about.mission_text')}</p>
                        </div>
                        <div className="misionItem">
                            <h3 className="misionItemTitulo">{t('about.approach_title')}</h3>
                            <p className="misionItemTexto">{t('about.approach_text')}</p>
                        </div>
                        <div className="misionItem">
                            <h3 className="misionItemTitulo">{t('about.values_title')}</h3>
                            <p className="misionItemTexto">{t('about.values_text')}</p>
                        </div>
                    </div>
                </div>
            </section>

            {/* Equipo */}
            <section className="nosotrosEquipo">
                <div className="nosotrosEquipoContenedor">
                    <SeccionHeader titulo={t('about.team_title')} />
                    <div className="equipoGrid">
                        {MIEMBROS_DATA.map(miembro => (
                            <TarjetaMiembro key={miembro.id} miembro={miembro} />
                        ))}
                    </div>
                </div>
            </section>

            {/* Contacto */}
            <SeccionContacto />
        </LayoutPagina>
    );
};

export default NosotrosIsland;

/**
 * Componente: SeccionShowcase
 * Muestra proyectos destacados organizados por categoría.
 * Datos centralizados en data/showcase.ts (DRY).
 */
import {useTranslation} from 'react-i18next';
import {SeccionHeader} from '../ui/SeccionHeader';
import {Badge} from '../ui/Badge';
import {CATEGORIAS_SHOWCASE} from '../../data/showcase';
import './SeccionShowcase.css';

export const SeccionShowcase = (): JSX.Element => {
    const {t} = useTranslation();

    return (
        <section className="seccionShowcase">
            <div className="showcaseContenedor">
                <SeccionHeader titulo={t('sections.selected_work')} />

                {CATEGORIAS_SHOWCASE.map((categoria, index) => (
                    <div className="showcaseFila" key={index}>
                        <div className="showcaseCategoria">
                            <h2 className="showcaseTituloCategoria">{categoria.titulo}</h2>
                        </div>

                        <div className="showcaseGridProyectos">
                            {categoria.proyectos.map(proyecto => (
                                <a key={proyecto.id} href={proyecto.link || '#'} className="proyectoCard">
                                    <div className="proyectoImagenWrapper">
                                        <img src={proyecto.imagen} alt={proyecto.titulo} className="proyectoImagen" loading="lazy" />
                                    </div>
                                    <div className="proyectoInfo">
                                        <h3 className="proyectoTitulo">
                                            {proyecto.titulo} <span className="proyectoCliente">- {proyecto.cliente}</span>
                                        </h3>
                                        <div className="proyectoTags">
                                            {(Array.isArray(proyecto.categorias) ? proyecto.categorias : [proyecto.categorias]).map(cat => (
                                                <Badge key={cat} label={cat} />
                                            ))}
                                        </div>
                                    </div>
                                </a>
                            ))}
                        </div>
                    </div>
                ))}
            </div>
        </section>
    );
};

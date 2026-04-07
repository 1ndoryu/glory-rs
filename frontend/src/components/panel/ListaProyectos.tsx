/* [074A-12] Lista de proyectos en el CMS admin.
 * Grid de cards con status, título, cliente, categorías. Click para editar.
 * Patrón: misma estructura que ListaBlog.tsx.
 * sentinel-disable-file button-nativo: Botón archivar es icon-only overlay sobre card,
 * botonBase interfiere con posicionamiento absoluto y estilos (mismo patrón ListaBlog). */
import React from 'react';
import { Plus, Archive } from 'lucide-react';
import { Button } from '../ui/Button';
import type { AdminProject } from '../../api/admin-projects';
import './ListaProyectos.css';

interface ListaProyectosProps {
    proyectos: AdminProject[];
    cargando: boolean;
    onEditar: (proyecto: AdminProject) => void;
    onCrear: () => void;
    onArchivar: (id: string) => void;
}

function BadgeStatus({ status }: { status: string }) {
    const clase = `listaProyectosBadge listaProyectosBadge--${status}`;
    const labels: Record<string, string> = {
        published: 'Publicado',
        draft: 'Borrador',
        archived: 'Archivado',
    };
    return <span className={clase}>{labels[status] ?? status}</span>;
}

export const ListaProyectos: React.FC<ListaProyectosProps> = ({
    proyectos,
    cargando,
    onEditar,
    onCrear,
    onArchivar,
}) => {
    if (cargando) {
        return <div className="listaProyectosCargando">Cargando proyectos...</div>;
    }

    return (
        <div className="listaProyectos">
            <div className="listaProyectosHeader">
                <h3 className="listaProyectosTitulo">Proyectos</h3>
                <Button variante="primario" tamano="pequeno" onClick={onCrear}>
                    <Plus size={14} />
                    Nuevo
                </Button>
            </div>

            <div className="listaProyectosGrid">
                {proyectos.map(proyecto => (
                    <div
                        key={proyecto.id}
                        className={`listaProyectosCard ${proyecto.status === 'archived' ? 'listaProyectosCard--inactivo' : ''}`}
                        onClick={() => onEditar(proyecto)}
                        role="button"
                        tabIndex={0}
                        onKeyDown={e => { if (e.key === 'Enter') onEditar(proyecto); }}
                    >
                        {proyecto.featured_image && (
                            <div className="listaProyectosImagen">
                                <img src={proyecto.featured_image} alt={proyecto.title} />
                            </div>
                        )}
                        <div className="listaProyectosInfo">
                            <div className="listaProyectosCardHeader">
                                <span className="listaProyectosNombre">{proyecto.title}</span>
                                <BadgeStatus status={proyecto.status} />
                            </div>
                            {proyecto.client && (
                                <span className="listaProyectosCliente">{proyecto.client}</span>
                            )}
                            <span className="listaProyectosSlug">/{proyecto.slug}</span>
                            <div className="listaProyectosCardFooter">
                                {proyecto.categories.length > 0 && (
                                    <span className="listaProyectosCategorias">
                                        {proyecto.categories.slice(0, 3).join(', ')}
                                    </span>
                                )}
                                {proyecto.technologies.length > 0 && (
                                    <span className="listaProyectosTech">
                                        {proyecto.technologies.slice(0, 3).join(', ')}
                                    </span>
                                )}
                            </div>
                        </div>
                        {proyecto.status !== 'archived' && (
                            <button
                                type="button"
                                className="listaProyectosArchivar"
                                title="Archivar proyecto"
                                onClick={e => { e.stopPropagation(); onArchivar(proyecto.id); }}
                            >
                                <Archive size={14} />
                            </button>
                        )}
                    </div>
                ))}
            </div>
        </div>
    );
};

/* [074A-11] Lista de blog posts en el CMS admin.
 * Grid de cards con status, título, fecha, tags. Click para editar.
 * Patrón: misma estructura que ListaServicios.tsx.
 * sentinel-disable-file button-nativo: Botón archivar es icon-only overlay sobre card,
 * botonBase interfiere con posicionamiento absoluto y estilos (mismo patrón ListaServicios). */
import React from 'react';
import {Plus, Archive} from 'lucide-react';
import {Button} from '../ui/Button';
import type {AdminBlogPost} from '../../api/admin-blog';
import './ListaBlog.css';

interface ListaBlogProps {
    posts: AdminBlogPost[];
    cargando: boolean;
    onEditar: (post: AdminBlogPost) => void;
    onCrear: () => void;
    onArchivar: (id: string) => void;
}

/* Badge de status con color semántico */
function BadgeStatus({status}: {status: string}) {
    const clase = `listaBlogBadge listaBlogBadge--${status}`;
    const labels: Record<string, string> = {
        published: 'Publicado',
        draft: 'Borrador',
        archived: 'Archivado',
    };
    return <span className={clase}>{labels[status] ?? status}</span>;
}

/* Formatea fecha ISO a formato legible */
function formatearFecha(iso: string): string {
    try {
        return new Date(iso).toLocaleDateString('es', {day: 'numeric', month: 'short', year: 'numeric'});
    } catch {
        return iso;
    }
}

export const ListaBlog: React.FC<ListaBlogProps> = ({
    posts,
    cargando,
    onEditar,
    onCrear,
    onArchivar,
}) => {
    if (cargando) {
        return <div className="listaBlogCargando">Cargando posts...</div>;
    }

    return (
        <div className="listaBlog">
            <div className="listaBlogHeader">
                <h3 className="listaBlogTitulo">Blog</h3>
                <Button variante="primario" tamano="pequeno" onClick={onCrear}>
                    <Plus size={14} />
                    Nuevo
                </Button>
            </div>

            <div className="listaBlogGrid">
                {posts.map(post => (
                    <div
                        key={post.id}
                        className={`listaBlogCard ${post.status === 'archived' ? 'listaBlogCard--inactivo' : ''}`}
                        onClick={() => onEditar(post)}
                        role="button"
                        tabIndex={0}
                        onKeyDown={e => { if (e.key === 'Enter') onEditar(post); }}
                    >
                        {post.featured_image && (
                            <div className="listaBlogImagen">
                                <img src={post.featured_image} alt={post.title} />
                            </div>
                        )}
                        <div className="listaBlogInfo">
                            <div className="listaBlogCardHeader">
                                <span className="listaBlogNombre">{post.title}</span>
                                <BadgeStatus status={post.status} />
                            </div>
                            <span className="listaBlogSlug">/{post.slug}</span>
                            {post.excerpt && (
                                <span className="listaBlogDesc">{post.excerpt}</span>
                            )}
                            <div className="listaBlogCardFooter">
                                <span className="listaBlogFecha">
                                    {formatearFecha(post.created_at)}
                                </span>
                                {post.tags.length > 0 && (
                                    <span className="listaBlogTags">
                                        {post.tags.slice(0, 3).join(', ')}
                                    </span>
                                )}
                            </div>
                        </div>
                        {post.status !== 'archived' && (
                            <button
                                type="button"
                                className="listaBlogArchivar"
                                title="Archivar post"
                                onClick={e => { e.stopPropagation(); onArchivar(post.id); }}
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

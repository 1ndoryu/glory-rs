/**
 * Componente: BlogIsland
 * Página de listado de artículos del blog.
 * TO-DO: Conectar con WP REST API para paginación real.
 */
import React, {useState, useMemo} from 'react';
import {useTranslation} from 'react-i18next';
import '../styles/variables.css';
import './BlogIsland.css';
import {LayoutPagina} from '../components/layout/LayoutPagina';
import {SEOHead} from '../components/seo/SEOHead';
import {SeccionContacto} from '../components/home/SeccionContacto';
import {POSTS_BLOG} from '../data/blog';
import {PostBlog} from '../types/contenido';
import {obtenerImagenBlog} from '../hooks/useImagenes';
import {navegar} from '../navegacionSPA';
import {Button} from '../components/ui/Button';

interface BlogIslandProps {
    titulo?: string;
}

/*
 * Tarjeta de artículo con layout horizontal.
 * Siempre muestra imagen: backend -> fallback colors (obtenerImagenBlog).
 */
/* [044A-35] Tarjeta minimalista: imagen de fondo con título y categoría superpuestos. */
const TarjetaArticulo: React.FC<{post: PostBlog; destacado?: boolean}> = ({post, destacado = false}) => {
    const imagenFinal = post.imagen || obtenerImagenBlog(post.id);

    return (
        <a href={post.link || '#'} onClick={(e) => { e.preventDefault(); if (post.link) navegar(post.link); }} className={`tarjetaArticulo ${destacado ? 'tarjetaArticuloDestacado' : ''}`}>
            <img src={imagenFinal} alt={post.titulo} className="articuloImagen" loading="lazy" />
            <div className="articuloOverlay">
                <span className="articuloCategoria">{post.categoria}</span>
                <h3 className="articuloTitulo">{post.titulo}</h3>
            </div>
        </a>
    );
};

export const BlogIsland = ({titulo}: BlogIslandProps): JSX.Element => {
    const {t} = useTranslation();
    const [categoriaActiva, setCategoriaActiva] = useState('todos');

    /* Categorías únicas extraídas de los posts */
    const categorias = useMemo(() => {
        const cats = new Set(POSTS_BLOG.map(p => p.categoria));
        return ['todos', ...Array.from(cats)];
    }, []);

    const postsFiltrados = useMemo(() => {
        if (categoriaActiva === 'todos') return POSTS_BLOG;
        return POSTS_BLOG.filter(p => p.categoria === categoriaActiva);
    }, [categoriaActiva]);

    return (
        <LayoutPagina className="blogMain" id="paginaBlog">
            <SEOHead
                title="Blog"
                description="Artículos sobre desarrollo web, diseño y tecnología del equipo de Nakomi Studio."
                path="/blog"
            />
            {/* Hero */}
            <section className="blogHero">
                <div className="blogHeroContenido">
                    <div>
                        <h1 className="blogHeroTitulo">{titulo || t('blog_page.title')}</h1>
                    </div>
                    <div className="blogHeroDescripcion">
                        <p>{t('blog_page.description')}</p>
                    </div>
                </div>
            </section>

            <section className="blogContenido">
                <div className="blogContenedor">
                    {/* Filtros de categoría */}
                    <div className="blogFiltros">
                        {categorias.map(cat => (
                            <Button
                                variante="texto"
                                key={cat}
                                className={`blogFiltroBtn ${categoriaActiva === cat ? 'blogFiltroBtnActivo' : ''}`}
                                onClick={() => setCategoriaActiva(cat)}
                                type="button"
                            >
                                {cat === 'todos' ? t('blog_page.all') : cat}
                            </Button>
                        ))}
                    </div>

                    {/* Todos los posts dentro de un solo contenedor */}
                    {postsFiltrados.length > 0 && (
                        <div className="blogListaArticulos">
                            {postsFiltrados.map((post, i) => (
                                <TarjetaArticulo key={post.id} post={post} destacado={i === 0} />
                            ))}
                        </div>
                    )}

                    {postsFiltrados.length === 0 && (
                        <p className="blogSinResultados">{t('blog_page.empty')}</p>
                    )}
                </div>
            </section>

            <SeccionContacto />
        </LayoutPagina>
    );
};

export default BlogIsland;

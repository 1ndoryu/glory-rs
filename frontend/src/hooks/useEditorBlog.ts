/* [074A-11] Hook interno para el estado del formulario de EditorBlog.
 * Extrae los useState del componente para cumplir regla max 3 useState.
 * Patrón: misma estructura que useEditorServicio.ts. */
import {useState, useCallback, useEffect} from 'react';
import type {AdminBlogPost, CreateBlogPostBody, UpdateBlogPostBody} from '../api/admin-blog';

export interface EditorBlogState {
    titulo: string;
    slug: string;
    extracto: string;
    contenido: string;
    imagenUrl: string;
    status: string;
    tags: string[];
    metaTitle: string;
    metaDescription: string;
    setTitulo: (v: string) => void;
    setSlug: (v: string) => void;
    setExtracto: (v: string) => void;
    setContenido: (v: string) => void;
    setImagenUrl: (v: string) => void;
    setStatus: (v: string) => void;
    setTags: (v: string[]) => void;
    setMetaTitle: (v: string) => void;
    setMetaDescription: (v: string) => void;
    buildBody: () => CreateBlogPostBody | UpdateBlogPostBody;
    resetear: () => void;
}

export function useEditorBlog(post: AdminBlogPost | null, abierto: boolean): EditorBlogState {
    const [titulo, setTitulo] = useState('');
    const [slug, setSlug] = useState('');
    const [extracto, setExtracto] = useState('');
    const [contenido, setContenido] = useState('');
    const [imagenUrl, setImagenUrl] = useState('');
    const [status, setStatus] = useState('draft');
    const [tags, setTags] = useState<string[]>([]);
    const [metaTitle, setMetaTitle] = useState('');
    const [metaDescription, setMetaDescription] = useState('');

    const resetear = useCallback(() => {
        setTitulo('');
        setSlug('');
        setExtracto('');
        setContenido('');
        setImagenUrl('');
        setStatus('draft');
        setTags([]);
        setMetaTitle('');
        setMetaDescription('');
    }, []);

    /* Sincronizar con post seleccionado */
    useEffect(() => {
        if (post) {
            setTitulo(post.title);
            setSlug(post.slug);
            setExtracto(post.excerpt ?? '');
            setContenido(post.content);
            setImagenUrl(post.featured_image ?? '');
            setStatus(post.status);
            setTags(post.tags);
            setMetaTitle(post.meta_title ?? '');
            setMetaDescription(post.meta_description ?? '');
        } else {
            resetear();
        }
    }, [post, abierto, resetear]);

    const buildBody = useCallback((): CreateBlogPostBody | UpdateBlogPostBody => ({
        title: titulo,
        slug,
        excerpt: extracto || undefined,
        content: contenido,
        featured_image: imagenUrl || undefined,
        status,
        tags: tags.length > 0 ? tags : undefined,
        meta_title: metaTitle || undefined,
        meta_description: metaDescription || undefined,
    }), [titulo, slug, extracto, contenido, imagenUrl, status, tags, metaTitle, metaDescription]);

    return {
        titulo, slug, extracto, contenido, imagenUrl, status, tags,
        metaTitle, metaDescription,
        setTitulo, setSlug, setExtracto, setContenido, setImagenUrl,
        setStatus, setTags, setMetaTitle, setMetaDescription,
        buildBody, resetear,
    };
}

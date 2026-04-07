/* [074A-9] Hook interno para el estado del formulario de EditorServicio.
 * Extrae los 10+ useState del componente para cumplir regla max 3 useState. */
import {useState, useCallback, useEffect} from 'react';
import type {AdminService, CreateServiceBody, UpdateServiceBody} from '../api/admin-services';

export interface EditorServicioState {
    titulo: string;
    slug: string;
    descripcion: string;
    contenido: string;
    precioCents: number;
    imagenUrl: string;
    metaTitle: string;
    metaDescription: string;
    status: string;
    sortOrder: number;
    setTitulo: (v: string) => void;
    setSlug: (v: string) => void;
    setDescripcion: (v: string) => void;
    setContenido: (v: string) => void;
    setPrecioCents: (v: number) => void;
    setImagenUrl: (v: string) => void;
    setMetaTitle: (v: string) => void;
    setMetaDescription: (v: string) => void;
    setStatus: (v: string) => void;
    setSortOrder: (v: number) => void;
    buildBody: () => CreateServiceBody | UpdateServiceBody;
    resetear: () => void;
}

export function useEditorServicio(servicio: AdminService | null, abierto: boolean): EditorServicioState {
    const [titulo, setTitulo] = useState('');
    const [slug, setSlug] = useState('');
    const [descripcion, setDescripcion] = useState('');
    const [contenido, setContenido] = useState('');
    const [precioCents, setPrecioCents] = useState(0);
    const [imagenUrl, setImagenUrl] = useState('');
    const [metaTitle, setMetaTitle] = useState('');
    const [metaDescription, setMetaDescription] = useState('');
    const [status, setStatus] = useState('draft');
    const [sortOrder, setSortOrder] = useState(0);

    const resetear = useCallback(() => {
        setTitulo('');
        setSlug('');
        setDescripcion('');
        setContenido('');
        setPrecioCents(0);
        setImagenUrl('');
        setMetaTitle('');
        setMetaDescription('');
        setStatus('draft');
        setSortOrder(0);
    }, []);

    /* Sincronizar con servicio seleccionado */
    useEffect(() => {
        if (servicio) {
            setTitulo(servicio.title);
            setSlug(servicio.slug);
            setDescripcion(servicio.description ?? '');
            setContenido(servicio.content ?? '');
            setPrecioCents(servicio.base_price_cents);
            setImagenUrl(servicio.image_url ?? '');
            setMetaTitle(servicio.meta_title ?? '');
            setMetaDescription(servicio.meta_description ?? '');
            setStatus(servicio.status);
            setSortOrder(servicio.sort_order);
        } else {
            resetear();
        }
    }, [servicio, abierto, resetear]);

    const buildBody = useCallback(() => ({
        title: titulo,
        slug,
        description: descripcion || undefined,
        base_price_cents: precioCents,
        image_url: imagenUrl || undefined,
        content: contenido || undefined,
        meta_title: metaTitle || undefined,
        meta_description: metaDescription || undefined,
        status,
        sort_order: sortOrder,
    }), [titulo, slug, descripcion, precioCents, imagenUrl, contenido, metaTitle, metaDescription, status, sortOrder]);

    return {
        titulo, slug, descripcion, contenido, precioCents, imagenUrl,
        metaTitle, metaDescription, status, sortOrder,
        setTitulo, setSlug, setDescripcion, setContenido, setPrecioCents,
        setImagenUrl, setMetaTitle, setMetaDescription, setStatus, setSortOrder,
        buildBody, resetear,
    };
}

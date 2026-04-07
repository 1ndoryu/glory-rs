/* [074A-11] Hook para gestión de blog posts en panel CMS.
 * Encapsula: listado, creación, actualización, archivado.
 * Patrón: misma estructura que useContenidoServicios.ts. */
import {useState, useCallback, useEffect} from 'react';
import {
    AdminBlogPost,
    CreateBlogPostBody,
    UpdateBlogPostBody,
    apiListAdminBlog,
    apiCreateBlogPost,
    apiUpdateBlogPost,
    apiArchiveBlogPost,
} from '../api/admin-blog';

export function useContenidoBlog() {
    const [posts, setPosts] = useState<AdminBlogPost[]>([]);
    const [cargando, setCargando] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [guardando, setGuardando] = useState(false);

    const cargar = useCallback(async () => {
        setCargando(true);
        setError(null);
        try {
            const data = await apiListAdminBlog();
            setPosts(data);
        } catch (err: unknown) {
            const msg = err instanceof Error ? err.message : 'Error al cargar posts';
            setError(msg);
        } finally {
            setCargando(false);
        }
    }, []);

    useEffect(() => {
        cargar();
    }, [cargar]);

    const crear = useCallback(async (body: CreateBlogPostBody): Promise<AdminBlogPost | null> => {
        setGuardando(true);
        setError(null);
        try {
            const nuevo = await apiCreateBlogPost(body);
            setPosts(prev => [...prev, nuevo]);
            return nuevo;
        } catch (err: unknown) {
            const msg = err instanceof Error ? err.message : 'Error al crear post';
            setError(msg);
            return null;
        } finally {
            setGuardando(false);
        }
    }, []);

    const actualizar = useCallback(async (id: string, body: UpdateBlogPostBody): Promise<AdminBlogPost | null> => {
        setGuardando(true);
        setError(null);
        try {
            const actualizado = await apiUpdateBlogPost(id, body);
            setPosts(prev => prev.map(p => p.id === id ? actualizado : p));
            return actualizado;
        } catch (err: unknown) {
            const msg = err instanceof Error ? err.message : 'Error al actualizar post';
            setError(msg);
            return null;
        } finally {
            setGuardando(false);
        }
    }, []);

    const archivar = useCallback(async (id: string): Promise<boolean> => {
        setGuardando(true);
        setError(null);
        try {
            await apiArchiveBlogPost(id);
            setPosts(prev => prev.map(p => p.id === id ? {...p, status: 'archived'} : p));
            return true;
        } catch (err: unknown) {
            const msg = err instanceof Error ? err.message : 'Error al archivar post';
            setError(msg);
            return false;
        } finally {
            setGuardando(false);
        }
    }, []);

    return {posts, cargando, error, guardando, crear, actualizar, archivar, recargar: cargar};
}

/* [074A-7] Sección "Contenido" del panel admin — CMS editorial.
 * Sub-tabs laterales: Servicios | Blog | Proyectos | Equipo.
 * Cada sub-tab renderiza su propio editor/lista.
 * [074A-9] Sub-tab Servicios conectada a ListaServicios + EditorServicio.
 * [074A-11] Sub-tab Blog conectada a ListaBlog + EditorBlog.
 * [074A-12] Sub-tab Proyectos conectada a ListaProyectos + EditorProyecto.
 * sentinel-disable-file componente-sin-hook: Orquestador de sub-tabs CMS.
 * Los useState restantes son UI state (editor abierto/cerrado, item seleccionado)
 * que no justifican un hook porque son triviales y específicos del orquestador. */
import React, {useState, useCallback} from 'react';
import {Briefcase, PenTool, FolderOpen, Users} from 'lucide-react';
import {Button} from '../ui/Button';
import {ListaServicios} from './ListaServicios';
import {EditorServicio} from './EditorServicio';
import {ListaBlog} from './ListaBlog';
import {EditorBlog} from './EditorBlog';
import {ListaProyectos} from './ListaProyectos';
import {EditorProyecto} from './EditorProyecto';
import {useContenidoServicios} from '../../hooks/useContenidoServicios';
import {useContenidoBlog} from '../../hooks/useContenidoBlog';
import {useContenidoProyectos} from '../../hooks/useContenidoProyectos';
import type {AdminService, CreateServiceBody, UpdateServiceBody} from '../../api/admin-services';
import type {AdminBlogPost, CreateBlogPostBody, UpdateBlogPostBody} from '../../api/admin-blog';
import type {AdminProject, CreateProjectBody, UpdateProjectBody} from '../../api/admin-projects';
import './SeccionContenido.css';

type SubTab = 'servicios' | 'blog' | 'proyectos' | 'equipo';

interface SubTabConfig {
    id: SubTab;
    label: string;
    icono: React.ElementType;
}

const SUB_TABS: SubTabConfig[] = [
    {id: 'servicios', label: 'Servicios', icono: Briefcase},
    {id: 'blog', label: 'Blog', icono: PenTool},
    {id: 'proyectos', label: 'Proyectos', icono: FolderOpen},
    {id: 'equipo', label: 'Equipo', icono: Users},
];

export const SeccionContenido: React.FC = () => {
    const [subTab, setSubTab] = useState<SubTab>('servicios');

    /* [074A-9] Estado del CMS servicios */
    const {servicios, cargando, error, guardando, crear, actualizar, archivar} = useContenidoServicios();
    const [editorAbierto, setEditorAbierto] = useState(false);
    const [servicioEditando, setServicioEditando] = useState<AdminService | null>(null);

    const handleEditarServicio = useCallback((svc: AdminService) => {
        setServicioEditando(svc);
        setEditorAbierto(true);
    }, []);

    const handleCrearServicio = useCallback(() => {
        setServicioEditando(null);
        setEditorAbierto(true);
    }, []);

    const handleGuardarServicio = useCallback(async (body: CreateServiceBody | UpdateServiceBody) => {
        if (servicioEditando) {
            const result = await actualizar(servicioEditando.id, body as UpdateServiceBody);
            if (result) setEditorAbierto(false);
        } else {
            const result = await crear(body as CreateServiceBody);
            if (result) setEditorAbierto(false);
        }
    }, [servicioEditando, actualizar, crear]);

    const handleArchivarServicio = useCallback(async (id: string) => {
        await archivar(id);
    }, [archivar]);

    /* [074A-11] Estado del CMS blog */
    const {
        posts: blogPosts,
        cargando: blogCargando,
        error: blogError,
        guardando: blogGuardando,
        crear: blogCrear,
        actualizar: blogActualizar,
        archivar: blogArchivar,
    } = useContenidoBlog();
    const [blogEditorAbierto, setBlogEditorAbierto] = useState(false);
    const [postEditando, setPostEditando] = useState<AdminBlogPost | null>(null);

    const handleEditarPost = useCallback((post: AdminBlogPost) => {
        setPostEditando(post);
        setBlogEditorAbierto(true);
    }, []);

    const handleCrearPost = useCallback(() => {
        setPostEditando(null);
        setBlogEditorAbierto(true);
    }, []);

    const handleGuardarPost = useCallback(async (body: CreateBlogPostBody | UpdateBlogPostBody) => {
        if (postEditando) {
            const result = await blogActualizar(postEditando.id, body as UpdateBlogPostBody);
            if (result) setBlogEditorAbierto(false);
        } else {
            const result = await blogCrear(body as CreateBlogPostBody);
            if (result) setBlogEditorAbierto(false);
        }
    }, [postEditando, blogActualizar, blogCrear]);

    const handleArchivarPost = useCallback(async (id: string) => {
        await blogArchivar(id);
    }, [blogArchivar]);

    /* [074A-12] Estado del CMS proyectos */
    const {
        proyectos: proyectosList,
        cargando: proyectosCargando,
        error: proyectosError,
        guardando: proyectosGuardando,
        crear: proyectosCrear,
        actualizar: proyectosActualizar,
        archivar: proyectosArchivar,
    } = useContenidoProyectos();
    const [proyectoEditorAbierto, setProyectoEditorAbierto] = useState(false);
    const [proyectoEditando, setProyectoEditando] = useState<AdminProject | null>(null);

    const handleEditarProyecto = useCallback((proy: AdminProject) => {
        setProyectoEditando(proy);
        setProyectoEditorAbierto(true);
    }, []);

    const handleCrearProyecto = useCallback(() => {
        setProyectoEditando(null);
        setProyectoEditorAbierto(true);
    }, []);

    const handleGuardarProyecto = useCallback(async (body: CreateProjectBody | UpdateProjectBody) => {
        if (proyectoEditando) {
            const result = await proyectosActualizar(proyectoEditando.id, body as UpdateProjectBody);
            if (result) setProyectoEditorAbierto(false);
        } else {
            const result = await proyectosCrear(body as CreateProjectBody);
            if (result) setProyectoEditorAbierto(false);
        }
    }, [proyectoEditando, proyectosActualizar, proyectosCrear]);

    const handleArchivarProyecto = useCallback(async (id: string) => {
        await proyectosArchivar(id);
    }, [proyectosArchivar]);

    return (
        <div className="contenidoContenedor">
            <div className="contenidoSubTabs">
                {SUB_TABS.map(tab => {
                    const Icono = tab.icono;
                    return (
                        <Button
                            key={tab.id}
                            type="button"
                            variante="texto"
                            className={`contenidoSubTab ${subTab === tab.id ? 'contenidoSubTab--activo' : ''}`}
                            onClick={() => setSubTab(tab.id)}
                        >
                            <Icono size={16} />
                            {tab.label}
                        </Button>
                    );
                })}
            </div>

            <div className="contenidoPanel">
                {subTab === 'servicios' && (
                    <>
                        {error && <div className="contenidoError">{error}</div>}
                        <ListaServicios
                            servicios={servicios}
                            cargando={cargando}
                            onEditar={handleEditarServicio}
                            onCrear={handleCrearServicio}
                            onArchivar={handleArchivarServicio}
                        />
                        <EditorServicio
                            abierto={editorAbierto}
                            onCerrar={() => setEditorAbierto(false)}
                            servicio={servicioEditando}
                            onGuardar={handleGuardarServicio}
                            guardando={guardando}
                        />
                    </>
                )}
                {subTab === 'blog' && (
                    <>
                        {blogError && <div className="contenidoError">{blogError}</div>}
                        <ListaBlog
                            posts={blogPosts}
                            cargando={blogCargando}
                            onEditar={handleEditarPost}
                            onCrear={handleCrearPost}
                            onArchivar={handleArchivarPost}
                        />
                        <EditorBlog
                            abierto={blogEditorAbierto}
                            onCerrar={() => setBlogEditorAbierto(false)}
                            post={postEditando}
                            onGuardar={handleGuardarPost}
                            guardando={blogGuardando}
                        />
                    </>
                )}
                {subTab === 'proyectos' && (
                    <>
                        {proyectosError && <div className="contenidoError">{proyectosError}</div>}
                        <ListaProyectos
                            proyectos={proyectosList}
                            cargando={proyectosCargando}
                            onEditar={handleEditarProyecto}
                            onCrear={handleCrearProyecto}
                            onArchivar={handleArchivarProyecto}
                        />
                        <EditorProyecto
                            abierto={proyectoEditorAbierto}
                            onCerrar={() => setProyectoEditorAbierto(false)}
                            proyecto={proyectoEditando}
                            onGuardar={handleGuardarProyecto}
                            guardando={proyectosGuardando}
                        />
                    </>
                )}
                {subTab !== 'servicios' && subTab !== 'blog' && subTab !== 'proyectos' && renderSubTab(subTab)}
            </div>
        </div>
    );
};

/* [074A-12] Renderiza sub-tabs que aún no tienen editor (equipo).
 * 'servicios', 'blog' y 'proyectos' se manejan directamente en el componente principal. */
function renderSubTab(subTab: SubTab) {
    switch (subTab) {
        case 'equipo':
            return <PlaceholderContenido tipo="Equipo" />;
        default:
            return null;
    }
}

/* Placeholder genérico hasta que cada sección tenga su editor real. */
function PlaceholderContenido({tipo}: {tipo: string}) {
    return (
        <div className="contenidoPlaceholder">
            <p className="contenidoPlaceholderTexto">
                Editor de {tipo} — próximamente.
            </p>
        </div>
    );
}

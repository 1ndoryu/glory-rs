/* [074A-7] Sección "Contenido" del panel admin — CMS editorial.
 * Sub-tabs laterales: Servicios | Blog | Proyectos | Equipo.
 * Cada sub-tab renderiza su propio editor/lista.
 * [074A-9] Sub-tab Servicios conectada a ListaServicios + EditorServicio. */
import React, {useState, useCallback} from 'react';
import {Briefcase, PenTool, FolderOpen, Users} from 'lucide-react';
import {Button} from '../ui/Button';
import {ListaServicios} from './ListaServicios';
import {EditorServicio} from './EditorServicio';
import {useContenidoServicios} from '../../hooks/useContenidoServicios';
import type {AdminService, CreateServiceBody, UpdateServiceBody} from '../../api/admin-services';
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
                {subTab !== 'servicios' && renderSubTab(subTab)}
            </div>
        </div>
    );
};

/* [074A-9] Renderiza sub-tabs que aún no tienen editor (blog, proyectos, equipo).
 * 'servicios' se maneja directamente en el componente principal. */
function renderSubTab(subTab: SubTab) {
    switch (subTab) {
        case 'blog':
            return <PlaceholderContenido tipo="Blog" />;
        case 'proyectos':
            return <PlaceholderContenido tipo="Proyectos" />;
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

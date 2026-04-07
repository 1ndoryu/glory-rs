/* [074A-7] Sección "Contenido" del panel admin — CMS editorial.
 * Sub-tabs laterales: Servicios | Blog | Proyectos | Equipo.
 * Cada sub-tab renderiza su propio editor/lista.
 * Base infraestructural — los editores concretos se implementan en tareas posteriores. */
import React, {useState} from 'react';
import {Briefcase, PenTool, FolderOpen, Users} from 'lucide-react';
import {Button} from '../ui/Button';
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
                {renderSubTab(subTab)}
            </div>
        </div>
    );
};

/* Renderiza el contenido de cada sub-tab. Placeholder hasta que se implementen
 * los editores concretos (074A-8 a 074A-13). */
function renderSubTab(subTab: SubTab) {
    switch (subTab) {
        case 'servicios':
            return <PlaceholderContenido tipo="Servicios" />;
        case 'blog':
            return <PlaceholderContenido tipo="Blog" />;
        case 'proyectos':
            return <PlaceholderContenido tipo="Proyectos" />;
        case 'equipo':
            return <PlaceholderContenido tipo="Equipo" />;
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

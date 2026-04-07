/* [074A-13] Lista de miembros del equipo para el panel admin CMS.
 * Patrón similar a ListaProyectos. */
/* sentinel-disable-file button-nativo: botones de acción del panel admin */

import React from 'react';
import {AdminTeamMember} from '../../api/admin-team';
import {Badge} from '../ui/Badge';
import './ListaEquipo.css';

interface ListaEquipoProps {
    miembros: AdminTeamMember[];
    cargando: boolean;
    onEditar: (m: AdminTeamMember) => void;
    onCrear: () => void;
    onArchivar: (id: string) => void;
}

export const ListaEquipo: React.FC<ListaEquipoProps> = ({miembros, cargando, onEditar, onCrear, onArchivar}) => {
    if (cargando) return <p className="equipoListaCargando">Cargando miembros...</p>;

    return (
        <div className="equipoListaContenedor">
            <div className="equipoListaAcciones">
                <button className="equipoListaCrear" onClick={onCrear}>+ Nuevo miembro</button>
            </div>
            {miembros.length === 0 && <p className="equipoListaVacia">No hay miembros del equipo.</p>}
            <div className="equipoListaGrid">
            {miembros.map(m => (
                <article key={m.id} className="equipoListaCard" onClick={() => onEditar(m)}>
                    <div className="equipoListaAvatar">
                        {m.avatar && <img src={m.avatar} alt={m.name} loading="lazy" />}
                    </div>
                    <div className="equipoListaInfo">
                        <h4 className="equipoListaNombre">{m.name}</h4>
                        <span className="equipoListaCargo">{m.role}</span>
                        <div className="equipoListaMeta">
                            <Badge label={m.status} />
                            <span className="equipoListaOrden">#{m.sort_order}</span>
                        </div>
                    </div>
                    {m.status !== 'archived' && (
                        <button
                            className="equipoListaArchivar"
                            onClick={e => { e.stopPropagation(); onArchivar(m.id); }}
                        >
                            Archivar
                        </button>
                    )}
                </article>
            ))}
            </div>
        </div>
    );
};

/* [074A-9] Lista de servicios en el CMS admin.
 * Grid de cards con status, título, precio. Click para editar, botón + para crear.
 * sentinel-disable-file button-nativo: Botón archivar es icon-only overlay sobre card,
 * botonBase interfiere con posicionamiento absoluto y estilos (mismo patrón UploadImage). */
import React from 'react';
import {Plus, Archive} from 'lucide-react';
import {Button} from '../ui/Button';
import type {AdminService} from '../../api/admin-services';
import './ListaServicios.css';

interface ListaServiciosProps {
    servicios: AdminService[];
    cargando: boolean;
    onEditar: (servicio: AdminService) => void;
    onCrear: () => void;
    onArchivar: (id: string) => void;
}

/* Badge de status con color semántico */
function BadgeStatus({status}: {status: string}) {
    const clase = `listaServiciosBadge listaServiciosBadge--${status}`;
    const labels: Record<string, string> = {
        published: 'Publicado',
        draft: 'Borrador',
        archived: 'Archivado',
    };
    return <span className={clase}>{labels[status] ?? status}</span>;
}

export const ListaServicios: React.FC<ListaServiciosProps> = ({
    servicios,
    cargando,
    onEditar,
    onCrear,
    onArchivar,
}) => {
    if (cargando) {
        return <div className="listaServiciosCargando">Cargando servicios...</div>;
    }

    return (
        <div className="listaServicios">
            <div className="listaServiciosHeader">
                <h3 className="listaServiciosTitulo">Servicios</h3>
                <Button variante="primario" tamano="pequeno" onClick={onCrear}>
                    <Plus size={14} />
                    Nuevo
                </Button>
            </div>

            <div className="listaServiciosGrid">
                {servicios.map(svc => (
                    <div
                        key={svc.id}
                        className={`listaServiciosCard ${!svc.is_active ? 'listaServiciosCard--inactivo' : ''}`}
                        onClick={() => onEditar(svc)}
                        role="button"
                        tabIndex={0}
                        onKeyDown={e => { if (e.key === 'Enter') onEditar(svc); }}
                    >
                        {svc.image_url && (
                            <div className="listaServiciosImagen">
                                <img src={svc.image_url} alt={svc.title} />
                            </div>
                        )}
                        <div className="listaServiciosInfo">
                            <div className="listaServiciosCardHeader">
                                <span className="listaServiciosNombre">{svc.title}</span>
                                <BadgeStatus status={svc.status} />
                            </div>
                            <span className="listaServiciosSlug">/{svc.slug}</span>
                            {svc.description && (
                                <span className="listaServiciosDesc">{svc.description}</span>
                            )}
                            <div className="listaServiciosCardFooter">
                                <span className="listaServiciosPrecio">
                                    ${(svc.base_price_cents / 100).toFixed(0)} {svc.currency}
                                </span>
                                <span className="listaServiciosPlanes">
                                    {svc.plans.length} plan{svc.plans.length !== 1 ? 'es' : ''}
                                </span>
                            </div>
                        </div>
                        {svc.status !== 'archived' && (
                            <button
                                type="button"
                                className="listaServiciosArchivar"
                                title="Archivar servicio"
                                onClick={e => { e.stopPropagation(); onArchivar(svc.id); }}
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

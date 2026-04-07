/* [074A-13] Editor modal para miembros del equipo.
 * Más simple que EditorProyecto: sin tabs, un solo formulario. */
/* sentinel-disable-file button-nativo: botones de formulario del editor */

import React from 'react';
import {useEditorMiembro} from '../../hooks/useEditorMiembro';
import {AdminTeamMember, CreateTeamMemberBody, UpdateTeamMemberBody} from '../../api/admin-team';
import {UploadImage} from '../ui/UploadImage';
import './EditorMiembro.css';

interface EditorMiembroProps {
    miembro: AdminTeamMember | null;
    abierto: boolean;
    guardando: boolean;
    onGuardar: (id: string | null, body: CreateTeamMemberBody | UpdateTeamMemberBody) => void;
    onCerrar: () => void;
}

export const EditorMiembro: React.FC<EditorMiembroProps> = ({miembro, abierto, guardando, onGuardar, onCerrar}) => {
    const editor = useEditorMiembro();

    React.useEffect(() => {
        if (miembro) {
            editor.cargarDesde(miembro);
        } else {
            editor.resetear();
        }
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [miembro]);

    if (!abierto) return null;

    const handleSubmit = () => {
        if (miembro) {
            onGuardar(miembro.id, editor.buildUpdateBody());
        } else {
            onGuardar(null, editor.buildCreateBody());
        }
    };

    return (
        <div className="editorMiembroOverlay" onClick={onCerrar}>
            <div className="editorMiembroModal" onClick={e => e.stopPropagation()}>
                <div className="editorMiembroContenido">
                    <div className="editorMiembroCampo">
                        <label>Nombre</label>
                        <input type="text" value={editor.nombre} onChange={e => editor.setNombre(e.target.value)} placeholder="Nombre del miembro" />
                    </div>
                    <div className="editorMiembroCampo">
                        <label>Slug</label>
                        <input type="text" value={editor.slug} onChange={e => editor.setSlug(e.target.value)} placeholder="slug-unico" />
                    </div>
                    <div className="editorMiembroCampo">
                        <label>Cargo</label>
                        <input type="text" value={editor.cargo} onChange={e => editor.setCargo(e.target.value)} placeholder="CEO, Developer, etc." />
                    </div>
                    <div className="editorMiembroCampo">
                        <label>Bio</label>
                        <textarea value={editor.bio} onChange={e => editor.setBio(e.target.value)} placeholder="Descripción breve del miembro" rows={3} />
                    </div>
                    <div className="editorMiembroCampo">
                        <label>Avatar</label>
                        <UploadImage valor={editor.avatar} onChange={editor.setAvatar} etiqueta="Subir avatar" />
                    </div>
                    <div className="editorMiembroFila">
                        <div className="editorMiembroCampo">
                            <label>LinkedIn</label>
                            <input type="text" value={editor.linkedin} onChange={e => editor.setLinkedin(e.target.value)} placeholder="URL LinkedIn" />
                        </div>
                        <div className="editorMiembroCampo">
                            <label>GitHub</label>
                            <input type="text" value={editor.github} onChange={e => editor.setGithub(e.target.value)} placeholder="URL GitHub" />
                        </div>
                    </div>
                    <div className="editorMiembroFila">
                        <div className="editorMiembroCampo">
                            <label>Twitter</label>
                            <input type="text" value={editor.twitter} onChange={e => editor.setTwitter(e.target.value)} placeholder="URL Twitter/X" />
                        </div>
                        <div className="editorMiembroCampo">
                            <label>Orden</label>
                            <input type="number" value={editor.sortOrder} onChange={e => editor.setSortOrder(Number(e.target.value))} />
                        </div>
                    </div>
                    <div className="editorMiembroCampo">
                        <label>Estado</label>
                        <div className="editorMiembroEstados">
                            {['published', 'draft', 'archived'].map(s => (
                                <button key={s} className={`editorMiembroEstadoBtn ${editor.status === s ? 'activo' : ''}`} onClick={() => editor.setStatus(s)}>
                                    {s === 'published' ? 'Publicado' : s === 'draft' ? 'Borrador' : 'Archivado'}
                                </button>
                            ))}
                        </div>
                    </div>
                </div>
                <div className="editorMiembroAcciones">
                    <button className="editorMiembroBtnCancelar" onClick={onCerrar}>Cancelar</button>
                    <button className="editorMiembroBtnGuardar" onClick={handleSubmit} disabled={guardando || !editor.nombre || !editor.slug}>
                        {guardando ? 'Guardando...' : miembro ? 'Actualizar' : 'Crear'}
                    </button>
                </div>
            </div>
        </div>
    );
};

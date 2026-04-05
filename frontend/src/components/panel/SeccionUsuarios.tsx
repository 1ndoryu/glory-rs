/* [054A-1] Sección admin: gestión de usuarios registrados.
 * Búsqueda por email/nombre, filtros por rol y status, paginación.
 * Acciones: cambiar rol, banear/reactivar. */

import { useState } from 'react';
import { Loader2, AlertCircle, Search, Users, Shield, Ban, UserCheck } from 'lucide-react';
import { useAdminUsers } from '../../hooks/useAdminUsers';
import { ROLE_LABELS, STATUS_LABELS, STATUS_CLASS } from '../../api/admin-users';
import type { AdminUserItem } from '../../api/admin-users';
import { Input } from '../ui/Input';
import { Modal } from '../ui/Modal';
import './SeccionUsuarios.css';

export function SeccionUsuarios() {
    const {
        users, total, page, totalPages, isLoading, error,
        search, roleFilter, statusFilter,
        setSearch, setRoleFilter, setStatusFilter, setPage,
        changeRole, changeStatus, isChangingRole, isChangingStatus,
    } = useAdminUsers();

    const [confirmAction, setConfirmAction] = useState<{
        userId: string;
        type: 'role' | 'status';
        value: string;
        email: string;
    } | null>(null);

    const handleConfirm = () => {
        if (!confirmAction) return;
        if (confirmAction.type === 'role') {
            changeRole({ userId: confirmAction.userId, role: confirmAction.value });
        } else {
            changeStatus({ userId: confirmAction.userId, status: confirmAction.value });
        }
        setConfirmAction(null);
    };

    if (isLoading) {
        return (
            <div className="usuariosVacio">
                <Loader2 className="usuariosSpinner" size={32} />
            </div>
        );
    }

    if (error) {
        return (
            <div className="usuariosError">
                <AlertCircle size={20} />
                <span>Error al cargar usuarios</span>
            </div>
        );
    }

    return (
        <div className="usuariosContenedor">
            {/* Filtros y búsqueda */}
            <div className="usuariosFiltros">
                <div className="usuariosBusqueda">
                    <Search size={16} className="usuariosBusquedaIcono" />
                    <Input
                        type="text"
                        className="usuariosBusquedaInput"
                        placeholder="Buscar por email o nombre..."
                        value={search}
                        onChange={(e) => setSearch(e.target.value)}
                    />
                </div>

                <select
                    className="usuariosFiltroSelect"
                    value={roleFilter}
                    onChange={(e) => setRoleFilter(e.target.value)}
                >
                    <option value="">Todos los roles</option>
                    <option value="admin">Admin</option>
                    <option value="employee">Empleado</option>
                    <option value="client">Cliente</option>
                </select>

                <select
                    className="usuariosFiltroSelect"
                    value={statusFilter}
                    onChange={(e) => setStatusFilter(e.target.value)}
                >
                    <option value="">Todos los status</option>
                    <option value="active">Activo</option>
                    <option value="banned">Baneado</option>
                    <option value="suspended">Suspendido</option>
                </select>

                <span className="usuariosTotal">
                    <Users size={14} />
                    {total} usuario{total !== 1 ? 's' : ''}
                </span>
            </div>

            {/* Tabla de usuarios */}
            {users.length === 0 ? (
                <div className="usuariosVacio">
                    <Users size={32} />
                    <p>No se encontraron usuarios</p>
                </div>
            ) : (
                <div className="usuariosTablaWrapper">
                    <table className="usuariosTabla">
                        <thead>
                            <tr>
                                <th>Usuario</th>
                                <th>Rol</th>
                                <th>Status</th>
                                <th>Registrado</th>
                                <th>Acciones</th>
                            </tr>
                        </thead>
                        <tbody>
                            {users.map((u) => (
                                <UserRow
                                    key={u.id}
                                    user={u}
                                    onChangeRole={(role) => setConfirmAction({
                                        userId: u.id, type: 'role', value: role, email: u.email,
                                    })}
                                    onChangeStatus={(status) => setConfirmAction({
                                        userId: u.id, type: 'status', value: status, email: u.email,
                                    })}
                                />
                            ))}
                        </tbody>
                    </table>
                </div>
            )}

            {/* Paginación */}
            {totalPages > 1 && (
                <div className="usuariosPaginacion">
                    <button
                        className="usuariosPaginacionBtn"
                        disabled={page <= 1}
                        onClick={() => setPage(page - 1)}
                        type="button"
                    >
                        ← Anterior
                    </button>
                    <span className="usuariosPaginacionInfo">
                        Página {page} de {totalPages}
                    </span>
                    <button
                        className="usuariosPaginacionBtn"
                        disabled={page >= totalPages}
                        onClick={() => setPage(page + 1)}
                        type="button"
                    >
                        Siguiente →
                    </button>
                </div>
            )}

            {/* Modal de confirmación */}
            <Modal abierto={!!confirmAction} onCerrar={() => setConfirmAction(null)} className="usuariosModal">
                <h3 className="usuariosModalTitulo">Confirmar acción</h3>
                <p className="usuariosModalTexto">
                    {confirmAction?.type === 'role'
                        ? `¿Cambiar rol de ${confirmAction.email} a ${ROLE_LABELS[confirmAction.value] || confirmAction.value}?`
                        : `¿Cambiar status de ${confirmAction?.email} a ${STATUS_LABELS[confirmAction?.value ?? ''] || confirmAction?.value}?`
                    }
                </p>
                <div className="usuariosModalAcciones">
                    <button
                        className="usuariosModalCancelar"
                        onClick={() => setConfirmAction(null)}
                        type="button"
                    >
                        Cancelar
                    </button>
                    <button
                        className="usuariosModalConfirmar"
                        onClick={handleConfirm}
                        disabled={isChangingRole || isChangingStatus}
                        type="button"
                    >
                        {(isChangingRole || isChangingStatus) ? 'Procesando...' : 'Confirmar'}
                    </button>
                </div>
            </Modal>
        </div>
    );
}

/* Fila individual de usuario con menú de acciones */
function UserRow({ user, onChangeRole, onChangeStatus }: {
    user: AdminUserItem;
    onChangeRole: (role: string) => void;
    onChangeStatus: (status: string) => void;
}) {
    const [menuAbierto, setMenuAbierto] = useState(false);

    const rolesDisponibles = ['admin', 'employee', 'client'].filter(r => r !== user.role);
    const isBanned = user.status === 'banned';

    return (
        <tr className="usuariosFila">
            <td className="usuariosCeldaUsuario">
                <div className="usuariosAvatar">
                    {user.avatar_url
                        ? <img src={user.avatar_url} alt="" className="usuariosAvatarImg" />
                        : <span className="usuariosAvatarPlaceholder">
                            {(user.display_name || user.email)[0].toUpperCase()}
                          </span>
                    }
                </div>
                <div className="usuariosInfo">
                    <span className="usuariosNombre">{user.display_name || '—'}</span>
                    <span className="usuariosEmail">{user.email}</span>
                </div>
            </td>
            <td>
                <span className={`usuariosRolBadge usuariosRol${user.role}`}>
                    {ROLE_LABELS[user.role] || user.role}
                </span>
            </td>
            <td>
                <span className={`usuariosStatusBadge ${STATUS_CLASS[user.status] || ''}`}>
                    {STATUS_LABELS[user.status] || user.status}
                </span>
            </td>
            <td className="usuariosFecha">
                {new Date(user.created_at).toLocaleDateString('es-ES', {
                    day: 'numeric', month: 'short', year: 'numeric',
                })}
            </td>
            <td className="usuariosAcciones">
                <div className="usuariosMenuWrapper">
                    <button
                        className="usuariosMenuBtn"
                        onClick={() => setMenuAbierto(!menuAbierto)}
                        type="button"
                    >
                        ⋯
                    </button>
                    {menuAbierto && (
                        <div className="usuariosMenu" onMouseLeave={() => setMenuAbierto(false)}>
                            {rolesDisponibles.map((r) => (
                                <button
                                    key={r}
                                    className="usuariosMenuItem"
                                    onClick={() => { onChangeRole(r); setMenuAbierto(false); }}
                                    type="button"
                                >
                                    <Shield size={14} />
                                    Cambiar a {ROLE_LABELS[r]}
                                </button>
                            ))}
                            <hr className="usuariosMenuDivider" />
                            {isBanned ? (
                                <button
                                    className="usuariosMenuItem usuariosMenuItemReactivar"
                                    onClick={() => { onChangeStatus('active'); setMenuAbierto(false); }}
                                    type="button"
                                >
                                    <UserCheck size={14} />
                                    Reactivar
                                </button>
                            ) : (
                                <button
                                    className="usuariosMenuItem usuariosMenuItemBan"
                                    onClick={() => { onChangeStatus('banned'); setMenuAbierto(false); }}
                                    type="button"
                                >
                                    <Ban size={14} />
                                    Banear
                                </button>
                            )}
                        </div>
                    )}
                </div>
            </td>
        </tr>
    );
}

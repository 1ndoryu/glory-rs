/**
 * Componente: SidebarPanel
 * Barra lateral de navegacion del panel de usuario.
 * Muestra avatar, nombre, rol y links de navegacion con iconos.
 * [044A-38 Fase 1] Tabs dinámicos por rol + botón switch-role para admin.
 */
import React, {useState, useCallback} from 'react';
import {useTranslation} from 'react-i18next';
import {FolderOpen, Briefcase, Receipt, User, CreditCard, ClipboardList, PackageOpen, Users, Settings, LayoutDashboard, ArrowLeftRight, ArrowRightLeft} from 'lucide-react';
import {obtenerTabsPorRol, obtenerUsuarioActual, type SeccionPanel} from '../../data/panel';
import {useAuthStore} from '../../stores/authStore';
import {apiSwitchRole} from '../../api/auth';
import type {UserRole} from '../../api/auth';
import './SidebarPanel.css';

interface SidebarPanelProps {
    seccionActiva: SeccionPanel;
    onCambiarSeccion: (seccion: SeccionPanel) => void;
}

/* Mapa de iconos por seccion */
const ICONOS_SECCION: Record<SeccionPanel, React.ElementType> = {
    'proyectos': FolderOpen,
    'servicios': Briefcase,
    'pagos': Receipt,
    'perfil': User,
    'metodos-pago': CreditCard,
    'asignados': ClipboardList,
    'disponibles': PackageOpen,
    'delegaciones': ArrowRightLeft,
    'todos-ordenes': ClipboardList,
    'empleados': Users,
    'config-servicios': Settings,
    'dashboard': LayoutDashboard,
};

/* [044A-38 Fase 1] Etiquetas legibles para cada rol */
const ROLE_LABELS: Record<UserRole, string> = {
    admin: 'Admin',
    employee: 'Empleado',
    client: 'Cliente',
};

export const SidebarPanel: React.FC<SidebarPanelProps> = ({seccionActiva, onCambiarSeccion}) => {
    const {t} = useTranslation();
    const usuario = obtenerUsuarioActual();
    const authUser = useAuthStore(s => s.user);
    const actualizarRol = useAuthStore(s => s.actualizarRol);
    const [switchingRole, setSwitchingRole] = useState(false);

    const effectiveRole: UserRole = authUser?.effectiveRole || 'client';
    const isAdmin = authUser?.role === 'admin';
    const tabs = obtenerTabsPorRol(effectiveRole);

    /* [044A-38 Fase 1] Cicla entre los 3 roles: admin → employee → client → admin */
    const handleSwitchRole = useCallback(async () => {
        if (switchingRole || !isAdmin) return;
        const cycle: UserRole[] = ['admin', 'employee', 'client'];
        const currentIdx = cycle.indexOf(effectiveRole);
        const nextRole = cycle[(currentIdx + 1) % cycle.length];

        setSwitchingRole(true);
        try {
            const resp = await apiSwitchRole(nextRole);
            actualizarRol(resp.token, resp.role, resp.effective_role);
            /* Resetear a la primera tab del nuevo rol */
            const newTabs = obtenerTabsPorRol(nextRole);
            if (newTabs.length > 0) {
                onCambiarSeccion(newTabs[0].id);
            }
        } catch {
            /* Error silencioso no permitido — mostrar en consola */
            console.error('[SidebarPanel] Error al cambiar rol');
        } finally {
            setSwitchingRole(false);
        }
    }, [switchingRole, isAdmin, effectiveRole, actualizarRol, onCambiarSeccion]);

    return (
        <aside className="panelSidebar" aria-label={t('accessibility.panel_nav')}>
            {/* Info usuario */}
            <div className="sidebarUsuario">
                <div className="sidebarAvatar">
                    <img
                        src={usuario?.avatar || 'https://i.pravatar.cc/48?u=default'}
                        alt="Avatar"
                    />
                </div>
                <div className="sidebarInfo">
                    <span className="sidebarNombre">{authUser?.email || usuario?.nombre || 'Usuario'}</span>
                    <span className="sidebarRol">{ROLE_LABELS[effectiveRole]}</span>
                </div>
            </div>

            {/* Navegacion con iconos — tabs dinámicos según rol */}
            <nav className="sidebarNav" aria-label={t('accessibility.panel_sections')}>
                {tabs.map(tab => {
                    const Icono = ICONOS_SECCION[tab.id];
                    return (
                        <button
                            key={tab.id}
                            className={`sidebarItem ${seccionActiva === tab.id ? 'sidebarItemActivo' : ''}`}
                            onClick={() => onCambiarSeccion(tab.id)}
                            aria-current={seccionActiva === tab.id ? 'page' : undefined}
                        >
                            <Icono size={18} className="sidebarItemIcono" aria-hidden="true" />
                            <span className="sidebarItemTexto">{tab.label}</span>
                        </button>
                    );
                })}
            </nav>

            {/* [044A-38 Fase 1] Botón switch-role — solo visible para admin */}
            {isAdmin && (
                <div className="sidebarSwitchRole">
                    <button
                        className="sidebarSwitchBtn"
                        onClick={handleSwitchRole}
                        disabled={switchingRole}
                        title={`Cambiar vista a otro rol (actual: ${ROLE_LABELS[effectiveRole]})`}
                    >
                        <ArrowLeftRight size={16} aria-hidden="true" />
                        <span>{switchingRole ? 'Cambiando...' : `Vista: ${ROLE_LABELS[effectiveRole]}`}</span>
                    </button>
                </div>
            )}
        </aside>
    );
};

/**
 * Componente: SidebarPanel
 * Barra lateral de navegacion del panel de usuario.
 * Muestra avatar, nombre, rol y links de navegacion con iconos.
 * [044A-38 Fase 1] Tabs dinámicos por rol + botón switch-role para admin.
 */
import React, {useState, useCallback} from 'react';
import {useTranslation} from 'react-i18next';
import {FolderOpen, Receipt, User, CreditCard, ClipboardList, PackageOpen, ArrowRightLeft, MessageSquare, RotateCcw, UserCog, Server} from 'lucide-react';
import {obtenerTabsPorRol, type SeccionPanel} from '../../data/panel';
import {useCurrentProfile} from '../../hooks/useCurrentProfile';
import {useAuthStore} from '../../stores/authStore';
import {apiSwitchRole} from '../../api/auth';
import type {UserRole} from '../../api/auth';
import {Button} from '../ui/Button';
import './SidebarPanel.css';

interface SidebarPanelProps {
    seccionActiva: SeccionPanel;
    onCambiarSeccion: (seccion: SeccionPanel) => void;
}

/* [064A-34] Mapa de iconos por seccion. Eliminados: servicios, empleados, config-servicios. */
const ICONOS_SECCION: Record<SeccionPanel, React.ElementType> = {
    'proyectos': FolderOpen,
    'pagos': Receipt,
    'perfil': User,
    'metodos-pago': CreditCard,
    'asignados': ClipboardList,
    'disponibles': PackageOpen,
    'delegaciones': ArrowRightLeft,
    'mensajes': MessageSquare,
    'todos-ordenes': ClipboardList,
    'reembolsos': RotateCcw,
    'usuarios': UserCog,
    'hosting': Server,
};

/* [044A-38 Fase 1] Etiquetas legibles para cada rol */
const ROLE_LABELS: Record<UserRole, string> = {
    admin: 'Admin',
    employee: 'Empleado',
    client: 'Cliente',
};

export const SidebarPanel: React.FC<SidebarPanelProps> = ({seccionActiva, onCambiarSeccion}) => {
    const {t} = useTranslation();
    const {perfil, avatarUrl} = useCurrentProfile();
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
                        src={avatarUrl}
                        alt="Avatar"
                    />
                </div>
                <div className="sidebarInfo">
                    <span className="sidebarNombre">{perfil?.display_name || authUser?.email || 'Usuario'}</span>
                    <span className="sidebarRol">{ROLE_LABELS[effectiveRole]}</span>
                </div>
            </div>

            {/* Navegacion con iconos — tabs dinámicos según rol */}
            <nav className="sidebarNav" aria-label={t('accessibility.panel_sections')}>
                {tabs.map(tab => {
                    const Icono = ICONOS_SECCION[tab.id];
                    return (
                        <Button
                            key={tab.id}
                            className={`sidebarItem ${seccionActiva === tab.id ? 'sidebarItemActivo' : ''}`}
                            onClick={() => onCambiarSeccion(tab.id)}
                            aria-current={seccionActiva === tab.id ? 'page' : undefined}
                            variante="texto"
                        >
                            <Icono size={18} className="sidebarItemIcono" aria-hidden="true" />
                            <span className="sidebarItemTexto">{tab.label}</span>
                        </Button>
                    );
                })}
            </nav>

            {/* [044A-38 Fase 1] Botón switch-role — solo visible para admin */}
            {isAdmin && (
                <div className="sidebarSwitchRole">
                    <Button
                        className="sidebarSwitchBtn"
                        onClick={handleSwitchRole}
                        disabled={switchingRole}
                        title={`Cambiar vista a otro rol (actual: ${ROLE_LABELS[effectiveRole]})`}
                        variante="texto"
                    >
                        <ArrowRightLeft size={16} aria-hidden="true" />
                        <span>{switchingRole ? 'Cambiando...' : `Vista: ${ROLE_LABELS[effectiveRole]}`}</span>
                    </Button>
                </div>
            )}
        </aside>
    );
};

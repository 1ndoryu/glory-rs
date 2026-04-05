/**
 * Island: PanelIsland
 * Panel de usuario con header custom (sin header/footer global) y sidebar lateral.
 * [044A-38 Fase 1] Secciones dinámicas por rol (admin/employee/client).
 * Redirige a / si no hay sesión activa. Tabs y sección inicial dependen del effectiveRole.
 */
import React, {useState, useEffect} from 'react';
import {useNavigate} from 'react-router-dom';
import {HeaderPanel} from '../components/panel/HeaderPanel';
import {SeccionPerfil} from '../components/panel/SeccionPerfil';
import {SeccionMetodosPago} from '../components/panel/SeccionMetodosPago';
import {SeccionProyectos} from '../components/panel/SeccionProyectos';
import {SeccionPagos} from '../components/panel/SeccionPagos';
import {SeccionDisponibles} from '../components/panel/SeccionDisponibles';
import {SeccionDelegaciones} from '../components/panel/SeccionDelegaciones';
import {SeccionChat} from '../components/panel/SeccionChat';
import {SeccionReembolsos} from '../components/panel/SeccionReembolsos';
import SeccionDashboard from '../components/panel/SeccionDashboard';
import {SeccionUsuarios} from '../components/panel/SeccionUsuarios';
import {SeccionHosting} from '../components/panel/SeccionHosting';
import {EmployeesSection} from '../components/panel/EmployeesSection';
import {ServicesCatalogSection} from '../components/panel/ServicesCatalogSection';
import {SidebarPanel} from '../components/panel/SidebarPanel';
import {PlaceholderSeccion} from '../components/panel/PlaceholderSeccion';
import {obtenerTabsPorRol, seccionInicialPorRol, type SeccionPanel} from '../data/panel';
import {useAuthStore} from '../stores/authStore';
import {SEOHead} from '../components/seo/SEOHead';
import type {UserRole} from '../api/auth';
import '../styles/variables.css';
import './PanelIsland.css';

export const PanelIsland: React.FC = () => {
    const logueado = useAuthStore(s => s.logueado);
    const effectiveRole: UserRole = useAuthStore(s => s.user?.effectiveRole) || 'client';
    const navigate = useNavigate();

    const tabs = obtenerTabsPorRol(effectiveRole);
    const [seccionActiva, setSeccionActiva] = useState<SeccionPanel>(() => seccionInicialPorRol(effectiveRole));

    /* [044A-38 Fase 1] Si no hay sesión, redirigir al home */
    useEffect(() => {
        if (!logueado) {
            navigate('/', {replace: true});
        }
    }, [logueado, navigate]);

    /* [044A-38 Fase 1] Cuando cambia el rol efectivo, resetear a la primera tab del nuevo rol */
    useEffect(() => {
        setSeccionActiva(seccionInicialPorRol(effectiveRole));
    }, [effectiveRole]);

    const tabActual = tabs.find(t => t.id === seccionActiva) || tabs[0];

    /* Renderizar contenido segun seccion activa */
    const renderContenido = () => {
        switch (seccionActiva) {
            case 'perfil':
                return <SeccionPerfil />;
            case 'metodos-pago':
                return <SeccionMetodosPago />;
            case 'servicios':
                return <ServicesCatalogSection mode="client" />;
            /* [044A-38 Fase 2] Mis Proyectos con lista de órdenes + detalle + acciones */
            case 'proyectos':
            case 'asignados':
            case 'todos-ordenes':
                return <SeccionProyectos />;
            /* [044A-38 Fase 3] Historial de pagos por orden */
            case 'pagos':
                return <SeccionPagos />;
            /* [044A-38 Fase 4] Órdenes disponibles para tomar (empleado) */
            case 'disponibles':
                return <SeccionDisponibles />;
            /* [044A-38 Fase 4] Delegaciones entre empleados */
            case 'delegaciones':
                return <SeccionDelegaciones />;
            /* [044A-38 Fase 5] Chat integrado con ordenes */
            case 'mensajes':
                return <SeccionChat />;
            /* [044A-38 Fase 7] Reembolsos (admin) */
            case 'reembolsos':
                return <SeccionReembolsos />;
            /* [044A-38 Fase 10] Dashboard admin — métricas, revenue, alertas */
            case 'dashboard':
                return <SeccionDashboard />;
            case 'empleados':
                return <EmployeesSection />;
            case 'config-servicios':
                return <ServicesCatalogSection mode="admin" />;
            /* [054A-1] Gestión de usuarios registrados (admin) */
            case 'usuarios':
                return <SeccionUsuarios />;
            /* [054A-2] Hosting: suscripciones, planes, estados, eventos */
            case 'hosting':
                return <SeccionHosting />;
            default:
                return <PlaceholderSeccion tab={tabActual} />;
        }
    };

    if (!logueado) return null;

    return (
        <>
            <SEOHead title="Panel" noindex />
            <HeaderPanel />
            <section id="panelUsuario" className="panelContenedor">
                <div className="panelLayout">
                    <SidebarPanel
                        seccionActiva={seccionActiva}
                        onCambiarSeccion={setSeccionActiva}
                    />

                    <main className="panelContenidoPrincipal">
                        <div className="panelContenidoCabecera">
                            <h1 className="panelTitulo">{tabActual.label}</h1>
                        </div>
                        <div className="panelContenido">
                            {renderContenido()}
                        </div>
                    </main>
                </div>
            </section>
        </>
    );
};

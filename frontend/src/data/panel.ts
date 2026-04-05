/**
 * Datos y tipos para el Panel de usuario.
 * Configuracion de tabs por rol, helper de usuario actual.
 * [044A-38 Fase 1] Tabs dinámicos por rol (admin/employee/client).
 */
import type {UserRole} from '../api/auth';

export type SeccionPanel =
    | 'proyectos' | 'servicios' | 'pagos' | 'perfil' | 'metodos-pago'
    | 'asignados' | 'disponibles'
    | 'todos-ordenes' | 'empleados' | 'config-servicios' | 'dashboard';

export interface TabConfig {
    id: SeccionPanel;
    label: string;
    descripcion: string;
}

/* [044A-38 Fase 1] Tabs para cliente */
const TABS_CLIENT: TabConfig[] = [
    {
        id: 'proyectos',
        label: 'Mis Proyectos',
        descripcion: 'Aqui podras ver el estado de tus proyectos en progreso y los finalizados. Seguimiento en tiempo real del avance de cada servicio contratado.'
    },
    {
        id: 'servicios',
        label: 'Servicios',
        descripcion: 'Gestiona tus servicios contratados: hosting, VPS, desarrollo web y mas. Consulta detalles, renueva o amplia tus planes.'
    },
    {
        id: 'pagos',
        label: 'Historial de Pagos',
        descripcion: 'Historial completo de pagos realizados. Facturas, recibos y metodos de pago asociados a tu cuenta.'
    },
    {
        id: 'perfil',
        label: 'Configurar Perfil',
        descripcion: 'Personaliza tu perfil publico: nombre, imagen, descripcion y redes sociales.'
    },
    {
        id: 'metodos-pago',
        label: 'Metodos de Pago',
        descripcion: 'Administra tus tarjetas de credito, direccion de facturacion y metodos de pago registrados.'
    }
];

/* [044A-38 Fase 1] Tabs para empleado */
const TABS_EMPLOYEE: TabConfig[] = [
    {
        id: 'asignados',
        label: 'Asignados',
        descripcion: 'Ordenes asignadas a ti. Revisa el estado, entrega fases y comunica con el cliente.'
    },
    {
        id: 'disponibles',
        label: 'Disponibles',
        descripcion: 'Ordenes sin asignar disponibles para tomar. Revisa requisitos y acepta nuevos proyectos.'
    },
    {
        id: 'perfil',
        label: 'Configurar Perfil',
        descripcion: 'Personaliza tu perfil publico: nombre, imagen, descripcion y redes sociales.'
    }
];

/* [044A-38 Fase 1] Tabs para admin: todo el sistema */
const TABS_ADMIN: TabConfig[] = [
    {
        id: 'todos-ordenes',
        label: 'Todas las Ordenes',
        descripcion: 'Vista completa de todas las ordenes del sistema. Filtra por estado, cliente o empleado.'
    },
    {
        id: 'empleados',
        label: 'Empleados',
        descripcion: 'Gestiona el equipo: asigna ordenes, revisa carga de trabajo y metricas de rendimiento.'
    },
    {
        id: 'config-servicios',
        label: 'Servicios',
        descripcion: 'Configura el catalogo de servicios: planes, precios, fases y visibilidad.'
    },
    {
        id: 'dashboard',
        label: 'Dashboard',
        descripcion: 'Metricas generales: revenue, ordenes activas, satisfaccion del cliente y alertas.'
    },
    {
        id: 'perfil',
        label: 'Configurar Perfil',
        descripcion: 'Personaliza tu perfil publico: nombre, imagen, descripcion y redes sociales.'
    }
];

/* [044A-38 Fase 1] Devuelve las tabs correspondientes al rol efectivo */
export function obtenerTabsPorRol(role: UserRole): TabConfig[] {
    switch (role) {
        case 'admin': return TABS_ADMIN;
        case 'employee': return TABS_EMPLOYEE;
        default: return TABS_CLIENT;
    }
}

/* Legacy: mantiene compat con imports existentes que aún usen TABS_PANEL */
export const TABS_PANEL: TabConfig[] = TABS_CLIENT;

/* [044A-38 Fase 1] Devuelve la seccion por defecto según el rol */
export function seccionInicialPorRol(role: UserRole): SeccionPanel {
    switch (role) {
        case 'admin': return 'todos-ordenes';
        case 'employee': return 'asignados';
        default: return 'proyectos';
    }
}

export interface UsuarioActual {
    id: number;
    nombre: string;
    email: string;
    avatar: string;
    rol: string;
}

/* [044A-1] TO-DO: Conectar con API Rust para obtener usuario actual */
export function obtenerUsuarioActual(): UsuarioActual | null {
    return null;
}

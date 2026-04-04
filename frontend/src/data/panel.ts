/**
 * Datos y tipos para el Panel de usuario.
 * Configuracion de tabs, helper de usuario actual.
 */

export type SeccionPanel = 'proyectos' | 'servicios' | 'pagos' | 'perfil' | 'metodos-pago';

export interface TabConfig {
    id: SeccionPanel;
    label: string;
    descripcion: string;
}

export const TABS_PANEL: TabConfig[] = [
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

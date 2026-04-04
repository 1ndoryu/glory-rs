/* [044A-13] Servicio API de autenticación.
 * Conecta con POST /api/auth/login y POST /api/auth/register del backend Rust. */
import instance from './axios-instance';

interface AuthResponse {
    token: string;
    user_id: string;
}

interface ApiError {
    error: string;
}

export async function apiLogin(email: string, password: string): Promise<AuthResponse> {
    const {data} = await instance.post<AuthResponse>('/api/auth/login', {email, password});
    return data;
}

export async function apiRegister(email: string, password: string): Promise<AuthResponse> {
    const {data} = await instance.post<AuthResponse>('/api/auth/register', {email, password});
    return data;
}

/* Extrae mensaje de error legible desde respuestas del backend */
export function extraerMensajeError(error: unknown): string {
    if (typeof error === 'object' && error !== null && 'response' in error) {
        const resp = (error as { response?: { data?: ApiError; status?: number } }).response;
        if (resp?.data?.error) return resp.data.error;
        if (resp?.status === 401) return 'Credenciales inválidas';
        if (resp?.status === 409) return 'El email ya está registrado';
        if (resp?.status === 422) return 'Datos de formulario inválidos';
    }
    return 'Error de conexión. Intenta de nuevo.';
}

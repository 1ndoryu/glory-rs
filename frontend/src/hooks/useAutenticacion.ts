/**
 * Hook: useAutenticacion
 * Encapsula toda la logica de estado del modal de autenticacion.
 * Reduce useState en ModalAutenticacion de 9 a 0 (SRP).
 * [044A-13] Conectado con backend REST API (login/registro via JWT).
 * Pendiente: OAuth Google, recuperación de contraseña.
 */
import {useState, useEffect, useRef, useCallback} from 'react';
import {useFocusTrap} from './useFocusTrap';
import {apiLogin, apiRegister, extraerMensajeError} from '../api/auth';
import {useAuthStore} from '../stores/authStore';

export type VistaModal = 'login' | 'registro' | 'recuperar';

interface EstadoLogin {
    email: string;
    password: string;
}

interface EstadoRegistro {
    nombre: string;
    email: string;
    password: string;
    confirmar: string;
}

interface EstadoRecuperar {
    email: string;
    enviado: boolean;
}

interface RetornoUseAutenticacion {
    vista: VistaModal;
    setVista: (v: VistaModal) => void;
    cargando: boolean;
    error: string | null;
    modalRef: React.RefObject<HTMLDivElement>;
    login: EstadoLogin;
    registro: EstadoRegistro;
    recuperar: EstadoRecuperar;
    actualizarLogin: (campo: keyof EstadoLogin, valor: string) => void;
    actualizarRegistro: (campo: keyof EstadoRegistro, valor: string) => void;
    actualizarRecuperar: (campo: keyof EstadoRecuperar, valor: string) => void;
    handleLogin: (e: React.FormEvent) => void;
    handleRegistro: (e: React.FormEvent) => void;
    handleRecuperar: (e: React.FormEvent) => void;
    handleGoogleLogin: () => void;
    resetRecuperacion: () => void;
}

export const useAutenticacion = (abierto: boolean, onCerrar: () => void): RetornoUseAutenticacion => {
    const [vista, setVista] = useState<VistaModal>('login');
    const [cargando, setCargando] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const modalRef = useRef<HTMLDivElement>(null);
    const authLogin = useAuthStore(s => s.login);

    const [login, setLogin] = useState<EstadoLogin>({email: '', password: ''});

    /* Focus trap: atrapa Tab dentro del modal cuando está abierto */
    useFocusTrap(modalRef, abierto);
    const [registro, setRegistro] = useState<EstadoRegistro>({nombre: '', email: '', password: '', confirmar: ''});
    const [recuperar, setRecuperar] = useState<EstadoRecuperar>({email: '', enviado: false});

    /* Cerrar con Escape y bloquear scroll */
    useEffect(() => {
        const handleEsc = (e: KeyboardEvent) => {
            if (e.key === 'Escape') onCerrar();
        };
        if (abierto) {
            document.addEventListener('keydown', handleEsc);
            document.body.style.overflow = 'hidden';
        }
        return () => {
            document.removeEventListener('keydown', handleEsc);
            document.body.style.overflow = '';
        };
    }, [abierto, onCerrar]);

    const actualizarLogin = useCallback((campo: keyof EstadoLogin, valor: string) => {
        setLogin(prev => ({...prev, [campo]: valor}));
    }, []);

    const actualizarRegistro = useCallback((campo: keyof EstadoRegistro, valor: string) => {
        setRegistro(prev => ({...prev, [campo]: valor}));
    }, []);

    const actualizarRecuperar = useCallback((campo: keyof EstadoRecuperar, valor: string) => {
        setRecuperar(prev => ({...prev, [campo]: valor}));
    }, []);

    /* [044A-13] Login real contra backend REST API
     * [044A-38 Fase 1] Ahora pasa role/effective_role al store */
    const handleLogin = useCallback(async (e: React.FormEvent) => {
        e.preventDefault();
        setError(null);
        setCargando(true);
        try {
            const resp = await apiLogin(login.email, login.password);
            authLogin(resp.token, resp.user_id, login.email, resp.role, resp.effective_role);
            onCerrar();
        } catch (err) {
            setError(extraerMensajeError(err));
        } finally {
            setCargando(false);
        }
    }, [login.email, login.password, authLogin, onCerrar]);

    /* [044A-13] Registro real contra backend REST API
     * [044A-38 Fase 1] Ahora pasa role/effective_role al store */
    const handleRegistro = useCallback(
        async (e: React.FormEvent) => {
            e.preventDefault();
            if (registro.password !== registro.confirmar) {
                setError('Las contraseñas no coinciden.');
                return;
            }
            setError(null);
            setCargando(true);
            try {
                const resp = await apiRegister(registro.email, registro.password);
                authLogin(resp.token, resp.user_id, registro.email, resp.role, resp.effective_role);
                onCerrar();
            } catch (err) {
                setError(extraerMensajeError(err));
            } finally {
                setCargando(false);
            }
        },
        [registro.email, registro.password, registro.confirmar, authLogin, onCerrar]
    );

    const handleRecuperar = useCallback((e: React.FormEvent) => {
        e.preventDefault();
        setCargando(true);
        setTimeout(() => {
            setCargando(false);
            setRecuperar(prev => ({...prev, enviado: true}));
        }, 500);
    }, []);

    const handleGoogleLogin = useCallback(() => {
        alert('Inicio de sesion con Google pendiente de configuracion OAuth.');
    }, []);

    const resetRecuperacion = useCallback(() => {
        setVista('login');
        setRecuperar({email: '', enviado: false});
    }, []);

    return {
        vista,
        setVista,
        cargando,
        error,
        modalRef,
        login,
        registro,
        recuperar,
        actualizarLogin,
        actualizarRegistro,
        actualizarRecuperar,
        handleLogin,
        handleRegistro,
        handleRecuperar,
        handleGoogleLogin,
        resetRecuperacion
    };
};

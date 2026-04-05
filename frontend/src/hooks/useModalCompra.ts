/* [044A-40] Hook para la lógica del modal de compra.
 * Maneja: pasos del modal, auth inline, creación de orden e inicio de pago.
 * Extraído de ModalCompra.tsx para cumplir SRP (max 3 useState en componente). */
import {useState} from 'react';
import {useAuthStore} from '../stores/authStore';
import {apiRegister, apiLogin} from '../api/auth';
import {apiCreateOrder} from '../api/orders';
import {apiInitiatePayment} from '../api/payments';
import {navegar} from '../navegacionSPA';
import type {PlanServicio} from '../data/planes/tipos';

export type PasoModal = 'resumen' | 'auth' | 'procesando' | 'error';

interface UseModalCompraParams {
    plan: PlanServicio;
    servicioSlug: string;
    onClose: () => void;
}

export function useModalCompra({plan, servicioSlug, onClose}: UseModalCompraParams) {
    const logueado = useAuthStore(s => s.logueado);
    const login = useAuthStore(s => s.login);

    const [paso, setPaso] = useState<PasoModal>('resumen');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [errorMsg, setErrorMsg] = useState('');

    /* Crear orden y redirigir a pasarela de pago */
    const crearOrdenYPagar = async () => {
        setPaso('procesando');
        try {
            const orden = await apiCreateOrder({
                service_slug: servicioSlug,
                plan_slug: plan.id,
                payment_mode: 'full',
                client_notes: undefined
            });

            /* Intentar iniciar pago Stripe */
            try {
                await apiInitiatePayment(orden.id, {phase_number: 1});
                /* TO-DO: Integrar Stripe Elements con client_secret para confirmación de pago.
                 * Por ahora redirigir al panel donde puede ver la orden pendiente de pago. */
            } catch {
                /* Si Stripe no está configurado, ir directo al panel */
            }

            navegar('/panel');
            onClose();
        } catch (err: unknown) {
            setPaso('error');
            const msg = err instanceof Error ? err.message : 'Error al crear la orden.';
            setErrorMsg(msg);
        }
    };

    /* Paso 1: usuario confirma que quiere continuar */
    const handleContinuar = () => {
        if (logueado) {
            crearOrdenYPagar();
        } else {
            setPaso('auth');
        }
    };

    /* Paso 2 (solo si no logueado): registrar o login + crear orden */
    const handleAuth = async (e: React.FormEvent) => {
        e.preventDefault();
        setErrorMsg('');
        setPaso('procesando');

        try {
            let authResp;
            try {
                authResp = await apiLogin(email, password);
            } catch {
                authResp = await apiRegister(email, password);
            }
            login(authResp.token, authResp.user_id, email, authResp.role, authResp.effective_role);
            await crearOrdenYPagar();
        } catch (err: unknown) {
            setPaso('error');
            const msg = err instanceof Error ? err.message : 'Error al procesar. Intenta de nuevo.';
            setErrorMsg(msg);
        }
    };

    const reintentar = () => setPaso('resumen');

    return {
        paso,
        email,
        setEmail,
        password,
        setPassword,
        errorMsg,
        handleContinuar,
        handleAuth,
        reintentar
    };
}

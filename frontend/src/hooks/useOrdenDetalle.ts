/* [064A-30] Hook para OrdenDetalle: extrae estado y lógica del componente.
 * Maneja checkout, menú contextual, modales de cancelación y reporte. */
import {useState} from 'react';
import type {OrderResponse} from '../api/orders';

interface CheckoutState {
    amountCents: number;
    phaseNumber?: number;
}

export function useOrdenDetalle(order: OrderResponse, onCancelar: (id: string) => Promise<void>) {
    const [checkout, setCheckout] = useState<CheckoutState | null>(null);
    const [menuAbierto, setMenuAbierto] = useState(false);
    const [modalCancelarAbierto, setModalCancelarAbierto] = useState(false);
    const [modalReportarAbierto, setModalReportarAbierto] = useState(false);

    const confirmarCancelacion = async () => {
        await onCancelar(order.id);
        setModalCancelarAbierto(false);
    };

    const abrirCheckout = (amountCents: number, phaseNumber?: number) => {
        setCheckout({amountCents, phaseNumber});
    };

    const cerrarCheckout = () => setCheckout(null);

    return {
        checkout,
        menuAbierto,
        setMenuAbierto,
        modalCancelarAbierto,
        setModalCancelarAbierto,
        modalReportarAbierto,
        setModalReportarAbierto,
        confirmarCancelacion,
        abrirCheckout,
        cerrarCheckout,
    };
}

/* [044A-40] Modal de compra de servicio.
 * Flujo: muestra resumen del plan → si no logueado pide email+contraseña → crea orden → redirige a pago.
 * Se abre al hacer click en CTA de un plan (SeccionPlanesServicio).
 * Usa componente <Modal> base y hook useModalCompra para la lógica. */
import React from 'react';
import {useTranslation} from 'react-i18next';
import {Modal} from '../ui/Modal';
import {Button} from '../ui/Button';
import {Input} from '../ui/Input';
import {useModalCompra} from '../../hooks/useModalCompra';
import type {PlanServicio} from '../../data/planes/tipos';
import './ModalCompra.css';

interface ModalCompraProps {
    plan: PlanServicio;
    servicioSlug: string;
    abierto: boolean;
    onCerrar: () => void;
}

export const ModalCompra: React.FC<ModalCompraProps> = ({plan, servicioSlug, abierto, onCerrar}) => {
    const {t} = useTranslation();
    const {
        paso, email, setEmail, password, setPassword,
        errorMsg, handleContinuar, handleAuth, reintentar
    } = useModalCompra({plan, servicioSlug, onClose: onCerrar});

    return (
        <Modal abierto={abierto} onCerrar={onCerrar} className="modalCompraContenido">
            {/* Resumen del plan */}
            {(paso === 'resumen' || paso === 'auth') && (
                <div className="modalCompraResumen">
                    <h2 className="modalCompraTitulo">{plan.nombre}</h2>
                    <p className="modalCompraDescripcion">{plan.descripcion}</p>
                    <div className="modalCompraPrecio">
                        <span className="modalCompraPrecioCifra">{plan.precio}</span>
                        {plan.periodo && <span className="modalCompraPrecioPeriodo">{plan.periodo}</span>}
                    </div>
                </div>
            )}

            {/* Paso resumen: botón continuar */}
            {paso === 'resumen' && (
                <div className="modalCompraAcciones">
                    <Button variante="primario" tamano="mediano" onClick={handleContinuar}>
                        {t('purchase.continue', 'Continuar')} ({plan.precio})
                    </Button>
                    <p className="modalCompraAviso">{t('purchase.no_charge_yet', 'Aún no se te cobrará')}</p>
                </div>
            )}

            {/* Paso auth: email + contraseña para guest checkout */}
            {paso === 'auth' && (
                <form className="modalCompraAuth" onSubmit={handleAuth}>
                    <p className="modalCompraAuthTexto">
                        {t('purchase.create_account', 'Crea una cuenta o inicia sesión para continuar')}
                    </p>
                    <Input
                        type="email"
                        placeholder={t('auth.email_placeholder', 'tu@email.com')}
                        value={email}
                        onChange={e => setEmail(e.target.value)}
                        required
                    />
                    <Input
                        type="password"
                        placeholder={t('auth.password_placeholder', 'Contraseña (mín. 8 caracteres)')}
                        value={password}
                        onChange={e => setPassword(e.target.value)}
                        required
                        minLength={8}
                    />
                    <Button variante="primario" tamano="mediano" type="submit">
                        {t('purchase.continue_pay', 'Continuar al pago')} ({plan.precio})
                    </Button>
                    <p className="modalCompraAviso">{t('purchase.no_charge_yet', 'Aún no se te cobrará')}</p>
                </form>
            )}

            {/* Procesando */}
            {paso === 'procesando' && (
                <div className="modalCompraProcesando">
                    <p>{t('purchase.processing', 'Procesando tu orden...')}</p>
                </div>
            )}

            {/* Error */}
            {paso === 'error' && (
                <div className="modalCompraError">
                    <p className="modalCompraErrorTexto">{errorMsg}</p>
                    <Button variante="outline" tamano="pequeno" onClick={reintentar}>
                        {t('common.retry', 'Intentar de nuevo')}
                    </Button>
                </div>
            )}
        </Modal>
    );
};

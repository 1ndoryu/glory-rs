/* [064A-62] Sección admin: configuración y herramientas de desarrollo.
 * Botones para recrear/borrar datos de prueba (seed).
 * Solo visible para admin.
 * [084A-11] Lógica extraída a useSeccionConfiguracion (max 3 useState). */

import { Loader2, AlertCircle, CheckCircle2, Database, Trash2 } from 'lucide-react';
import { useSeccionConfiguracion } from '../../hooks/useSeccionConfiguracion';
import { Button } from '../ui/Button';
import { Modal } from '../ui/Modal';
import './SeccionConfiguracion.css';

export function SeccionConfiguracion() {
    const { cargando, resultado, errorMsg, confirmando, setConfirmando, ejecutarSeed } = useSeccionConfiguracion();

    return (
        <div className="configSeccion">
            <div className="configBloque">
                <h2 className="configBloqueTitle">Datos de Prueba</h2>
                <p className="configBloqueDesc">
                    Gestiona los datos de prueba del sistema. Recrear crea usuarios y órdenes
                    de ejemplo para testear flujos. Borrar elimina únicamente los datos generados por seed.
                </p>

                <div className="configAcciones">
                    <Button
                        onClick={() => setConfirmando('recrear')}
                        disabled={cargando}
                    >
                        {cargando ? <Loader2 size={16} className="configSpinner" /> : <Database size={16} />}
                        Recrear Datos de Prueba
                    </Button>

                    <Button
                        variante="secundario"
                        onClick={() => setConfirmando('borrar')}
                        disabled={cargando}
                    >
                        {cargando ? <Loader2 size={16} className="configSpinner" /> : <Trash2 size={16} />}
                        Borrar Datos de Prueba
                    </Button>
                </div>

                {resultado && (
                    <div className="configMensaje configMensajeExito">
                        <CheckCircle2 size={16} />
                        <span>{resultado}</span>
                    </div>
                )}

                {errorMsg && (
                    <div className="configMensaje configMensajeError">
                        <AlertCircle size={16} />
                        <span>{errorMsg}</span>
                    </div>
                )}
            </div>

            <Modal
                abierto={confirmando !== null}
                onCerrar={() => setConfirmando(null)}
            >
                <div className="configConfirmContenido">
                    <p className="configConfirmTexto">
                        {confirmando === 'recrear'
                            ? '¿Recrear datos de prueba? Se borrarán los existentes y se crearán nuevos usuarios y órdenes de test.'
                            : '¿Borrar todos los datos de prueba? Esta acción elimina usuarios test y sus órdenes/fases/chat.'}
                    </p>
                    <div className="configConfirmAcciones">
                        <Button variante="secundario" onClick={() => setConfirmando(null)}>
                            Cancelar
                        </Button>
                        <Button onClick={() => ejecutarSeed(confirmando)}>
                            Confirmar
                        </Button>
                    </div>
                </div>
            </Modal>
        </div>
    );
}

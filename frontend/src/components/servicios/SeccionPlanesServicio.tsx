/* [044A-40] Componente: SeccionPlanesServicio
 * Muestra las tarjetas de precios de un servicio.
 * Al hacer click en CTA abre ModalCompra en vez de redirigir a /contacto/.
 * Incluye botón "Conversar" debajo de cada CTA. */
import React, {useState} from 'react';
import {useTranslation} from 'react-i18next';
import {obtenerPlanesServicio, type PlanServicio} from '../../data/planes/index';
import {Button} from '../ui/Button';
import {navegar} from '../../navegacionSPA';
import {ModalCompra} from './ModalCompra';
import './SeccionPlanesServicio.css';

interface SeccionPlanesServicioProps {
    slug: string;
}

interface TarjetaPlanProps {
    plan: PlanServicio;
    onSeleccionar: (plan: PlanServicio) => void;
}

const TarjetaPlan: React.FC<TarjetaPlanProps> = ({plan, onSeleccionar}) => {
    const {t} = useTranslation();
    const claseDestacado = plan.destacado ? 'tarjetaPlanDestacado' : '';

    const handleClickCTA = () => {
        onSeleccionar(plan);
    };

    /* [044A-40] Botón conversar: abre chat con contexto del plan seleccionado */
    const handleConversar = () => {
        /* TO-DO: Abrir chat con contexto de servicio/plan. Por ahora redirige a contacto. */
        navegar('/contacto/');
    };

    return (
        <div className={`tarjetaPlan ${claseDestacado}`}>
            {plan.destacado && <div className="tarjetaPlanBadge">{t('plans.recommended')}</div>}

            <div className="tarjetaPlanCabecera">
                <h3 className="tarjetaPlanNombre">{plan.nombre}</h3>
                <div className="tarjetaPlanPrecio">
                    <span className="tarjetaPlanPrecioCifra">{plan.precio}</span>
                    {plan.periodo && <span className="tarjetaPlanPrecioPeriodo">{plan.periodo}</span>}
                </div>
                <p className="tarjetaPlanDescripcion">{plan.descripcion}</p>
            </div>

            <ul className="tarjetaPlanCaracteristicas">
                {plan.caracteristicas.map((car, idx) => (
                    <li key={idx} className={`tarjetaPlanItem ${car.incluido ? 'tarjetaPlanItemIncluido' : 'tarjetaPlanItemNoIncluido'}`}>
                        <span className="tarjetaPlanItemIcono">
                            {car.incluido ? (
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
                                    <polyline points="20 6 9 17 4 12" />
                                </svg>
                            ) : (
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                                    <line x1="18" y1="6" x2="6" y2="18" />
                                    <line x1="6" y1="6" x2="18" y2="18" />
                                </svg>
                            )}
                        </span>
                        <span className="tarjetaPlanItemTexto">{car.texto}</span>
                    </li>
                ))}
            </ul>

            <div className="tarjetaPlanAccion">
                <Button
                    variante={plan.destacado ? 'primario' : 'outline'}
                    tamano="mediano"
                    onClick={handleClickCTA}
                >
                    {plan.ctaTexto}
                </Button>
                <Button variante="texto" className="tarjetaPlanConversar" onClick={handleConversar}>
                    {t('plans.chat_with_us', 'Conversar')}
                </Button>
            </div>
        </div>
    );
};

export const SeccionPlanesServicio: React.FC<SeccionPlanesServicioProps> = ({slug}) => {
    const {t} = useTranslation();
    const datos = obtenerPlanesServicio(slug);
    const [planSeleccionado, setPlanSeleccionado] = useState<PlanServicio | null>(null);

    if (!datos) return null;

    return (
        <section id="planesServicio" className="planesSeccion">
            <div className="planesContenedor">
                <div className="planesCabecera">
                    <h2 className="planesTitulo">{t('plans.title')}</h2>
                    <p className="planesSubtitulo">{t('plans.subtitle')}</p>
                </div>
                <div className="planesGrid">
                    {datos.planes.map(plan => (
                        <TarjetaPlan key={plan.id} plan={plan} onSeleccionar={setPlanSeleccionado} />
                    ))}
                </div>
            </div>

            {planSeleccionado && (
                <ModalCompra
                    plan={planSeleccionado}
                    servicioSlug={datos.servicioSlug}
                    abierto
                    onCerrar={() => setPlanSeleccionado(null)}
                />
            )}
        </section>
    );
};

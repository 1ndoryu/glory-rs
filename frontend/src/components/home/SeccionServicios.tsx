/**
 * Componente: SeccionServicios
 * Muestra una lista de servicios ofrecidos con imágenes y enlaces.
 * Diseño minimalista con tipografía grande.
 */
import React from 'react';
import {useTranslation} from 'react-i18next';
import {SeccionHeader} from '../ui/SeccionHeader';
import {ServiceCard} from '../ui/ServiceCard';
import {SERVICIOS_PRINCIPALES} from '../../data/servicios';
import './SeccionServicios.css';

export const SeccionServicios: React.FC = () => {
    const {t} = useTranslation();

    return (
        <section className="seccionServicios" id="servicios">
            <div className="serviciosContenedor">
                <SeccionHeader titulo={t('sections.services')} />
                <div className="serviciosLista">
                    {SERVICIOS_PRINCIPALES.map(servicio => (
                        <ServiceCard key={servicio.id} servicio={servicio} variant="simple" />
                    ))}
                </div>
            </div>
        </section>
    );
};

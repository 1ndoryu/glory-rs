/**
 * Componente: GridServicios
 * Grid de 3 columnas para mostrar las tarjetas de servicios.
 */
import React from 'react';
import {useTranslation} from 'react-i18next';
import {Servicio} from '../../types/servicios';
import {ServiceCard} from '../ui/ServiceCard';
import './GridServicios.css';

interface GridServiciosProps {
    servicios: Servicio[];
}

export const GridServicios: React.FC<GridServiciosProps> = ({servicios}) => {
    const {t} = useTranslation();

    if (servicios.length === 0) {
        return (
            <div className="gridVacio">
                <p className="gridVacioTexto">{t('services_page.empty')}</p>
            </div>
        );
    }

    return (
        <div className="gridServicios">
            {servicios.map(servicio => (
                <ServiceCard key={servicio.id} servicio={servicio} variant="detailed" />
            ))}
        </div>
    );
};

/**
 * Componente: SeccionServiciosRelacionados
 * Descripcion: Muestra 3 servicios relacionados excluyendo el actual.
 */
import React, {useMemo} from 'react';
import {useTranslation} from 'react-i18next';
import {SeccionHeader} from '../ui/SeccionHeader';
import {ServiceCard} from '../ui/ServiceCard';
import {obtenerServiciosRelacionados} from '../../data/servicios';
import './SeccionServiciosRelacionados.css';

interface SeccionServiciosRelacionadosProps {
    servicioActualId?: string;
}

export const SeccionServiciosRelacionados: React.FC<SeccionServiciosRelacionadosProps> = ({servicioActualId}) => {
    const {t} = useTranslation();
    const servicios = useMemo(
        () => obtenerServiciosRelacionados(servicioActualId, 3),
        [servicioActualId]
    );

    if (servicios.length === 0) return null;

    return (
        <section className="seccionServiciosRelacionados">
            <div className="relacionadosContenedor">
                <SeccionHeader titulo={t('sections.more_services')} />

                <div className="relacionadosLista">
                    {servicios.map(servicio => (
                        <ServiceCard key={servicio.id} servicio={servicio} variant="simple" />
                    ))}
                </div>
            </div>
        </section>
    );
};

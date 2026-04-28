import type { ReactNode } from 'react';
import { AlertTriangle, Link, Music, Music2, Upload } from 'lucide-react';

interface AutomationBatchDetailsProps {
    lote: {
        tipo: string;
        recortes: number;
        samples_publicados: number;
        canciones_nuevas: number;
        sampleos_nuevos: number;
        error_mensaje: string | null;
    };
}

const DetailItem = ({ children }: { children: ReactNode }): JSX.Element => (
    <span className="detalleLoteItem">{children}</span>
);

export const AutomationBatchDetails = ({ lote }: AutomationBatchDetailsProps): JSX.Element => {
    if (lote.tipo === 'extraccion') {
        return (
            <span className="detalleLoteTexto">
                {lote.recortes > 0 && <DetailItem><Music size={12} className="iconoDetalleLote" /> {lote.recortes} recortes</DetailItem>}
                {lote.samples_publicados > 0 && <DetailItem><Upload size={12} className="iconoDetalleLote" /> {lote.samples_publicados} pub.</DetailItem>}
                {lote.error_mensaje && <DetailItem><AlertTriangle size={12} className="iconoDetalleLote" /> err</DetailItem>}
            </span>
        );
    }

    return (
        <span className="detalleLoteTexto">
            {lote.canciones_nuevas > 0 && <DetailItem><Music2 size={12} className="iconoDetalleLote" /> {lote.canciones_nuevas} canciones</DetailItem>}
            {lote.sampleos_nuevos > 0 && <DetailItem><Link size={12} className="iconoDetalleLote" /> {lote.sampleos_nuevos} sampleos</DetailItem>}
            {lote.error_mensaje && <DetailItem><AlertTriangle size={12} className="iconoDetalleLote" /> err</DetailItem>}
        </span>
    );
};
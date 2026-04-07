/* [044A-38 Fase 6] Panel de entregables dentro de cada fase.
 * [074A-51] Simplificado: sin upload de archivos. Entrega solo marca la fase.
 * Los archivos se comparten por chat. Botón primario. */
import {Download, FileText, Package} from 'lucide-react';
import {formatFileSize} from '../../api/deliverables';
import {useEntregablesPanel} from '../../hooks/useEntregablesPanel';
import {Button} from '../ui/Button';
import './EntregablesPanel.css';

interface EntregablesPanelProps {
    orderId: string;
    phaseNumber: number;
    canDeliver: boolean;
}

export function EntregablesPanel({orderId, phaseNumber, canDeliver}: EntregablesPanelProps) {
    const {
        error, cargando, entregando,
        handleDeliver, handleDownload, sortedRevisions, revisionGroups,
    } = useEntregablesPanel(orderId, phaseNumber);

    return (
        <div className="entregablesPanel">
            {/* [074A-51] Botón de entrega sin archivos — solo marca la fase como entregada */}
            {canDeliver && (
                <div className="entregablesEntregarZona">
                    {error && <p className="entregablesError">{error}</p>}
                    <Button
                        variante="primario"
                        tamano="pequeno"
                        className="faseBtn"
                        onClick={handleDeliver}
                        disabled={entregando}
                        type="button"
                    >
                        <Package size={14} /> {entregando ? 'Entregando...' : 'Entregar'}
                    </Button>
                </div>
            )}

            {/* Historial de entregables agrupado por revisión */}
            {cargando && <p className="entregablesCargando">Cargando entregables...</p>}
            {!cargando && sortedRevisions.length > 0 && (
                <div className="entregablesHistory">
                    {sortedRevisions.map(rev => (
                        <div key={rev} className="entregablesRevision">
                            <span className="entregablesRevisionLabel">Revisión {rev}</span>
                            <ul className="entregablesFileList">
                                {revisionGroups[rev].map(d => (
                                    <li key={d.id} className="entregablesFileItem">
                                        <FileText size={12} />
                                        <span className="entregablesFileName">{d.file_name}</span>
                                        <span className="entregablesFileSize">{formatFileSize(d.file_size_bytes)}</span>
                                        <Button
                                            variante="texto"
                                            className="entregablesDownloadBtn"
                                            onClick={() => handleDownload(d.id, d.file_name)}
                                            type="button"
                                        >
                                            <Download size={12} />
                                        </Button>
                                    </li>
                                ))}
                            </ul>
                            {revisionGroups[rev][0].notes && (
                                <p className="entregablesRevisionNotes">{revisionGroups[rev][0].notes}</p>
                            )}
                        </div>
                    ))}
                </div>
            )}
        </div>
    );
}

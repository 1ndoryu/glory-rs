/* [044A-38 Fase 6] Panel de entregables dentro de cada fase.
 * Maneja upload de archivos (empleado) y listado/descarga (todos los roles).
 * Lógica extraída a useEntregablesPanel (SRP). */
import {Upload, Download, FileText, Package, X} from 'lucide-react';
import {formatFileSize} from '../../api/deliverables';
import {useEntregablesPanel} from '../../hooks/useEntregablesPanel';
import './EntregablesPanel.css';

interface EntregablesPanelProps {
    orderId: string;
    phaseNumber: number;
    canDeliver: boolean;
}

export function EntregablesPanel({orderId, phaseNumber, canDeliver}: EntregablesPanelProps) {
    const {
        files, notes, setNotes, error, cargando, entregando,
        fileInputRef, handleFileSelect, removeFile, handleDeliver,
        handleDownload, sortedRevisions, revisionGroups,
    } = useEntregablesPanel(orderId, phaseNumber);

    return (
        <div className="entregablesPanel">
            {/* Zona de upload para empleados */}
            {canDeliver && (
                <div className="entregablesUpload">
                    <div className="entregablesUploadHeader">
                        <Upload size={14} />
                        <span>Entregar archivos</span>
                    </div>
                    <input
                        ref={fileInputRef}
                        type="file"
                        multiple
                        onChange={handleFileSelect}
                        className="entregablesFileInput"
                    />
                    {files.length > 0 && (
                        <ul className="entregablesFileList">
                            {files.map((f, i) => (
                                <li key={`${f.name}-${i}`} className="entregablesFileItem">
                                    <FileText size={12} />
                                    <span className="entregablesFileName">{f.name}</span>
                                    <span className="entregablesFileSize">{formatFileSize(f.size)}</span>
                                    <button className="entregablesFileRemove" onClick={() => removeFile(i)} type="button">
                                        <X size={12} />
                                    </button>
                                </li>
                            ))}
                        </ul>
                    )}
                    <textarea
                        className="entregablesNotes"
                        placeholder="Notas opcionales sobre la entrega..."
                        value={notes}
                        onChange={e => setNotes(e.target.value)}
                        rows={2}
                    />
                    {error && <p className="entregablesError">{error}</p>}
                    <button
                        className="faseBtn faseBtnEntregar"
                        onClick={handleDeliver}
                        disabled={entregando || files.length === 0}
                        type="button"
                    >
                        <Package size={14} /> {entregando ? 'Entregando...' : 'Entregar'}
                    </button>
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
                                        <button
                                            className="entregablesDownloadBtn"
                                            onClick={() => handleDownload(d.id, d.file_name)}
                                            type="button"
                                        >
                                            <Download size={12} />
                                        </button>
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

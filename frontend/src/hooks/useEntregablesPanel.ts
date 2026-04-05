/* [044A-38 Fase 6] Hook para el panel de entregables.
 * Encapsula estado de upload, validación de archivos, entrega y descarga.
 * Extraído de EntregablesPanel.tsx para respetar SRP (max 3 useState en componente). */

import { useRef, useState, useCallback } from 'react';
import { useDeliverables } from './useDeliverables';
import type { PhaseDeliverable } from '../api/deliverables';

const MAX_FILES = 5;
const MAX_SIZE = 10 * 1024 * 1024;

export function useEntregablesPanel(orderId: string, phaseNumber: number) {
    const {deliverables, cargando, entregarFase, entregando, descargar} = useDeliverables(orderId, phaseNumber);
    const [files, setFiles] = useState<File[]>([]);
    const [notes, setNotes] = useState('');
    const [error, setError] = useState('');
    const fileInputRef = useRef<HTMLInputElement>(null);

    const handleFileSelect = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
        if (!e.target.files) return;
        const selected = Array.from(e.target.files);

        if (files.length + selected.length > MAX_FILES) {
            setError(`Máximo ${MAX_FILES} archivos por entrega`);
            return;
        }
        const oversized = selected.find(f => f.size > MAX_SIZE);
        if (oversized) {
            setError(`"${oversized.name}" excede 10 MB`);
            return;
        }
        setError('');
        setFiles(prev => [...prev, ...selected]);
    }, [files.length]);

    const removeFile = useCallback((idx: number) => {
        setFiles(prev => prev.filter((_, i) => i !== idx));
    }, []);

    const handleDeliver = useCallback(async () => {
        if (files.length === 0) {
            setError('Selecciona al menos un archivo');
            return;
        }
        try {
            setError('');
            await entregarFase({files, notes: notes || undefined});
            setFiles([]);
            setNotes('');
            if (fileInputRef.current) fileInputRef.current.value = '';
        } catch {
            setError('Error al entregar. Intenta de nuevo.');
        }
    }, [files, notes, entregarFase]);

    const handleDownload = useCallback(async (deliverableId: string, fileName: string) => {
        try {
            const blobUrl = await descargar(deliverableId);
            const a = document.createElement('a');
            a.href = blobUrl;
            a.download = fileName;
            a.click();
            URL.revokeObjectURL(blobUrl);
        } catch {
            setError('Error al descargar archivo');
        }
    }, [descargar]);

    /* Agrupar entregables por revisión (descendente) */
    const revisionGroups = deliverables.reduce<Record<number, PhaseDeliverable[]>>((acc, d) => {
        if (!acc[d.revision_number]) acc[d.revision_number] = [];
        acc[d.revision_number].push(d);
        return acc;
    }, {});
    const sortedRevisions = Object.keys(revisionGroups).map(Number).sort((a, b) => b - a);

    return {
        files,
        notes,
        setNotes,
        error,
        cargando,
        entregando,
        fileInputRef,
        handleFileSelect,
        removeFile,
        handleDeliver,
        handleDownload,
        sortedRevisions,
        revisionGroups,
    };
}

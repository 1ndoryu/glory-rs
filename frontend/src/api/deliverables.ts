/* [044A-38 Fase 6] API de entregables: tipos, funciones REST y constantes.
 * Upload multipart, listado y descarga de archivos por fase. */

import axiosInstance from './axios-instance';

/* ============================================================
   TIPOS
   ============================================================ */

export interface PhaseDeliverable {
    id: string;
    phase_id: string;
    uploaded_by: string;
    file_name: string;
    file_url: string;
    file_size_bytes: number | null;
    mime_type: string | null;
    revision_number: number;
    notes: string | null;
    created_at: string;
}

export interface DeliverPhaseResponse {
    phase_id: string;
    revision_number: number;
    deliverables: PhaseDeliverable[];
}

export interface PhaseDeliverablesResponse {
    deliverables: PhaseDeliverable[];
    approval_status: string;
    revisions_used: number;
    max_revisions: number;
}

/* ============================================================
   REST
   ============================================================ */

/** Entregar fase con archivos (multipart/form-data) */
export async function apiDeliverPhase(
    orderId: string,
    phaseNumber: number,
    files: File[],
    notes?: string,
): Promise<DeliverPhaseResponse> {
    const form = new FormData();
    if (notes) form.append('notes', notes);
    for (const file of files) {
        form.append('files', file);
    }
    const {data} = await axiosInstance.post<DeliverPhaseResponse>(
        `/api/orders/${orderId}/phases/${phaseNumber}/deliver`,
        form,
    );
    return data;
}

/** Listar entregables de una fase */
export async function apiListDeliverables(
    orderId: string,
    phaseNumber: number,
): Promise<PhaseDeliverablesResponse> {
    const {data} = await axiosInstance.get<PhaseDeliverablesResponse>(
        `/api/orders/${orderId}/phases/${phaseNumber}/deliverables`,
    );
    return data;
}

/** Descargar un entregable (devuelve blob URL para <a> download) */
export async function apiDownloadDeliverable(deliverableId: string): Promise<string> {
    const {data} = await axiosInstance.get(
        `/api/deliverables/${deliverableId}/download`,
        {responseType: 'blob'},
    );
    return URL.createObjectURL(data as Blob);
}

/* ============================================================
   CONSTANTES UI
   ============================================================ */

export const PHASE_STATUS_LABELS: Record<string, string> = {
    Locked: 'Bloqueada',
    PendingPayment: 'Pago pendiente',
    Paid: 'Pagada',
    InProgress: 'En progreso',
    Delivered: 'Entregada',
    RevisionRequested: 'Revisión solicitada',
    Approved: 'Aprobada',
    Skipped: 'Omitida',
};

export const PHASE_STATUS_COLORS: Record<string, string> = {
    Locked: '#94a3b8',
    PendingPayment: '#f59e0b',
    Paid: '#3b82f6',
    InProgress: '#8b5cf6',
    Delivered: '#f97316',
    RevisionRequested: '#ef4444',
    Approved: '#22c55e',
    Skipped: '#6b7280',
};

/** Formatear tamaño de archivo legible */
export function formatFileSize(bytes: number | null): string {
    if (!bytes) return '—';
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
}

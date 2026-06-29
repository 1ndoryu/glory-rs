/*
 * Normalizer: messageNormalizer — Kamples
 * Convierte ConversationSummary / ConversationMessage (snake_case del backend Rust)
 * a Conversacion / Mensaje (camelCase legacy).
 * Patrón idéntico a commentNormalizer: acepta ambos formatos para robustez.
 */

import type { Conversacion, Mensaje, TipoMensaje, MediaMetadata } from '../../types';
import type { UsuarioResumen } from '../../types';
import { normalizarUsuarioResumen } from './sampleNormalizer';

type RawRecord = Record<string, unknown>;

const esObjeto = (valor: unknown): valor is RawRecord =>
    valor !== null && typeof valor === 'object' && !Array.isArray(valor);

/* Normalizar ConversationSummary (snake_case) → Conversacion (camelCase) */
export const normalizarConversacion = (raw: unknown): Conversacion => {
    const r = esObjeto(raw) ? raw : {};

    const participanteRaw = esObjeto(r.participante) ? r.participante : undefined;
    const participante: UsuarioResumen = normalizarUsuarioResumen(participanteRaw);

    return {
        id: Number(r.id ?? 0),
        participante,
        ultimoMensaje: String(r.ultimo_mensaje ?? r.ultimoMensaje ?? ''),
        ultimoMensajeTipo: (r.ultimo_mensaje_tipo ?? r.ultimoMensajeTipo ?? 'texto') as TipoMensaje,
        ultimoMensajeAt: String(r.ultimo_mensaje_at ?? r.ultimoMensajeAt ?? ''),
        noLeidos: Number(r.no_leidos ?? r.noLeidos ?? 0),
        esMutuo: Boolean(r.es_mutuo ?? r.esMutuo),
        aceptada: Boolean(r.aceptada),
        enLinea: Boolean(r.en_linea ?? r.enLinea),
    };
};

/* Normalizar lista de conversaciones */
export const normalizarListaConversaciones = (valor: unknown): Conversacion[] =>
    Array.isArray(valor) ? valor.map(normalizarConversacion) : [];

/* Normalizar MediaMetadata desde JSONB del backend */
const normalizarMediaMetadata = (raw: unknown): MediaMetadata | null => {
    if (!esObjeto(raw)) return null;
    /* El backend guarda metadatos en snake_case dentro del JSONB */
    return {
        formato: raw.formato as string | undefined,
        tamano: typeof raw.tamano === 'number' ? raw.tamano : undefined,
        mimeType: (raw.mime_type ?? raw.mimeType) as string | undefined,
        sampleId: (raw.sample_id ?? raw.sampleId) as number | undefined,
        titulo: raw.titulo as string | undefined,
        idCorto: (raw.id_corto ?? raw.idCorto) as string | undefined,
        slug: raw.slug as string | undefined,
        tipo: raw.tipo as string | undefined,
        bpm: typeof raw.bpm === 'number' ? raw.bpm : null,
        key: (raw.key ?? raw.music_key ?? null) as string | null,
        picos: Array.isArray(raw.picos) ? raw.picos as number[] : undefined,
        waveformUrl: (raw.waveform_url ?? raw.waveformUrl) as string | undefined,
    } as MediaMetadata;
};

/* Normalizar ConversationMessage (snake_case) → Mensaje (camelCase) */
export const normalizarMensaje = (raw: unknown): Mensaje => {
    const r = esObjeto(raw) ? raw : {};

    return {
        id: Number(r.id ?? 0),
        conversacionId: Number(r.conversacion_id ?? r.conversacionId ?? 0),
        remitenteId: Number(r.remitente_id ?? r.remitenteId ?? 0),
        contenido: String(r.contenido ?? ''),
        tipo: (r.tipo ?? 'texto') as TipoMensaje,
        mediaUrl: (r.media_url ?? r.mediaUrl ?? null) as string | null,
        mediaMetadata: normalizarMediaMetadata(r.media_metadata ?? r.mediaMetadata),
        leido: Boolean(r.leido),
        creadoAt: String(r.created_at ?? r.creadoAt ?? ''),
    };
};

/* Normalizar lista de mensajes */
export const normalizarListaMensajes = (valor: unknown): Mensaje[] =>
    Array.isArray(valor) ? valor.map(normalizarMensaje) : [];

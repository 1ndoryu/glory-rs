/*
 * Normalizer: commentNormalizer — Kamples
 * Convierte CommentDetail (snake_case del backend Rust) a Comentario (camelCase legacy).
 * Patrón idéntico a postNormalizer: acepta ambos formatos para robustez.
 */

import type { Comentario, UsuarioResumen } from '../../types';
import { normalizarUsuarioResumen } from './sampleNormalizer';

type RawRecord = Record<string, unknown>;

const esObjeto = (valor: unknown): valor is RawRecord =>
    valor !== null && typeof valor === 'object' && !Array.isArray(valor);

/* C-comment-fix: normalizar un ComentarioDetail individual → Comentario legacy */
export const normalizarComentario = (raw: unknown): Comentario => {
    const r = esObjeto(raw) ? raw : {};

    /* autor: viene como CommentAuthorSummary (snake_case) o UsuarioResumen (camelCase) */
    const autorRaw = esObjeto(r.autor) ? r.autor : undefined;
    const autor: UsuarioResumen = normalizarUsuarioResumen(autorRaw);

    const mediaMetadataRaw = esObjeto(r.media_metadata) ? r.media_metadata
        : esObjeto(r.mediaMetadata) ? r.mediaMetadata
            : null;

    return {
        id: Number(r.id ?? 0),
        autorId: Number(r.autor_id ?? r.autorId ?? 0),
        contenido: String(r.contenido ?? ''),
        creadoAt: String(r.created_at ?? r.creadoAt ?? ''),
        editadoAt: (r.updated_at ?? r.editadoAt ?? null) as string | null,
        autor,
        tipoContenido: (r.tipo_contenido ?? r.tipoContenido ?? 'texto') as Comentario['tipoContenido'],
        mediaUrl: (r.media_url ?? r.mediaUrl ?? null) as string | null,
        mediaMetadata: mediaMetadataRaw ? {
            formato: mediaMetadataRaw.formato as string | undefined,
            tamano: typeof mediaMetadataRaw.tamano === 'number' ? mediaMetadataRaw.tamano : undefined,
            mimeType: mediaMetadataRaw.mimeType as string | undefined,
            picos: Array.isArray(mediaMetadataRaw.picos) ? mediaMetadataRaw.picos as number[] : undefined,
            waveformUrl: mediaMetadataRaw.waveformUrl as string | undefined,
        } : null,
        parentId: (r.parent_id ?? r.parentId ?? null) as number | null,
        totalLikes: Number(r.total_likes ?? r.totalLikes ?? 0),
        totalRespuestas: Number(r.total_respuestas ?? r.totalRespuestas ?? 0),
        liked: Boolean(r.liked),
        respuestas: Array.isArray(r.respuestas)
            ? r.respuestas.map((sub: unknown) => normalizarComentario(sub))
            : undefined,
    };
};

export const normalizarListaComentarios = (valor: unknown): Comentario[] =>
    Array.isArray(valor) ? valor.map(normalizarComentario) : [];

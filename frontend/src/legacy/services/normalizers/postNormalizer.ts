import type { ComentarioDestacado, Publicacion, RepostOriginal } from '../../types';
import { normalizarListaSamples, normalizarUsuarioResumen } from './sampleNormalizer';

type RawRecord = Record<string, unknown>;

const esObjeto = (valor: unknown): valor is RawRecord =>
    valor !== null && typeof valor === 'object' && !Array.isArray(valor);

const normalizarComentarioDestacado = (valor: unknown): ComentarioDestacado | null => {
    if (!esObjeto(valor)) return null;

    const autor = normalizarUsuarioResumen(esObjeto(valor.autor) ? valor.autor : undefined);

    return {
        id: Number(valor.id ?? 0),
        autorId: Number(valor.autor_id ?? valor.autorId ?? 0),
        contenido: String(valor.contenido ?? ''),
        totalLikes: Number(valor.total_likes ?? valor.totalLikes ?? 0),
        creadoAt: String(valor.created_at ?? valor.creadoAt ?? ''),
        tipoContenido: (valor.tipo_contenido ?? valor.tipoContenido ?? 'texto') as ComentarioDestacado['tipoContenido'],
        mediaUrl: (valor.media_url ?? valor.mediaUrl ?? null) as string | null,
        autor: {
            id: autor.id,
            username: autor.username,
            nombreVisible: autor.nombreVisible,
            avatarUrl: autor.avatarUrl ?? undefined,
        },
    };
};

const normalizarRepostOriginal = (valor: unknown): RepostOriginal | null => {
    if (!esObjeto(valor)) return null;

    return {
        id: Number(valor.id ?? 0),
        contenido: String(valor.contenido ?? ''),
        imagenes: Array.isArray(valor.imagenes) ? valor.imagenes.map((img) => String(img)) : [],
        autor: normalizarUsuarioResumen(esObjeto(valor.autor) ? valor.autor : undefined),
        samplesAdjuntos: normalizarListaSamples(valor.samplesAdjuntos ?? valor.samples_adjuntos),
    };
};

/* [274A-3] Adaptador PostDetail Rust -> Publicacion legacy.
 * El backend nuevo habla snake_case y puede devolver ids en `samples_adjuntos`;
 * el frontend legacy renderiza camelCase y necesita arrays siempre definidos. */
export const normalizarPublicacion = (valor: unknown): Publicacion => {
    const raw = esObjeto(valor) ? valor : {};
    const reaccion = (raw.reaccion ?? raw.mi_reaccion ?? null) as Publicacion['reaccion'];

    return {
        id: Number(raw.id ?? 0),
        autorId: Number(raw.autorId ?? raw.autor_id ?? 0),
        tipo: String(raw.tipo ?? 'social') as Publicacion['tipo'],
        contenido: String(raw.contenido ?? ''),
        imagenes: Array.isArray(raw.imagenes) ? raw.imagenes.map((img) => String(img)) : [],
        samplesAdjuntos: normalizarListaSamples(raw.samplesAdjuntos ?? raw.samples_adjuntos),
        totalLikes: Number(raw.totalLikes ?? raw.total_likes ?? 0),
        totalComentarios: Number(raw.totalComentarios ?? raw.total_comentarios ?? 0),
        totalReposts: Number(raw.totalReposts ?? raw.total_reposts ?? 0),
        liked: Boolean(raw.liked ?? (reaccion === 'like' || reaccion === 'encanta')),
        reaccion,
        reposteado: Boolean(raw.reposteado ?? raw.yo_ya_repostee),
        creadoAt: String(raw.creadoAt ?? raw.created_at ?? ''),
        moderacionEstado: (raw.moderacionEstado ?? raw.moderacion_estado ?? null) as Publicacion['moderacionEstado'],
        autor: normalizarUsuarioResumen(esObjeto(raw.autor) ? raw.autor : undefined),
        repostOriginal: normalizarRepostOriginal(raw.repostOriginal ?? raw.repost_original),
        comentarioDestacado: normalizarComentarioDestacado(raw.comentarioDestacado ?? raw.comentario_destacado),
    };
};

export const normalizarListaPublicaciones = (valor: unknown): Publicacion[] =>
    Array.isArray(valor) ? valor.map(normalizarPublicacion) : [];
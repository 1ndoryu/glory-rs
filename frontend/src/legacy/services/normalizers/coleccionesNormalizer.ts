import type { Coleccion, ColeccionResumen, SampleResumen, UsuarioResumen } from '../../types';

const normalizarUsuarioResumen = (
    raw: Record<string, unknown> | null | undefined,
    fallbackUsername = ''
): UsuarioResumen => {
    const username = (raw?.username ?? raw?.creator_username ?? fallbackUsername ?? '') as string;
    const nombreVisible = (raw?.nombre_visible ?? raw?.nombreVisible ?? username) as string;

    return {
        id: (raw?.id ?? raw?.usuario_id ?? raw?.usuarioId ?? raw?.creator_id ?? 0) as number,
        username,
        nombreVisible,
        avatarUrl: (raw?.avatar_url ?? raw?.avatarUrl ?? raw?.creator_avatar_url ?? null) as string | null,
        verificado: (raw?.verificado ?? raw?.creator_verificado ?? false) as boolean,
    };
};

const extraerCreadorRaw = (raw: Record<string, unknown>): Record<string, unknown> | undefined => {
    if (raw.creador && typeof raw.creador === 'object') {
        return raw.creador as Record<string, unknown>;
    }
    if (raw.creator && typeof raw.creator === 'object') {
        return raw.creator as Record<string, unknown>;
    }
    if (raw.usuario && typeof raw.usuario === 'object') {
        return raw.usuario as Record<string, unknown>;
    }
    if (
        raw.username !== undefined
        || raw.nombre_visible !== undefined
        || raw.nombreVisible !== undefined
        || raw.creator_username !== undefined
    ) {
        return raw;
    }
    return undefined;
};

const normalizarSampleResumen = (raw: Record<string, unknown>): SampleResumen => {
    const creadorRaw = extraerCreadorRaw(raw);
    const fallbackUsername = (raw.creator_username ?? raw.username ?? 'autor') as string;

    return {
        id: (raw.id ?? 0) as number,
        titulo: (raw.titulo ?? '') as string,
        slug: (raw.slug ?? String(raw.id ?? '')) as string,
        descripcion: (raw.descripcion ?? undefined) as string | undefined,
        bpm: (raw.bpm ?? null) as number | null,
        key: (raw.key ?? raw.music_key ?? null) as SampleResumen['key'],
        escala: (raw.escala ?? null) as SampleResumen['escala'],
        duracion: Number(raw.duracion ?? 0),
        formato: (raw.formato ?? undefined) as string | undefined,
        tags: Array.isArray(raw.tags) ? raw.tags as string[] : [],
        tipo: (raw.tipo ?? 'loop') as SampleResumen['tipo'],
        esPremium: (raw.es_premium ?? raw.esPremium ?? false) as boolean,
        precio: (raw.precio ?? null) as number | null,
        rutaPreview: (raw.rutaPreview ?? raw.ruta_preview ?? '') as string,
        rutaWaveform: (raw.rutaWaveform ?? raw.ruta_waveform ?? '') as string,
        imagenUrl: (raw.imagenUrl ?? raw.imagen_url ?? null) as string | null,
        totalDescargas: Number(raw.totalDescargas ?? raw.total_descargas ?? 0),
        totalLikes: Number(raw.totalLikes ?? raw.total_likes ?? 0),
        totalReproducciones: Number(raw.totalReproducciones ?? raw.total_reproducciones ?? 0),
        metadata: (raw.metadata ?? null) as SampleResumen['metadata'],
        audioHash: (raw.audioHash ?? raw.audio_hash ?? null) as string | null | undefined,
        creador: normalizarUsuarioResumen(creadorRaw, fallbackUsername),
        liked: raw.liked as boolean | undefined,
        reaccion: (raw.reaccion ?? null) as SampleResumen['reaccion'],
        verificado: (raw.verificado ?? undefined) as boolean | undefined,
        mostrarEnComunidad: (raw.mostrarEnComunidad ?? raw.mostrar_en_comunidad) as boolean | undefined,
        yaColeccionado: (raw.yaColeccionado ?? raw.ya_coleccionado) as boolean | undefined,
        yaGuardadoEnColeccion: (raw.yaGuardadoEnColeccion ?? raw.ya_guardado_en_coleccion) as boolean | undefined,
        yaComentado: (raw.yaComentado ?? raw.ya_comentado) as boolean | undefined,
        esMio: (raw.esMio ?? raw.es_mio) as boolean | undefined,
        yaComprado: (raw.yaComprado ?? raw.ya_comprado) as boolean | undefined,
        cancionOrigenId: (raw.cancionOrigenId ?? raw.cancion_origen_id ?? null) as number | null | undefined,
        relacionSampleoId: (raw.relacionSampleoId ?? raw.relacion_sampleo_id ?? null) as number | null | undefined,
        coleccionOriginal: (raw.coleccionOriginal ?? raw.coleccion_original ?? null) as SampleResumen['coleccionOriginal'],
        extraccion: (raw.extraccion ?? null) as SampleResumen['extraccion'],
        scoreDebug: (raw.scoreDebug ?? raw.score_debug ?? null) as SampleResumen['scoreDebug'],
    };
};

const normalizarColeccionPadre = (raw: Record<string, unknown>) => ({
    id: (raw.id ?? 0) as number,
    nombre: (raw.nombre ?? '') as string,
    slug: (raw.slug ?? null) as string | null,
});

export const normalizarColeccionResumen = (raw: Record<string, unknown>): ColeccionResumen => ({
    id: (raw.id ?? 0) as number,
    nombre: (raw.nombre ?? '') as string,
    slug: (raw.slug ?? null) as string | null,
    imagenUrl: (raw.imagen_url ?? raw.imagenUrl ?? null) as string | null,
    totalSamples: (raw.total_items ?? raw.total_samples ?? raw.totalSamples ?? 0) as number,
    esPublica: (raw.publica ?? raw.esPublica ?? true) as boolean,
    parentId: (raw.parent_id ?? raw.parentId ?? null) as number | null,
    tags: Array.isArray(raw.tags) ? raw.tags as string[] : [],
});

export const normalizarColeccion = (raw: Record<string, unknown>): Coleccion => {
    const samples = Array.isArray(raw.samples)
        ? (raw.samples as Record<string, unknown>[]).map(normalizarSampleResumen)
        : undefined;
    const totalReal = (raw.total_items ?? raw.total_samples ?? raw.totalSamples ?? null) as number | null;
    const totalSamples = totalReal != null ? totalReal
        : (Array.isArray(samples) ? samples.length : 0);

    return {
        id: (raw.id ?? 0) as number,
        usuarioId: (raw.usuario_id ?? raw.usuarioId ?? 0) as number,
        nombre: (raw.nombre ?? '') as string,
        slug: (raw.slug ?? null) as string | null,
        descripcion: (raw.descripcion ?? '') as string,
        esPublica: (raw.publica ?? raw.esPublica ?? true) as boolean,
        imagenUrl: (raw.imagen_url ?? raw.imagenUrl ?? null) as string | null,
        totalSamples,
        creadoAt: (raw.created_at ?? raw.creadoAt ?? '') as string,
        actualizadoAt: (raw.updated_at ?? raw.actualizadoAt ?? '') as string,
        parentId: (raw.parent_id ?? raw.parentId ?? null) as number | null,
        tags: Array.isArray(raw.tags) ? raw.tags as string[] : [],
        usuario: raw.username || (raw.usuario && typeof raw.usuario === 'object')
            ? normalizarUsuarioResumen(
                (raw.usuario && typeof raw.usuario === 'object')
                    ? raw.usuario as Record<string, unknown>
                    : raw,
                (raw.username ?? '') as string,
            )
            : undefined,
        samples,
        subcolecciones: Array.isArray(raw.subcolecciones)
            ? (raw.subcolecciones as Record<string, unknown>[]).map(normalizarColeccionResumen)
            : undefined,
        coleccionPadre: raw.coleccionPadre && typeof raw.coleccionPadre === 'object'
            ? normalizarColeccionPadre(raw.coleccionPadre as Record<string, unknown>)
            : raw.coleccion_padre && typeof raw.coleccion_padre === 'object'
                ? normalizarColeccionPadre(raw.coleccion_padre as Record<string, unknown>)
                : null,
        contieneElSample: (raw.contieneElSample ?? raw.contiene_el_sample) as boolean | undefined,
        estaGuardada: (raw.estaGuardada ?? raw.esta_guardada) as boolean | undefined,
        estaLikeada: (raw.estaLikeada ?? raw.esta_likeada) as boolean | undefined,
        totalLikes: (raw.totalLikes ?? raw.total_likes ?? 0) as number,
    };
};

export const normalizarLista = (data: unknown[]): Coleccion[] =>
    Array.isArray(data) ? data.map(d => normalizarColeccion(d as Record<string, unknown>)) : [];
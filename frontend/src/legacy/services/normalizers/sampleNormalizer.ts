import type { Sample, SampleResumen, UsuarioResumen } from '../../types';

type RawRecord = Record<string, unknown>;

const esObjeto = (valor: unknown): valor is RawRecord =>
    valor !== null && typeof valor === 'object' && !Array.isArray(valor);

const extraerCreadorRaw = (raw: RawRecord): RawRecord | undefined => {
    if (esObjeto(raw.creador)) return raw.creador;
    if (esObjeto(raw.creator)) return raw.creator;
    if (esObjeto(raw.usuario)) return raw.usuario;
    if (
        raw.username !== undefined
        || raw.nombre_visible !== undefined
        || raw.nombreVisible !== undefined
        || raw.creator_username !== undefined
        || raw.creador_id !== undefined
        || raw.creator_id !== undefined
    ) {
        return raw;
    }
    return undefined;
};

export const normalizarUsuarioResumen = (
    raw: RawRecord | null | undefined,
    fallbackUsername = 'autor'
): UsuarioResumen => {
    const username = String(raw?.username ?? raw?.creator_username ?? fallbackUsername);
    const nombreVisible = String(raw?.nombre_visible ?? raw?.nombreVisible ?? username);

    return {
        id: Number(raw?.id ?? raw?.usuario_id ?? raw?.usuarioId ?? raw?.creador_id ?? raw?.creator_id ?? 0),
        username,
        nombreVisible,
        avatarUrl: (raw?.avatar_url ?? raw?.avatarUrl ?? raw?.creator_avatar_url ?? null) as string | null,
        verificado: Boolean(raw?.verificado ?? raw?.creator_verificado ?? false),
    };
};

export const normalizarSampleResumen = (valor: unknown): SampleResumen => {
    const raw = esObjeto(valor) ? valor : {};
    const creadorRaw = extraerCreadorRaw(raw);
    const fallbackUsername = String(raw.creator_username ?? raw.username ?? `autor-${String(raw.id ?? 0)}`);

    return {
        id: Number(raw.id ?? 0),
        titulo: String(raw.titulo ?? ''),
        slug: String(raw.slug ?? raw.id ?? ''),
        descripcion: (raw.descripcion ?? undefined) as string | undefined,
        bpm: raw.bpm == null ? null : Number(raw.bpm),
        key: (raw.key ?? raw.music_key ?? null) as SampleResumen['key'],
        escala: (raw.escala ?? null) as SampleResumen['escala'],
        duracion: Number(raw.duracion ?? 0),
        formato: (raw.formato ?? undefined) as string | undefined,
        tags: Array.isArray(raw.tags) ? raw.tags.map(tag => String(tag)) : [],
        tipo: String(raw.tipo ?? 'loop') as SampleResumen['tipo'],
        esPremium: Boolean(raw.es_premium ?? raw.esPremium ?? false),
        precio: raw.precio == null ? null : Number(raw.precio),
        rutaPreview: String(raw.rutaPreview ?? raw.ruta_preview ?? ''),
        rutaWaveform: String(raw.rutaWaveform ?? raw.ruta_waveform ?? ''),
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

export const normalizarSampleDetalle = (valor: unknown): Sample => {
    const raw = esObjeto(valor) ? valor : {};
    const resumen = normalizarSampleResumen(raw);
    const creadoAt = String(raw.creadoAt ?? raw.created_at ?? '');

    return {
        ...resumen,
        creadorId: Number(raw.creadorId ?? raw.creador_id ?? resumen.creador.id),
        descripcion: String(raw.descripcion ?? resumen.descripcion ?? ''),
        formato: String(raw.formato ?? resumen.formato ?? ''),
        tamano: Number(raw.tamano ?? 0),
        estado: String(raw.estado ?? 'activo') as Sample['estado'],
        metadata: (raw.metadata ?? resumen.metadata ?? null) as Sample['metadata'],
        rutaOriginal: (raw.rutaOriginal ?? raw.ruta_original ?? undefined) as string | undefined,
        rutaOptimizada: (raw.rutaOptimizada ?? raw.ruta_optimizada ?? undefined) as string | undefined,
        totalComentarios: Number(raw.totalComentarios ?? raw.total_comentarios ?? 0),
        publicadoAt: (raw.publicadoAt ?? raw.publicado_at ?? null) as string | null,
        creadoAt,
        actualizadoAt: String(raw.actualizadoAt ?? raw.updated_at ?? creadoAt),
        idCorto: (raw.idCorto ?? raw.id_corto ?? undefined) as string | undefined,
        permitirDescarga: (raw.permitirDescarga ?? raw.permitir_descarga ?? undefined) as boolean | undefined,
        licenciaLibre: (raw.licenciaLibre ?? raw.licencia_libre ?? undefined) as boolean | undefined,
        mostrarEnComunidad: (raw.mostrarEnComunidad ?? raw.mostrar_en_comunidad ?? resumen.mostrarEnComunidad) as boolean | undefined,
        cancionOrigenId: (raw.cancionOrigenId ?? raw.cancion_origen_id ?? resumen.cancionOrigenId ?? null) as number | null | undefined,
        relacionSampleoId: (raw.relacionSampleoId ?? raw.relacion_sampleo_id ?? resumen.relacionSampleoId ?? null) as number | null | undefined,
        verificado: Boolean(raw.verificado ?? resumen.verificado ?? false),
    };
};

export const normalizarListaSamples = (valor: unknown): SampleResumen[] =>
    Array.isArray(valor) ? valor.map(normalizarSampleResumen) : [];
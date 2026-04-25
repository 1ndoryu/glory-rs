/* sentinel-disable-file limite-lineas â€” proxy bootstrap central: path mapping + adaptadores de request/response para legacyâ†”Rust */
/*
 * [204A-1] Proxy fetch: intercepta /wp-json/kamples/v1/* y reescribe a /api/*
 * del backend Rust. Adapta formatos request/response entre legacy (camelCase WP)
 * y backend Rust (snake_case). Sustituye el stub mock anterior.
 *
 * Convenciones de mapping:
 * - Paths: /kamples/v1/X â†’ /api/X con excepciones explÃ­citas en PATH_MAP
 * - Request: body camelCase/email â†’ snake_case/identifier segÃºn endpoint
 * - Response: snake_case â†’ camelCase, user â†’ usuario, campos faltantes â†’ defaults
 *
 * Gotcha: el Bearer token solo se envÃ­a si __KAMPLES_DESKTOP__ = true (ver gloryContextShim.ts).
 * Los endpoints no implementados en Rust (ej: /admin/cola-ia) retornan 404 real
 * que apiCliente convierte en { ok: false } â€” los hooks lo manejan con guardas.
 */

const LS_KEY_TOKEN = 'kamples_auth_token';
const LS_KEY_REFRESH = 'kamples_refresh_token';

/* Paths del legacy que tienen nombre diferente en el backend Rust */
const PATH_MAP: Record<string, string> = {
    '/auth/registro': '/auth/register',
    '/me':            '/users/me',
};

function mapPath(legacyPath: string): string {
    if (PATH_MAP[legacyPath] !== undefined) return PATH_MAP[legacyPath];
    /* /me/* â†’ /users/me/* */
    if (legacyPath.startsWith('/me/')) return legacyPath.replace('/me/', '/users/me/');
    /* /perfil/{username} â†’ /users/{username} */
    if (legacyPath.startsWith('/perfil/')) return legacyPath.replace('/perfil/', '/users/');
    return legacyPath;
}

/* Convierte snake_case a camelCase solo en los keys de un objeto plano.
 * No modifica arrays ni valores primitivos. */
function snakeToCamel(key: string): string {
    return key.replace(/_([a-z])/g, (_, c: string) => c.toUpperCase());
}

function convertKeys(obj: unknown): unknown {
    if (Array.isArray(obj)) return obj.map(convertKeys);
    if (obj !== null && typeof obj === 'object') {
        const out: Record<string, unknown> = {};
        for (const [k, v] of Object.entries(obj as Record<string, unknown>)) {
            out[snakeToCamel(k)] = convertKeys(v);
        }
        return out;
    }
    return obj;
}

/* Adapta un UserResponse (Rust snake_case) a UsuarioAutenticado (legacy camelCase).
 * Rust puede devolver UserResponse (login, bÃ¡sico) o PrivateProfileResponse (/me). */
function adaptarUsuario(rustUser: Record<string, unknown>): Record<string, unknown> {
    /* Paso 1: snake_case genÃ©rico â†’ camelCase */
    const u = convertKeys(rustUser) as Record<string, unknown>;

    /* Paso 2: renombres especÃ­ficos que snakeâ†’camel no resuelve igual que el legacy */
    /* created_at â†’ createdAt (convertKeys), pero legacy usa creadoAt */
    if ('createdAt' in u) {
        u.creadoAt = u.createdAt;
        u.actualizadoAt = u.updatedAt ?? u.createdAt;
        delete u.createdAt;
        delete u.updatedAt;
    }
    /* generos_favoritos â†’ generosFavoritos (convertKeys), legacy usa generosPreferidos */
    if ('generosFavoritos' in u) {
        u.generosPreferidos = Array.isArray(u.generosFavoritos) ? u.generosFavoritos : [];
        delete u.generosFavoritos;
    }

    /* Paso 3: campos ausentes en Rust pero requeridos por UsuarioAutenticado */
    u.wpUserId      = u.wpUserId      ?? u.id ?? 0;
    u.bio           = u.bio           ?? '';
    u.avatarUrl     = u.avatarUrl     ?? null;
    u.portadaUrl    = u.portadaUrl    ?? null;
    u.ubicacion     = u.ubicacion     ?? null;
    u.sitioWeb      = u.sitioWeb      ?? null;
    u.verificado    = u.verificado    ?? false;
    u.siguiendo     = u.siguiendo     ?? false;
    u.stripeCustomerId = u.stripeCustomerId ?? null;
    u.stripeConnectId  = u.stripeConnectId  ?? null;
    u.paypalEmail      = u.paypalEmail      ?? null;
    u.generosPreferidos = u.generosPreferidos ?? [];
    u.creadoAt      = u.creadoAt      ?? '';
    u.actualizadoAt = u.actualizadoAt ?? '';
    u.totalSeguidores = u.totalSeguidores ?? 0;
    u.totalSeguidos   = u.totalSeguidos   ?? 0;
    u.totalSamples    = u.totalSamples    ?? 0;
    u.totalDescargas  = u.totalDescargas  ?? 0;
    /* LÃ­mites de descarga â€” evita NaN en la UI de crÃ©ditos */
    u.descargasHoy    = u.descargasHoy    ?? 0;
    u.limiteDescargas = u.limiteDescargas ?? 100;
    u.subidasEsteMes  = u.subidasEsteMes  ?? 0;
    u.limiteSubidas   = u.limiteSubidas   ?? 10;
    u.mensajesHoy     = u.mensajesHoy     ?? 0;
    u.limiteMensajes  = u.limiteMensajes  ?? 50;
    u.suspension      = u.suspension      ?? null;

    return u;
}

/* Adapta el body de request para endpoints con diferencia de campos */
function adaptarBody(rustPath: string, bodyText: string): string {
    if (!bodyText) return bodyText;
    try {
        const payload = JSON.parse(bodyText) as Record<string, unknown>;

        /* Login: legacy envÃ­a { email, password } â€” Rust espera { identifier, password } */
        if (rustPath === '/auth/login') {
            if (payload.email !== undefined && payload.identifier === undefined) {
                payload.identifier = payload.email;
                delete payload.email;
            }
            return JSON.stringify(payload);
        }

        /* Registro: legacy envÃ­a { nombreVisible } â€” Rust espera { nombre_visible } */
        if (rustPath === '/auth/register') {
            if (payload.nombreVisible !== undefined && payload.nombre_visible === undefined) {
                payload.nombre_visible = payload.nombreVisible;
                delete payload.nombreVisible;
            }
            return JSON.stringify(payload);
        }

        /* Perfil propio: el legacy envía camelCase y algunos campos que Rust aún no soporta.
         * Convertimos solo los campos persistibles para que PATCH /users/me guarde datos reales. */
        if (rustPath === '/users/me') {
            if (payload.nombreVisible !== undefined && payload.nombre_visible === undefined) {
                payload.nombre_visible = payload.nombreVisible;
                delete payload.nombreVisible;
            }
            if (payload.sitioWeb !== undefined && payload.sitio_web === undefined) {
                payload.sitio_web = payload.sitioWeb;
                delete payload.sitioWeb;
            }
            if (payload.avatarUrl !== undefined && payload.avatar_url === undefined) {
                payload.avatar_url = payload.avatarUrl;
                delete payload.avatarUrl;
            }
            if (payload.portadaUrl !== undefined && payload.portada_url === undefined) {
                payload.portada_url = payload.portadaUrl;
                delete payload.portadaUrl;
            }
            if (payload.generosPreferidos !== undefined && payload.generos_favoritos === undefined) {
                payload.generos_favoritos = payload.generosPreferidos;
                delete payload.generosPreferidos;
            }

            /* Campos todavía no soportados por UpdateProfileRequest: evitar que viajen como ruido. */
            delete payload.username;
            delete payload.paypalEmail;

            return JSON.stringify(payload);
        }

        if (rustPath === '/publicaciones') {
            if (payload.samplesAdjuntos !== undefined && payload.samples_adjuntos === undefined) {
                payload.samples_adjuntos = payload.samplesAdjuntos;
                delete payload.samplesAdjuntos;
            }
            return JSON.stringify(payload);
        }
    } catch { /* JSON invÃ¡lido â€” pasar tal cual */ }
    return bodyText;
}

/* Adapta la respuesta Rust para endpoints con diferencia de estructura.
 * Devuelve el objeto modificado o el original si no necesita cambios. */
function adaptarRespuesta(rustPath: string, json: unknown): unknown {
    if (json === null || json === undefined || typeof json !== 'object') return json;
    if (Array.isArray(json)) return json;
    const obj = json as Record<string, unknown>;

    /* Auth (login/register/google): Rust devuelve { token, refresh_token, user }
     * Legacy espera { token, usuario } â€” reescribir user â†’ usuario con campos adaptados */
    if (
        rustPath === '/auth/login' ||
        rustPath === '/auth/register' ||
        rustPath.startsWith('/auth/google')
    ) {
        if (obj.user && !obj.usuario) {
            return {
                ...obj,
                usuario: adaptarUsuario(obj.user as Record<string, unknown>),
                user: undefined,
            };
        }
        return obj;
    }

    /* /users/me: PrivateProfileResponse (snake_case) â†’ UsuarioAutenticado (camelCase) */
    if (rustPath === '/users/me/avatar' || rustPath === '/users/me/portada') {
        const data = obj.data && typeof obj.data === 'object' && !Array.isArray(obj.data)
            ? adaptarUsuario(obj.data as Record<string, unknown>)
            : obj.data;
        return {
            ...obj,
            data,
            avatarUrl: rustPath.endsWith('/avatar') ? (obj.url ?? (data as Record<string, unknown>)?.avatarUrl) : undefined,
            portadaUrl: rustPath.endsWith('/portada') ? (obj.url ?? (data as Record<string, unknown>)?.portadaUrl) : undefined,
        };
    }

    if (rustPath === '/users/me') {
        /* Si ya viene con 'username' y clave snake_case, adaptar */
        if ('nombre_visible' in obj || 'avatar_url' in obj || 'created_at' in obj) {
            return adaptarUsuario(obj);
        }
        return adaptarUsuario(obj);
    }

    if (rustPath === '/publicaciones' && obj.post && typeof obj.post === 'object') {
        return {
            ok: obj.ok ?? true,
            data: convertKeys(obj.post),
        };
    }

    /* /users/{username}: PublicProfileResponse â†’ Usuario (camelCase) */
    if (rustPath.startsWith('/users/') && rustPath !== '/users/me') {
        if ('nombre_visible' in obj || 'avatar_url' in obj) {
            return adaptarUsuario(obj);
        }
    }

    /* Listas de usuarios en respuestas paginadas ({ data: [], total: N })
     * que el backend admin devuelve */
    if (obj.data && Array.isArray(obj.data) && typeof obj.total === 'number') {
        /* No convertir keys en listas admin â€” esas usan snake_case tal como el tipo lo espera */
        return obj;
    }

    return json;
}

/* Guarda los tokens en localStorage tras login/registro OAuth exitoso.
 * El access token autentica las requests y el refresh token evita que
 * una sesión persistida quede rota en el siguiente arranque del SPA. */
function guardarTokensSiPresentes(json: unknown): void {
    if (json !== null && typeof json === 'object') {
        const obj = json as Record<string, unknown>;
        const token = obj.token ?? (obj as { data?: Record<string, unknown> }).data?.token;
        const refreshToken =
            obj.refresh_token
            ?? obj.refreshToken
            ?? (obj as { data?: Record<string, unknown> }).data?.refresh_token
            ?? (obj as { data?: Record<string, unknown> }).data?.refreshToken;
        if (typeof token === 'string' && token) {
            try {
                localStorage.setItem(LS_KEY_TOKEN, token);
            } catch { /* storage bloqueado */ }
        }
        if (typeof refreshToken === 'string' && refreshToken) {
            try {
                localStorage.setItem(LS_KEY_REFRESH, refreshToken);
            } catch { /* storage bloqueado */ }
        }
    }
}

const fetchOriginal = window.fetch.bind(window);

window.fetch = async (input, init) => {
    const url =
        typeof input === 'string'
            ? input
            : input instanceof Request
                ? input.url
                : String(input);

    /* Llamadas directas a /api/* (cliente Orval generado): inyectar Bearer token si existe */
    if (url.includes('/api/') && !url.includes('/wp-json/')) {
        const token = localStorage.getItem(LS_KEY_TOKEN);
        if (token) {
            const headers = new Headers((init?.headers as HeadersInit | undefined) ?? {});
            if (!headers.has('Authorization')) {
                headers.set('Authorization', `Bearer ${token}`);
            }
            return fetchOriginal(input, { ...init, headers });
        }
        return fetchOriginal(input, init);
    }

    /* Solo interceptar rutas del legacy wp-json */
    if (!url.includes('/wp-json/')) {
        return fetchOriginal(input, init);
    }

    /* Extraer el path posterior a /kamples/v1 y el query string */
    const afterV1 = url.replace(/^.*\/kamples\/v1/, '');
    const [legacyPath, query = ''] = afterV1.split('?');
    const rustPath  = mapPath(legacyPath ?? '');
    const rustUrl   = `/api${rustPath}${query ? '?' + query : ''}`;

    /* Adaptar body si es necesario */
    let bodyText = '';
    if (init?.body) {
        bodyText = typeof init.body === 'string' ? init.body : '';
    }

    const adaptedBody = adaptarBody(rustPath, bodyText);

    /* Construir headers: eliminar X-WP-Nonce (el backend Rust no lo entiende)
     * e inyectar Bearer token si hay uno guardado en localStorage */
    const origHeaders = new Headers((init?.headers as HeadersInit | undefined) ?? {});
    origHeaders.delete('X-WP-Nonce');
    const savedToken = localStorage.getItem(LS_KEY_TOKEN);
    if (savedToken && !origHeaders.has('Authorization')) {
        origHeaders.set('Authorization', `Bearer ${savedToken}`);
    }

    const newInit: RequestInit = {
        ...init,
        headers: origHeaders,
    };
    if (adaptedBody !== bodyText && adaptedBody !== '') {
        newInit.body = adaptedBody;
    }

    /* Llamada real al backend */
    let resp: Response;
    try {
        resp = await fetchOriginal(rustUrl, newInit);
    } catch (err) {
        /* Backend no disponible â€” devolver error de red */
        return Promise.reject(err);
    }

    /* Para respuestas de auth, adaptar la estructura de la respuesta */
    const needsAdaptation =
        rustPath === '/auth/login' ||
        rustPath === '/auth/register' ||
        rustPath.startsWith('/auth/google') ||
        rustPath === '/users/me' ||
        rustPath.startsWith('/users/');

    if (!needsAdaptation) {
        return resp;
    }

    /* Leer y adaptar el body */
    const text = await resp.text();
    let json: unknown;
    try {
        json = JSON.parse(text);
    } catch {
        /* Respuesta no JSON â€” devolver tal cual */
        return new Response(text, { status: resp.status, headers: resp.headers });
    }

    /* Guardar tokens si la respuesta de auth los incluye */
    if (resp.ok && (rustPath === '/auth/login' || rustPath === '/auth/register' || rustPath.startsWith('/auth/google'))) {
        guardarTokensSiPresentes(json);
    }

    const adapted = adaptarRespuesta(rustPath, json);
    return new Response(JSON.stringify(adapted), {
        status: resp.status,
        headers: resp.headers,
    });
};

export {};

/* [274A-11] Resolver compartido entre `scripts/audit-api.mjs` (CLI) y el
 * analyzer de Code Sentinel `apiEndpointAnalyzer.ts` (port en TS).
 *
 * Responsabilidad: dado un path legacy (lo que se pasa a apiGet/Post/...),
 * devolver el path real que pega el backend Rust, replicando exactamente
 * la logica de `frontend/src/bootstrap/wpJsonStub.ts::mapPath` mas el
 * matching contra los paths de `openapi.json`.
 *
 * Mantener sincronizado con wpJsonStub.ts. Si se cambian PATH_MAP,
 * PATH_MAP_KEEP, PATH_KEEP_PREFIXES o las reglas /me/*, /perfil/*, hay
 * que actualizar este archivo en el mismo commit. */

import fs from 'node:fs';
import path from 'node:path';

/* Espejo exacto de wpJsonStub.ts. */
const PATH_MAP = {
    '/auth/registro': '/auth/register',
    '/me': '/users/me',
};

const PATH_MAP_KEEP = new Set([
    '/me/bloqueados',
]);

const PATH_KEEP_PREFIXES = [
    '/me/coleccionados',
    '/me/favoritos',
    '/me/descargas/sugerencias',
];

export function mapPath(legacyPath) {
    if (PATH_MAP[legacyPath] !== undefined) return PATH_MAP[legacyPath];
    if (PATH_MAP_KEEP.has(legacyPath)) return legacyPath;
    if (PATH_KEEP_PREFIXES.some(p => legacyPath === p || legacyPath.startsWith(p + '/'))) {
        return legacyPath;
    }
    if (legacyPath.startsWith('/me/')) return legacyPath.replace('/me/', '/users/me/');
    if (legacyPath.startsWith('/perfil/')) return legacyPath.replace('/perfil/', '/users/');
    return legacyPath;
}

/* Recibe el path de openapi.json (`/api/foo/{id}`) y lo convierte en
 * RegExp que matchea paths concretos del frontend (`/foo/123`).
 * Cualquier `{param}` se reemplaza por `[^/]+`. */
function openapiPathToRegex(openapiPath) {
    let pat = openapiPath.replace(/^\/api/, '');
    /* Reemplazar parametros por placeholder unico antes de escapar. */
    const PLACEHOLDER = '\u0000PARAM\u0000';
    pat = pat.replace(/\{[^/}]+\}/g, PLACEHOLDER);
    /* Escapar metacaracteres regex. */
    pat = pat.replace(/[.+?^${}()|[\]\\]/g, '\\$&');
    /* Restaurar placeholder. */
    pat = pat.split(PLACEHOLDER).join('[^/]+');
    return new RegExp('^' + pat + '$');
}

/* Construye un indice METHOD -> [{regex, original}] desde openapi.json. */
export function buildOpenapiIndex(openapiJson) {
    const index = new Map();
    for (const [pathStr, ops] of Object.entries(openapiJson.paths || {})) {
        for (const method of Object.keys(ops)) {
            const m = method.toUpperCase();
            if (!['GET', 'POST', 'PUT', 'PATCH', 'DELETE'].includes(m)) continue;
            if (!index.has(m)) index.set(m, []);
            index.get(m).push({ regex: openapiPathToRegex(pathStr), original: pathStr });
        }
    }
    return index;
}

export function loadOpenapi(repoRoot) {
    const file = path.join(repoRoot, 'openapi.json');
    const json = JSON.parse(fs.readFileSync(file, 'utf8'));
    return { json, index: buildOpenapiIndex(json) };
}

/* Resuelve un call apiGet/Post/Put/Patch/Delete del frontend.
 * Retorna { resolvedPath, matched, matchedOpenapiPath } */
export function resolveCall({ method, legacyPath }, openapiIndex) {
    const m = method.toUpperCase();
    const mapped = mapPath(legacyPath);
    const resolved = '/api' + mapped;
    const candidates = openapiIndex.get(m) || [];
    for (const c of candidates) {
        if (c.regex.test(mapped)) {
            return { resolvedPath: resolved, matched: true, matchedOpenapiPath: c.original };
        }
    }
    return { resolvedPath: resolved, matched: false, matchedOpenapiPath: null };
}

/* Extrae todos los calls apiGet/Post/Put/Patch/Delete de un archivo TS.
 * Solo soporta strings literales (template literals con ${} se omiten porque
 * son dinamicos y no auditables estaticamente sin runtime). */
const CALL_RE = /\bapi(Get|Post|Put|Patch|Delete)\s*<[^>]*>?\s*\(\s*(['"`])([^'"`$\n]+?)\2/g;

export function extractCallsFromSource(source) {
    const calls = [];
    let match;
    while ((match = CALL_RE.exec(source)) !== null) {
        const method = match[1].toUpperCase();
        const legacyPath = match[3];
        if (!legacyPath.startsWith('/')) continue;
        /* Calcular linea (1-based) y columna (1-based) del inicio del literal. */
        const before = source.slice(0, match.index);
        const line = (before.match(/\n/g) || []).length + 1;
        const lastNl = before.lastIndexOf('\n');
        const column = match.index - (lastNl + 1) + 1;
        calls.push({ method, legacyPath, line, column });
    }
    return calls;
}

/* Tambien matchea sin generic: `apiGet('/foo')`. */
const CALL_RE_NO_GENERIC = /\bapi(Get|Post|Put|Patch|Delete)\s*\(\s*(['"`])([^'"`$\n]+?)\2/g;

export function extractAllCalls(source) {
    const seen = new Set();
    const calls = [];
    for (const re of [CALL_RE, CALL_RE_NO_GENERIC]) {
        re.lastIndex = 0;
        let m;
        while ((m = re.exec(source)) !== null) {
            const key = m.index + ':' + m[3];
            if (seen.has(key)) continue;
            seen.add(key);
            const method = m[1].toUpperCase();
            const legacyPath = m[3];
            if (!legacyPath.startsWith('/')) continue;
            const before = source.slice(0, m.index);
            const line = (before.match(/\n/g) || []).length + 1;
            const lastNl = before.lastIndexOf('\n');
            const column = m.index - (lastNl + 1) + 1;
            calls.push({ method, legacyPath, line, column });
        }
    }
    return calls;
}

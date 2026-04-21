/*
 * Util: resolverRutaAsset — Kamples (QL46)
 * En Tauri (desktop/Android APK), las rutas relativas al origin no resuelven al servidor
 * porque el WebView carga desde tauri.localhost. Esta función prefija la URL de producción
 * cuando se detecta el entorno Tauri.
 *
 * [204A-1] En el SPA Vite, los assets del tema WordPress fueron copiados a
 * /legacy-assets/. Reescribimos /wp-content/themes/glorytemplate/App/Assets/* →
 * /legacy-assets/* para que las imagenes y fuentes resuelvan en el navegador.
 */

const SERVIDOR_PROD = 'https://kamples.com';
const PREFIJO_LEGACY_WP = '/wp-content/themes/glorytemplate/App/Assets';
const PREFIJO_SPA = '/legacy-assets';

const esEntornoTauri = (): boolean =>
    typeof window !== 'undefined' && !!(window as unknown as Record<string, unknown>).__KAMPLES_DESKTOP__;

function rebaseAsset(ruta: string): string {
    if (ruta.startsWith(PREFIJO_LEGACY_WP)) {
        return PREFIJO_SPA + ruta.slice(PREFIJO_LEGACY_WP.length);
    }
    return ruta;
}

/**
 * Resuelve una ruta relativa al servidor correcto.
 * En web, retorna la ruta reescrita al SPA (legacy-assets).
 * En Tauri/APK, prefija con el servidor de producción.
 */
export const resolverRutaAsset = (rutaRelativa: string): string => {
    if (esEntornoTauri()) return `${SERVIDOR_PROD}${rutaRelativa}`;
    return rebaseAsset(rutaRelativa);
};

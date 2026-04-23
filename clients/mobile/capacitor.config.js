/* [174A-110b] Capacitor config — SPA Rust empaquetada en APK.
 * - webDir: apunta al build SPA del frontend Rust (Vite). Antes era 'www' (placeholder
 *   que solo redirigía a kamples.com via WebView).
 * - server.url: SOLO si KAMPLES_CAP_SERVER_URL está definido. Útil para live-reload contra
 *   vite dev server (`http://10.0.2.2:5173` en emulador, IP LAN en device físico).
 *   En producción no se define → la APK carga el bundle local de frontend/dist.
 * Gotcha: webDir es relativo a este archivo, por eso `../../frontend/dist`.
 * FCM y deep links Google PKCE ya viven en el SPA (frontend/src/legacy/services/fcmToken.ts
 * y googleAuthMobileCapacitor.ts); se activan automáticamente al cargar el bundle. */

const liveReloadUrl = process.env.KAMPLES_CAP_SERVER_URL?.trim();

/** @type {import('@capacitor/cli').CapacitorConfig} */
const config = {
    appId: 'com.kamples.mobile',
    appName: 'Kamples',
    webDir: '../../frontend/dist',
    android: {
        path: 'android'
    },
    server: liveReloadUrl ? {
        url: liveReloadUrl,
        cleartext: liveReloadUrl.startsWith('http://')
    } : undefined,
    plugins: {
        PushNotifications: {
            presentationOptions: ["badge", "sound", "alert"],
        },
    }
};

module.exports = config;
/// <reference types="vite/client" />

/*
 * Declaración de __KAMPLES_CONFIG__ para que tsc -b (build mode) pueda resolver
 * la referencia en clients/desktop/src/services/apiDesktopAdapter.ts.
 * La declaración canónica vive en clients/desktop/src/global.d.ts;
 * aquí se replica mínimamente para el contexto de compilación del frontend.
 */
declare interface Window {
    __KAMPLES_CONFIG__?: { serverUrl?: string };
}

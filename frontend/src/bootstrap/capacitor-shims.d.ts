/*
 * [204A-1] Stubs de modulos nativos (Capacitor) para que el legacy compile en web.
 * En runtime estos modulos no se ejecutan porque las islas detectan via
 * window.Capacitor.isNativePlatform() si estan en nativo. Aqui solo declaramos
 * los tipos como `any` para que TypeScript no falle al compilar el SPA web.
 */

declare module '@capacitor/filesystem' {
    export const Filesystem: any;
    export const Directory: any;
    export const Encoding: any;
    export type ReadFileResult = any;
    export type WriteFileResult = any;
    export type ReadFileOptions = any;
    export type WriteFileOptions = any;
    const _default: any;
    export default _default;
}

declare module '@capacitor/app' {
    export const App: any;
    const _default: any;
    export default _default;
}

declare module '@capacitor/browser' {
    export const Browser: any;
    const _default: any;
    export default _default;
}

declare module '@capacitor/core' {
    export const Capacitor: any;
    const _default: any;
    export default _default;
}

declare module '@capacitor/push-notifications' {
    export const PushNotifications: any;
    const _default: any;
    export default _default;
}

declare module '@capacitor/share' {
    export const Share: any;
    const _default: any;
    export default _default;
}

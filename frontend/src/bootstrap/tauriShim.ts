/* [204A-1] Stub virtual de @tauri-apps/api/app para builds web.
 * Solo se importa dinamicamente en useVerificadorVersion.ts cuando la app corre
 * dentro de un wrapper Tauri (desktop). En web nunca se ejecuta, pero Rollup
 * necesita resolver el modulo en build time. */

export async function getVersion(): Promise<string> {
    return '0.0.0-web';
}

export default { getVersion };

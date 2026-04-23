/* [174A-110b] Build del frontend SPA Rust para empaquetar en la APK Capacitor.
 * Antes generaba un placeholder www/ que solo redirigía a kamples.com.
 * Ahora delega en `npm run build` del frontend SPA → produce frontend/dist/.
 * Capacitor lee directamente de frontend/dist (ver capacitor.config.js webDir).
 *
 * Variables de entorno honradas:
 *   KAMPLES_CAP_SERVER_URL → live reload contra vite dev (no construye, solo loggea).
 *
 * Idempotente: si el bundle ya existe y --force no se pasó, no reconstruye. */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const directorioFrontend = path.resolve(__dirname, '..', '..', '..', 'frontend');
const directorioDist = path.join(directorioFrontend, 'dist');
const liveReloadUrl = process.env.KAMPLES_CAP_SERVER_URL?.trim();
const forzar = process.argv.includes('--force');

if (liveReloadUrl) {
    console.log(`[prepararWeb] Modo live-reload activo → ${liveReloadUrl}`);
    console.log('[prepararWeb] Saltando build SPA. Asegúrate de tener vite dev server corriendo.');
    /* Capacitor todavía requiere que webDir exista, aunque cargue desde server.url. */
    if (!fs.existsSync(directorioDist)) {
        fs.mkdirSync(directorioDist, { recursive: true });
        fs.writeFileSync(
            path.join(directorioDist, 'index.html'),
            '<!doctype html><html><body>live-reload mode</body></html>',
            'utf8',
        );
    }
    return;
}

if (!forzar && fs.existsSync(path.join(directorioDist, 'index.html'))) {
    console.log('[prepararWeb] frontend/dist ya existe. Usa --force para reconstruir.');
    return;
}

console.log('[prepararWeb] Construyendo SPA Rust (npm run build en frontend/)...');
execSync('npm run build', {
    cwd: directorioFrontend,
    stdio: 'inherit',
});
console.log('[prepararWeb] Build completado en frontend/dist/.');

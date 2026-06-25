#!/usr/bin/env node
/* Launcher de Tauri desktop con gestión de CARGO_TARGET_DIR.
 * Pre-limpieza + watcher periódico + exec de `tauri dev`.
 * Resuelve que `npm run tauri:dev` escribía a C:\tmp\glory-target sin limpieza,
 * acumulando +7GB de artefactos Cargo. */

import { spawn, spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createConnection } from 'node:net';

const __dirname = dirname(fileURLToPath(import.meta.url));
const projectRoot = resolve(__dirname, '..');
const desktopDir = resolve(projectRoot, 'clients', 'desktop');
const frameworkScripts = resolve(projectRoot, 'glory-rs', 'scripts');
const cargoTargetBase = process.env.CARGO_TARGET_DIR_BASE || (process.platform === 'win32' ? 'C:\\tmp\\glory-target' : '/tmp/glory-target');
const maxMb = process.env.GLORY_CARGO_TARGET_MAX_MB || '4096';
const intervalSeconds = process.env.GLORY_CARGO_CLEAN_INTERVAL_SECONDS || '120';
const isWin = process.platform === 'win32';
const backendPort = parseInt(process.env.KAMPLES_BACKEND_PORT || '3000', 10);

function cmdName(name) {
    if (!isWin) return name;
    if (name === 'npm') return 'npm.cmd';
    if (name === 'cargo') return 'cargo.exe';
    if (name === 'powershell') return 'powershell.exe';
    return name;
}

/* Esperar a que un puerto TCP esté disponible */
function waitForPort(port, timeoutMs = 120_000) {
    return new Promise((resolve, reject) => {
        const start = Date.now();
        const tryConnect = () => {
            const sock = createConnection({ host: '127.0.0.1', port });
            sock.once('connect', () => { sock.destroy(); resolve(); });
            sock.once('error', () => {
                sock.destroy();
                if (Date.now() - start > timeoutMs) {
                    reject(new Error(`Timeout esperando puerto ${port}`));
                } else {
                    setTimeout(tryConnect, 800);
                }
            });
        };
        tryConnect();
    });
}

/* Pre-limpieza: forzar antes de iniciar (no hay build activo) */
function runPreClean() {
    const script = resolve(frameworkScripts, 'clean-cargo-target.ps1');
    if (!isWin || !existsSync(script)) return;

    console.log(`[launch-tauri] Pre-limpieza de ${cargoTargetBase} (max ${maxMb} MB)...`);
    const r = spawnSync(cmdName('powershell'), [
        '-ExecutionPolicy', 'Bypass',
        '-File', script,
        '-TargetDirs', cargoTargetBase,
        '-MaxTotalMB', maxMb,
        '-Force',
    ], { stdio: 'inherit', timeout: 60000 });

    if (r.status != null && r.status !== 0) {
        console.warn(`[launch-tauri] Pre-limpieza fallo (exit ${r.status})`);
    }
}

/* Watcher en background */
let watcherProc = null;
let backendProc = null;
function startWatcher() {
    const script = resolve(frameworkScripts, 'watch-cargo-target.ps1');
    if (!isWin || !existsSync(script)) return;

    console.log(`[launch-tauri] Watcher cada ${intervalSeconds}s (max ${maxMb} MB)`);
    watcherProc = spawn(cmdName('powershell'), [
        '-ExecutionPolicy', 'Bypass',
        '-File', script,
        '-TargetDirs', cargoTargetBase,
        '-MaxTotalMB', maxMb,
        '-IntervalSeconds', intervalSeconds,
    ], { stdio: 'ignore', detached: false });
}

function cleanup() {
    if (watcherProc && !watcherProc.killed) watcherProc.kill();
    if (backendProc && !backendProc.killed) backendProc.kill();
}

/* Ejecutar */
async function main() {
    runPreClean();
    startWatcher();

    /* Arrancar backend Rust en background */
    console.log(`[launch-tauri] Iniciando backend Rust (puerto ${backendPort})...`);
    backendProc = spawn('cargo run --bin glory-backend', {
        cwd: projectRoot,
        stdio: 'inherit',
        shell: isWin,
        env: { ...process.env },
    });

    backendProc.on('exit', (code) => {
        if (code != null && code !== 0) {
            console.error(`[launch-tauri] Backend Rust terminó con código ${code}`);
        }
    });

    /* Esperar a que el backend esté listo */
    try {
        await waitForPort(backendPort, 120_000);
        console.log(`[launch-tauri] Backend listo en puerto ${backendPort}`);
    } catch (err) {
        console.error(`[launch-tauri] ${err.message}. Continuando de todas formas...`);
    }

    /* Lanzar Tauri desktop */
    const tauriProc = spawn(cmdName('npm'), ['run', 'tauri:dev'], {
        cwd: desktopDir,
        stdio: 'inherit',
        shell: isWin,
        env: { ...process.env },
    });

    tauriProc.on('exit', (code) => {
        cleanup();
        process.exit(code ?? 0);
    });

    process.on('SIGINT', () => { cleanup(); tauriProc.kill('SIGINT'); });
    process.on('SIGTERM', () => { cleanup(); tauriProc.kill('SIGTERM'); });
}

main();

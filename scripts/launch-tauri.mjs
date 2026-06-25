#!/usr/bin/env node
/* Launcher de Tauri desktop con gestión de CARGO_TARGET_DIR.
 * Pre-limpieza + watcher periódico + exec de `tauri dev`.
 * Resuelve que `npm run tauri:dev` escribía a C:\tmp\glory-target sin limpieza,
 * acumulando +7GB de artefactos Cargo. */

import { spawn, spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const desktopDir = resolve(__dirname, '..', 'clients', 'desktop');
const frameworkScripts = resolve(__dirname, '..', 'glory-rs', 'scripts');
const cargoTargetBase = process.env.CARGO_TARGET_DIR_BASE || (process.platform === 'win32' ? 'C:\\tmp\\glory-target' : '/tmp/glory-target');
const maxMb = process.env.GLORY_CARGO_TARGET_MAX_MB || '4096';
const intervalSeconds = process.env.GLORY_CARGO_CLEAN_INTERVAL_SECONDS || '120';
const isWin = process.platform === 'win32';

function cmdName(name) {
    if (!isWin) return name;
    if (name === 'npm') return 'npm.cmd';
    if (name === 'powershell') return 'powershell.exe';
    return name;
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
    if (watcherProc && !watcherProc.killed) {
        watcherProc.kill();
    }
}

/* Ejecutar */
runPreClean();
startWatcher();

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

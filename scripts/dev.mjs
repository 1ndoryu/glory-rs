#!/usr/bin/env node

/* [dev.mjs] Script de desarrollo con BD por rama.
 * Detecta la rama git actual, deriva el nombre de BD, la crea si no existe,
 * y lanza el stack completo (backend + frontend) con DATABASE_URL correcto.
 * La logica de deteccion vive en branch-db.mjs para ser reutilizable. */

import { spawn } from 'node:child_process';
import { readFileSync, existsSync } from 'node:fs';
import path from 'node:path';
import { getBranchDbUrl } from './branch-db.mjs';

const backgroundChildren = [];

function cleanup() {
  for (const child of backgroundChildren) {
    if (!child.killed) {
      child.kill();
    }
  }
}

function spawnCargoTargetWatcher(env, cargoTargetRoot) {
  if (process.platform !== 'win32') {
    return;
  }

  const watcherScript = path.resolve('scripts/watch-cargo-target.ps1');
  if (!existsSync(watcherScript)) {
    return;
  }

  const watcher = spawn(
    'powershell',
    [
      '-ExecutionPolicy',
      'Bypass',
      '-File',
      watcherScript,
      '-TargetDirs',
      cargoTargetRoot,
    ],
    { stdio: 'ignore', env, shell: false },
  );

  watcher.on('error', (err) => {
    console.error('[cargo-target-watch] Error:', err.message);
  });
  backgroundChildren.push(watcher);
}

process.on('SIGINT', cleanup);
process.on('SIGTERM', cleanup);

console.log('');
const { dbUrl, cargoTargetDir } = getBranchDbUrl();
console.log('');

const childEnv = { ...process.env, DATABASE_URL: dbUrl, CARGO_TARGET_DIR: cargoTargetDir };
spawnCargoTargetWatcher(childEnv, path.dirname(cargoTargetDir));

/* Encontrar el entry point JS de concurrently para invocarlo con node directamente.
 * Esto evita dependencia de shell y problemas con .cmd en Windows. */
const concurrentlyPkg = JSON.parse(
  readFileSync(path.resolve('node_modules/concurrently/package.json'), 'utf8'),
);
const concurrentlyBinRelative =
  typeof concurrentlyPkg.bin === 'string'
    ? concurrentlyPkg.bin
    : concurrentlyPkg.bin?.concurrently ?? 'dist/bin/concurrently.js';
const concurrentlyEntry = path.resolve('node_modules/concurrently', concurrentlyBinRelative);

const child = spawn(
  process.execPath,
  [
    concurrentlyEntry,
    '--names', 'BACK,FRONT',
    '--prefix-colors', 'blue,green',
    'node scripts/run-cargo.mjs run --bin glory-backend',
    'npm --prefix frontend run dev',
  ],
  { stdio: 'inherit', env: childEnv, shell: false },
);

child.on('error', (err) => { console.error('[dev] Error:', err.message); process.exit(1); });
child.on('exit', (code) => {
  cleanup();
  process.exit(code ?? 0);
});

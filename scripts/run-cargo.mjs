#!/usr/bin/env node

/* [144A-1] Centraliza la ejecucion de Cargo para los scripts npm.
 * Permite usar la instalacion estandar de rustup aunque el PATH de la terminal
 * todavia no se haya refrescado y evita el error opaco de cmd.exe en Windows.
 * Pendiente: si el proyecto necesita bootstrap automatico de Rust, resolverlo
 * fuera del repo para no instalar toolchains sin consentimiento explicito. */

import { spawn } from 'node:child_process';
import { accessSync, constants } from 'node:fs';
import os from 'node:os';
import path from 'node:path';

const taskId = '144A-1';
const cargoArgs = process.argv.slice(2);
const executableName = process.platform === 'win32' ? 'cargo.exe' : 'cargo';

function fileExists(filePath) {
  try {
    accessSync(filePath, constants.F_OK);
    return true;
  } catch {
    return false;
  }
}

function unique(values) {
  return [...new Set(values.filter(Boolean))];
}

function buildCandidates() {
  const homeDir = os.homedir();
  const pathEntries = (process.env.PATH ?? '').split(path.delimiter);

  return unique([
    process.env.CARGO,
    ...pathEntries.map((entry) => path.join(entry, executableName)),
    homeDir ? path.join(homeDir, '.cargo', 'bin', executableName) : '',
  ]);
}

function resolveCargoPath() {
  return buildCandidates().find(fileExists);
}

function printMissingCargoMessage() {
  const cargoHome = path.join(os.homedir(), '.cargo', 'bin');
  const installCommand =
    process.platform === 'win32'
      ? 'winget install --id Rustlang.Rustup --exact --accept-source-agreements --accept-package-agreements'
      : 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh';

  console.error(`[${taskId}] No se encontro Cargo. El backend Rust no puede iniciarse sin la toolchain.`);
  console.error(`Instala rustup y vuelve a abrir la terminal: ${installCommand}`);
  console.error(`Ruta esperada por defecto: ${cargoHome}`);
}

if (cargoArgs.length === 0) {
  console.error('Uso: node scripts/run-cargo.mjs <subcomando cargo> [...args]');
  process.exit(1);
}

const cargoPath = resolveCargoPath();

if (!cargoPath) {
  printMissingCargoMessage();
  process.exit(1);
}

const childEnv = {
  ...process.env,
  PATH: unique([path.dirname(cargoPath), process.env.PATH ?? '']).join(path.delimiter),
};

const childProcess = spawn(cargoPath, cargoArgs, {
  stdio: 'inherit',
  env: childEnv,
});

childProcess.on('error', (error) => {
  console.error(`[run-cargo] Error al ejecutar Cargo: ${error.message}`);
  process.exit(1);
});

childProcess.on('exit', (code, signal) => {
  if (signal) {
    process.kill(process.pid, signal);
    return;
  }

  process.exit(code ?? 1);
});
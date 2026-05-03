#!/usr/bin/env node

/* Launcher de desarrollo generico para el template.
 * Deriva DB y cargo target desde Cargo.toml + rama, arranca un watcher que limpia
 * el pool completo de targets y levanta backend + frontend con el entorno correcto. */

import { spawn, spawnSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { getBranchDbContext, findPsql } from './branch-db.mjs';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const projectRoot = path.resolve(__dirname, '..');
const cargoToml = path.join(projectRoot, 'Cargo.toml');
const frontendDir = path.join(projectRoot, 'frontend');
const children = [];

function commandName(cmd) {
  if (process.platform !== 'win32') {
    return cmd;
  }
  if (cmd === 'npm') return 'npm.cmd';
  if (cmd === 'cargo') return 'cargo.exe';
  if (cmd === 'git') return 'git.exe';
  if (cmd === 'powershell') return 'powershell.exe';
  if (cmd === 'psql') return 'psql.exe';
  return cmd;
}

function parseDbUrl(databaseUrl) {
  return new URL(databaseUrl);
}

function resetPublicSchema(psqlBin, databaseUrl) {
  const parsed = parseDbUrl(databaseUrl);
  const env = { ...process.env, PGPASSWORD: decodeURIComponent(parsed.password) };
  const args = [
    '-h',
    parsed.hostname,
    '-p',
    parsed.port || '5432',
    '-U',
    decodeURIComponent(parsed.username),
    '-d',
    parsed.pathname.slice(1),
    '-c',
    'DROP SCHEMA public CASCADE; CREATE SCHEMA public; GRANT ALL ON SCHEMA public TO CURRENT_USER;',
  ];
  return spawnSync(psqlBin, args, { cwd: projectRoot, env, encoding: 'utf8' });
}

function runSqlxMigrations(databaseUrl, cargoTargetDir) {
  return spawnSync(commandName('cargo'), ['sqlx', 'migrate', 'run'], {
    cwd: projectRoot,
    encoding: 'utf8',
    env: { ...process.env, DATABASE_URL: databaseUrl, CARGO_TARGET_DIR: cargoTargetDir },
  });
}

function ensureMigrationsAreCompatible(databaseUrl, dbName, cargoTargetDir) {
  const firstRun = runSqlxMigrations(databaseUrl, cargoTargetDir);
  if (firstRun.status === 0) {
    return;
  }

  const output = `${firstRun.stdout}\n${firstRun.stderr}`;
  if (!/VersionMissing|VersionMismatch|previously applied but has been modified/.test(output)) {
    process.stdout.write(firstRun.stdout || '');
    process.stderr.write(firstRun.stderr || '');
    console.error('[glory-dev] No se pudieron aplicar las migraciones locales.');
    process.exit(firstRun.status ?? 1);
  }

  const psqlBin = findPsql();
  if (!psqlBin) {
    process.stderr.write(firstRun.stderr || '');
    console.error('[glory-dev] La BD local tiene migraciones incompatibles, pero psql no esta disponible para resetearla.');
    process.exit(firstRun.status ?? 1);
  }

  console.warn(`[glory-dev] Historial de migraciones incompatible en ${dbName}; reseteando schema public de desarrollo.`);
  const resetResult = resetPublicSchema(psqlBin, databaseUrl);
  if (resetResult.status !== 0) {
    process.stderr.write(resetResult.stderr || '');
    console.error('[glory-dev] No se pudo resetear la BD local de desarrollo.');
    process.exit(resetResult.status ?? 1);
  }

  const secondRun = runSqlxMigrations(databaseUrl, cargoTargetDir);
  if (secondRun.status !== 0) {
    process.stdout.write(secondRun.stdout || '');
    process.stderr.write(secondRun.stderr || '');
    console.error('[glory-dev] Las migraciones siguen fallando tras resetear la BD local.');
    process.exit(secondRun.status ?? 1);
  }
}

function resolveRustcWrapper() {
  if (process.env.RUSTC_WRAPPER) {
    return process.env.RUSTC_WRAPPER;
  }

  if (process.platform === 'win32' && process.env.USERPROFILE) {
    const sccachePath = path.join(process.env.USERPROFILE, '.cargo', 'bin', 'sccache.exe');
    if (existsSync(sccachePath)) {
      return sccachePath;
    }
  }

  const command = spawnSync('sccache', ['--version'], { encoding: 'utf8' });
  return command.status === 0 ? 'sccache' : null;
}

function spawnProc(label, cmd, args, options) {
  const proc = spawn(commandName(cmd), args, {
    stdio: 'inherit',
    shell: false,
    ...options,
  });
  proc.on('error', (err) => console.error(`[${label}] Error: ${err.message}`));
  proc.on('exit', (code) => {
    console.log(`[${label}] Proceso terminado con codigo ${code}`);
    cleanup();
  });
  children.push(proc);
  return proc;
}

function cleanup() {
  for (const child of children) {
    if (!child.killed) {
      child.kill();
    }
  }
}

function spawnCargoTargetWatcher(env, cargoTargetBase) {
  if (process.platform !== 'win32') {
    return;
  }

  const watcherScript = path.join(projectRoot, 'scripts', 'watch-cargo-target.ps1');
  if (!existsSync(watcherScript)) {
    return;
  }

  const watcher = spawn(commandName('powershell'), [
    '-ExecutionPolicy',
    'Bypass',
    '-File',
    watcherScript,
    '-TargetDirs',
    cargoTargetBase,
  ], {
    cwd: projectRoot,
    env,
    stdio: 'ignore',
    shell: false,
  });
  watcher.on('error', (err) => console.error(`[cargo-target-watch] Error: ${err.message}`));
  children.push(watcher);
}

function detectBinName() {
  const toml = readFileSync(cargoToml, 'utf8');
  const match = toml.match(/^name\s*=\s*"([^"]+)"/m);
  return match ? match[1] : null;
}

process.on('SIGINT', cleanup);
process.on('SIGTERM', cleanup);

if (!existsSync(cargoToml)) {
  console.error('[glory-dev] No se encontro Cargo.toml en el proyecto.');
  process.exit(1);
}

if (!existsSync(frontendDir)) {
  console.error('[glory-dev] No se encontro frontend/ en el proyecto.');
  process.exit(1);
}

const { branch, dbName, dbUrl, cargoTargetBase, cargoTargetDir } = getBranchDbContext();
const binName = detectBinName();
if (!binName) {
  console.error('[glory-dev] No se pudo detectar el nombre del binario en Cargo.toml');
  process.exit(1);
}

ensureMigrationsAreCompatible(dbUrl, dbName, cargoTargetDir);

const childEnv = {
  ...process.env,
  DATABASE_URL: dbUrl,
  CARGO_TARGET_DIR: cargoTargetDir,
  GLORY_DEV_BRANCH: branch,
  GLORY_DEV_DB_NAME: dbName,
};

const rustcWrapper = resolveRustcWrapper();
if (rustcWrapper) {
  childEnv.RUSTC_WRAPPER = rustcWrapper;
}

console.log(`[glory-dev] Rama: ${branch}`);
console.log(`[glory-dev] Base local: ${dbName}`);
console.log(`[glory-dev] Cargo target: ${cargoTargetDir}`);
if (rustcWrapper) {
  console.log(`[glory-dev] Rust cache: ${rustcWrapper}`);
}
console.log(`[glory-dev] Iniciando backend (cargo run --bin ${binName}) y frontend (vite)...\n`);

spawnCargoTargetWatcher(childEnv, cargoTargetBase);
spawnProc('backend', 'cargo', ['run', '--bin', binName], { cwd: projectRoot, env: childEnv });
spawnProc('frontend', 'npm', ['--prefix', 'frontend', 'run', 'dev'], { cwd: projectRoot, env: childEnv });

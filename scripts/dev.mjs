#!/usr/bin/env node

/* [dev.mjs] Script de desarrollo con BD por rama.
 * Detecta la rama git actual, deriva el nombre de BD, la crea si no existe,
 * y lanza el stack completo (backend + frontend) con DATABASE_URL correcto.
 * Convencion: {base_db}_{branch_slug}. main/master usan el nombre base sin sufijo.
 * Esto permite cambiar de rama sin conflictos de schema entre proyectos. */

import { execSync, execFileSync, spawn } from 'node:child_process';
import { readFileSync, existsSync, readdirSync } from 'node:fs';
import path from 'node:path';

/* --- 1. Rama actual --- */
let branch = 'main';
try {
  branch = execSync('git rev-parse --abbrev-ref HEAD', {
    encoding: 'utf8',
    stdio: ['pipe', 'pipe', 'pipe'],
  }).trim();
} catch {
  console.warn('[dev] No se pudo detectar la rama, usando "main"');
}

const branchSlug = branch.replace(/[^a-zA-Z0-9]/g, '_').toLowerCase();

/* --- 2. Parsear .env --- */
function parseEnvFile(content) {
  const vars = {};
  for (const line of content.split(/\r?\n/)) {
    const m = line.match(/^([A-Z_][A-Z0-9_]*)=(.*)$/);
    if (m) vars[m[1]] = m[2].trim();
  }
  return vars;
}

const envPath = path.resolve('.env');
const envVars = existsSync(envPath) ? parseEnvFile(readFileSync(envPath, 'utf8')) : {};
const baseDbUrl = envVars.DATABASE_URL;

if (!baseDbUrl) {
  console.error('[dev] DATABASE_URL no encontrado en .env');
  process.exit(1);
}

/* --- 3. Parsear URL de conexion --- */
const urlMatch = baseDbUrl.match(/^postgres(?:ql)?:\/\/([^:]+):([^@]+)@([^:/]+):(\d+)\/(.+)$/);
if (!urlMatch) {
  console.error('[dev] No se pudo parsear DATABASE_URL:', baseDbUrl);
  process.exit(1);
}
const [, dbUser, dbPass, dbHost, dbPort, baseDbName] = urlMatch;

/* --- 4. Nombre de BD por rama --- */
const isDefaultBranch = branch === 'main' || branch === 'master';
const dbName = isDefaultBranch ? baseDbName : `${baseDbName}_${branchSlug}`;
const dbUrl = `postgres://${dbUser}:${dbPass}@${dbHost}:${dbPort}/${dbName}`;

console.log(`\x1b[36m[dev] Rama:         \x1b[0m ${branch}`);
console.log(`\x1b[36m[dev] Base de datos:\x1b[0m ${dbName}`);

/* --- 5. Encontrar psql en Windows o usar el del PATH --- */
function findPsql() {
  if (process.platform === 'win32') {
    const pgRoot = 'C:\\Program Files\\PostgreSQL';
    if (existsSync(pgRoot)) {
      try {
        const versions = readdirSync(pgRoot)
          .filter((v) => /^\d+$/.test(v))
          .sort((a, b) => parseInt(b) - parseInt(a));
        for (const v of versions) {
          const candidate = path.join(pgRoot, v, 'bin', 'psql.exe');
          if (existsSync(candidate)) return candidate;
        }
      } catch { /* no PostgreSQL instalado */ }
    }
  }
  return 'psql';
}

const psqlBin = findPsql();
const psqlEnv = { ...process.env, PGPASSWORD: dbPass };

/* --- 6. Crear BD si no existe --- */
function psqlQuery(sql, db = 'postgres') {
  return execFileSync(psqlBin, ['-U', dbUser, '-h', dbHost, '-p', dbPort, '-d', db, '-t', '-A', '-c', sql], {
    env: psqlEnv,
    encoding: 'utf8',
    stdio: ['pipe', 'pipe', 'pipe'],
  });
}

try {
  const exists = psqlQuery(`SELECT 1 FROM pg_database WHERE datname = '${dbName}'`).trim();
  if (exists === '1') {
    console.log(`\x1b[32m[dev] BD existente: ${dbName}\x1b[0m`);
  } else {
    psqlQuery(`CREATE DATABASE "${dbName}"`);
    console.log(`\x1b[32m[dev] BD creada: ${dbName}\x1b[0m`);
  }
} catch (e) {
  console.error(`\x1b[31m[dev] Error con la base de datos: ${e.message}\x1b[0m`);
  process.exit(1);
}

/* --- 7. Lanzar stack dev con DATABASE_URL de la rama --- */
const nodeBin = path.resolve('node_modules', '.bin');

const childEnv = {
  ...process.env,
  DATABASE_URL: dbUrl,
  PATH: [nodeBin, process.env.PATH].filter(Boolean).join(path.delimiter),
};

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

console.log('');

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

child.on('error', (err) => {
  console.error('[dev] Error al lanzar concurrently:', err.message);
  process.exit(1);
});

child.on('exit', (code) => process.exit(code ?? 0));

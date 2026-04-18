import { existsSync, readFileSync, watchFile } from 'node:fs';
import path from 'node:path';

const watchMode = process.argv.includes('--watch');
const candidates = [
  path.resolve(process.cwd(), 'roadmap.md'),
  path.resolve(process.cwd(), 'App', 'roadmap.md'),
].filter(filePath => existsSync(filePath));

function collectTasks(filePath) {
  const lines = readFileSync(filePath, 'utf8').split(/\r?\n/);
  const tasks = [];
  let inPendingSection = false;
  let pendingHeadingLevel = 0;

  for (let index = 0; index < lines.length; index += 1) {
    const rawLine = lines[index] ?? '';
    const line = rawLine.trim();

    const headingMatch = line.match(/^(#+)\s+/);
    if (headingMatch) {
      const headingLevel = headingMatch[1].length;
      if (/pendiente/i.test(line)) {
        inPendingSection = true;
        pendingHeadingLevel = headingLevel;
      } else if (inPendingSection && headingLevel <= pendingHeadingLevel) {
        inPendingSection = false;
        pendingHeadingLevel = 0;
      }

      if (!inPendingSection && !/pendiente/i.test(line)) {
        continue;
      }
    }

    if (!inPendingSection) {
      continue;
    }

    if (!line || line === '(sin tareas pendientes)' || /^\([^)]*\)$/.test(line)) {
      continue;
    }

    if (/^(?:-|--|###)\s+/.test(line)) {
      tasks.push({
        filePath,
        line: index + 1,
        column: 1,
        text: line.replace(/^(?:-|--|###)\s+/, '').trim(),
      });
    }
  }

  return tasks;
}

function report() {
  const tasks = candidates.flatMap(collectTasks);

  for (const task of tasks) {
    console.log(`${task.filePath}:${task.line}:${task.column}: warning: TAREA PENDIENTE: ${task.text}`);
  }

  if (watchMode) {
    if (tasks.length > 0) {
      console.log(`[roadmap-watcher] ${tasks.length} tarea(s) pendientes detectadas`);
    } else {
      console.log('[roadmap-watcher] Sin tareas');
    }
    return;
  }

  process.exitCode = tasks.length > 0 ? 1 : 0;
}

if (candidates.length === 0) {
  console.log('[roadmap-watcher] Sin roadmap.md para analizar');
  process.exit(0);
}

if (watchMode) {
  console.log('[roadmap-watcher] Vigilando roadmap(s)...');
  report();
  for (const filePath of candidates) {
    watchFile(filePath, { interval: 2000 }, () => {
      console.log(`[roadmap-watcher] Cambio detectado: ${filePath}`);
      report();
    });
  }
} else {
  report();
}
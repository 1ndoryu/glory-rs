#!/usr/bin/env node

/* [144A-1] Implementa el watcher del roadmap usado por la tarea de VS Code.
 * Lee solo el bloque "Tareas pendientes" y emite warnings compatibles con el
 * problem matcher configurado en .vscode/tasks.json.
 * Gotcha: el script debe soportar modo one-shot y --watch con mensajes estables. */

import { readFileSync, watchFile } from 'node:fs';
import path from 'node:path';
import process from 'node:process';

const watchMode = process.argv.includes('--watch');
const roadmapPath = path.resolve(process.cwd(), 'roadmap.md');

function pluralize(count) {
  return count === 1 ? 'tarea pendiente detectada' : 'tareas pendientes detectadas';
}

function normalizeTaskLine(rawLine) {
  const trimmedLine = rawLine.trim();

  if (!trimmedLine) {
    return null;
  }

  if (/^\(sin tareas pendientes\)$/i.test(trimmedLine)) {
    return null;
  }

  if (/^\(.*\)$/.test(trimmedLine)) {
    return null;
  }

  return trimmedLine.replace(/^[-*]\s+/, '').replace(/^\d+\.\s+/, '');
}

function extractPendingTasks(content) {
  const lines = content.split(/\r?\n/);
  const tasks = [];
  let insidePendingSection = false;

  lines.forEach((line, index) => {
    const trimmedLine = line.trim();

    if (!insidePendingSection) {
      if (/^##\s+Tareas pendientes$/i.test(trimmedLine)) {
        insidePendingSection = true;
      }

      return;
    }

    if (/^##\s+/.test(trimmedLine)) {
      insidePendingSection = false;
      return;
    }

    const normalizedTask = normalizeTaskLine(line);

    if (!normalizedTask) {
      return;
    }

    tasks.push({
      line: index + 1,
      text: normalizedTask,
    });
  });

  return tasks;
}

function scanRoadmap() {
  const roadmapContent = readFileSync(roadmapPath, 'utf8');
  const pendingTasks = extractPendingTasks(roadmapContent);

  if (pendingTasks.length === 0) {
    console.log('[roadmap-watcher] Sin tareas pendientes');
    return pendingTasks;
  }

  pendingTasks.forEach((task) => {
    console.log(`${roadmapPath}:${task.line}:1: warning: ${task.text}`);
  });

  console.log(`[roadmap-watcher] ${pendingTasks.length} ${pluralize(pendingTasks.length)}`);
  return pendingTasks;
}

function runScan() {
  try {
    scanRoadmap();
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`[roadmap-watcher] Error: ${message}`);
    process.exitCode = 1;
  }
}

if (!watchMode) {
  runScan();
  process.exit();
}

console.log(`[roadmap-watcher] Vigilando ${roadmapPath}`);
runScan();

watchFile(roadmapPath, { interval: 500 }, (currentStats, previousStats) => {
  if (currentStats.mtimeMs === previousStats.mtimeMs) {
    return;
  }

  console.log('[roadmap-watcher] Cambio detectado');
  runScan();
});
# Lecciones Aprendidas

Registro de errores recurrentes, patrones que funcionaron, y conocimiento adquirido durante el desarrollo.
Cada lección debe ser concisa y accionable.

---

## 2026-03-25 — Emojis Unicode en JSX

**Problema:** Se usaron emojis Unicode directamente en componentes React en vez de SVG/iconos.
**Causa raíz:** No había regla que lo prohibiera ni detección automática.
**Solución:** Regla 18 en protocolo + regla `emoji-en-codigo` en Glory Sentinel.
**Acción preventiva:** Sentinel lo detecta automáticamente ahora.

## 2026-03-25 — node_modules/ raíz sin .gitignore

**Problema:** `npm install` en la raíz creó `node_modules/` que no estaba en `.gitignore`, causando 2596 archivos sin trackear.
**Causa raíz:** `.gitignore` solo excluía `/frontend/node_modules/`, no `node_modules/` global.
**Solución:** Añadido `node_modules/` sin prefijo al `.gitignore`.
**Acción preventiva:** Regla 12.1 en protocolo (git limpio).

## 2026-03-25 — Comentarios JS con glob paths

**Problema:** `/* **/node_modules/** */` dentro de comentarios JS rompe el parser de TypeScript porque interpreta `*/` como cierre de comentario.
**Solución:** No usar patrones glob con `**/` dentro de comentarios JS/TS.

## 2026-03-25 — Orval v8 customInstance signature

**Problema:** Login no funcionaba. `customInstance` retornaba `response.data` pero Orval v8 genera tipos que esperan `{ data, status, headers }`.
**Causa raíz:** Los componentes checaban `respuesta.status === 200` que nunca era true.
**Solución:** Retornar `{ data: response.data, status: response.status, headers: response.headers } as T`.

## 2026-03-25 — PowerShell 5 limitaciones en scripts

**Problema:** Scripts PS1 con em dash (`—`), ternarios inline, o comas entre hashtables en arrays fallan en PS5.
**Causa raíz:** PS5 tiene parser más limitado que PS7.
**Solución:** Solo ASCII, if/else estándar, sin comas entre elementos de array de hashtables.

## 2026-03-25 — Git submodule con ruta local

**Problema:** `git submodule add ../glory-rs-framework glory-rs` resuelve contra la URL remota de GitHub, no contra el filesystem local.
**Causa raíz:** Git interpreta paths relativos en relación al remote origin, no al directorio actual.
**Solución:** Usar path absoluto + `git config --global protocol.file.allow always` (Git moderno bloquea file:// por defecto).

## 2026-03-25 — TypeScript en submódulo necesita sus propias deps

**Problema:** Componentes TSX en un submódulo no resuelven `react` types. Errores como "Property 'children' does not exist on BotonProps" aunque extends `ButtonHTMLAttributes`.
**Causa raíz:** TypeScript busca `node_modules` ascendiendo desde la ubicación del archivo. El submódulo no tiene `node_modules` y el del proyecto consumidor está en `frontend/node_modules`, no en la raíz.
**Solución:** El submódulo necesita su propio `package.json` con devDependencies (react, @types/react, lucide-react) y `npm install`. También necesita `server.fs.allow: ['..']` en Vite.

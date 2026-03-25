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

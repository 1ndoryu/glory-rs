# Prevención: block comments anidados en Rust

**Fecha:** 2026-04-20
**Tarea origen:** 174A-108b
**Status:** PENDIENTE — implementar en Code Sentinel

## Problema

Rust soporta block comments **anidables**: `/* ... /* ... */ ... */`. Cualquier ocurrencia literal de `/*` dentro de un block comment abre un nivel anidado, y el `*/` correspondiente cierra solo ese nivel. Esto causa errores `unterminated block comment` que NO son obvios al leer el código.

Caso real (línea 54 de `src/lib.rs`):

```rust
/* [174A-108b] Secret compartido con el scraper Python para autenticar
 * llamadas a `/api/admin/scraper/*` sin sesión de usuario. */
pub scraper_secret: Option<String>,
```

El `/*` dentro de `scraper/*` (la barra antes del asterisco-glob) abre un nivel anidado. El `*/` final cierra ese nivel pero deja el outer abierto → error.

Mismo patrón observado en `src/config/mod.rs`.

## Regla a implementar

Detectar en archivos `.rs`:
1. Dentro de un `/* ... */` (line comment `//` excluido), buscar ocurrencias de `/*` o `*/` literales que no estén balanceadas.
2. Reportar como warning con sugerencia: "Block comment contiene `/` seguido de `*` (o viceversa) — Rust lo interpretará como apertura de comentario anidado. Reformula el contenido sin esos pares."

## Implementación sugerida

En `code-sentinel/src/analyzers/`:
- Nuevo analyzer `rustNestedBlockCommentAnalyzer.ts`.
- Tokenizar comments con un mini-parser: encontrar `/*`, contar nivel hasta `*/`, si nivel > 1 al cierre o queda abierto al EOF → warning.
- Aplicar solo a archivos `.rs`.

## Test case

```rust
/* foo `/api/x/*` bar */          // <-- DEBE detectar
/* normal comment */              // <-- OK
// /* esto va en line comment */  // <-- OK (es line comment)
/* outer /* inner */ outer */     // <-- OK (anidado bien balanceado)
/* outer /* inner */              // <-- DEBE detectar (unbalanced)
```

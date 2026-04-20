# `frontend/src/features/` — Migración progresiva de islands legacy

[174A-102] Origen: `glorytemplate/App/React/islands/`.

Cada subdirectorio aquí es la copia bruta de un dominio del proyecto PHP. NO
están en el `include` de `tsconfig.json` todavía: el compilador los ignora
porque dependen de `services/`, `stores/` y `hooks/` del legado que aún
no se han migrado al cliente Orval.

El flujo por feature será:

1. Tomar la feature (ej. `samples/`).
2. Migrar sus servicios → hooks Orval/React Query (174A-104).
3. Reemplazar imports de `App/React/services/...` por `@/api/generated/...`.
4. Sacar la carpeta del `exclude` de `tsconfig.json`.
5. `npm run type-check` debe seguir verde antes de commitear.

**No tocar el contenido de las carpetas hasta migrarlas** — son referencia
funcional palabra-por-palabra del comportamiento PHP, conservar 1:1 hasta
que la feature esté validada en el SPA nuevo.

## Dominios copiados (17)

admin · auth · blog · canciones · colecciones · comunidad · dev · discover ·
explorador · feed · legal · libreria · mensajes · planes · player · samples ·
social

(`BienvenidaIsland.tsx` se copió como archivo suelto en la raíz de `features/`.)

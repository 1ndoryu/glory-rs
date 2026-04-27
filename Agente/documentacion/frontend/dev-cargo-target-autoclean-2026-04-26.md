# Dev cargo target autoclean — 2026-04-26

## Contexto

`npm run dev` en la raiz levanta backend Rust (`cargo run`) y frontend Vite al mismo tiempo. El backend compila en `C:/tmp/glory-target` por `.cargo/config.toml`.

En sesiones largas, el target podia crecer varios GB por `incremental/`, `deps/` y outputs auxiliares. Ya existia `scripts/clean-cargo-target.ps1`, pero su modo default no tocaba `incremental/`, por lo que la autolimpieza al salir no frenaba el crecimiento durante una sesion activa.

## Cambio aplicado

- `scripts/clean-cargo-target.ps1`
  - agrega `MaxTotalMB` (default 4096 MB)
  - en modo default, si el target supera el tope, poda `debug/incremental` y `release/incremental`
  - `-Hard` deja de matar `glory-backend`; ahora solo elimina `incremental/`
  - `-Aggressive` sigue siendo la opcion que mata el backend y borra `deps/`, `build/` y `.fingerprint/`
- `scripts/watch-cargo-target.ps1`
  - nuevo watcher ligero para sesiones largas
  - revisa el tamaño del target cada 300 s
  - si supera el tope y no hay `rustc` activo, dispara limpieza `-Hard`
- `package.json`
  - `predev` ejecuta limpieza segura antes de arrancar
  - `dev` ahora levanta `dev:sweep` junto a backend y frontend
  - `dev:clean` pasa a usar limpieza agresiva antes de arrancar
  - `postdev` conserva la poda segura al finalizar

## Razon

La solucion evita que el usuario tenga que acordarse de limpiar manualmente, pero sin romper el ciclo de desarrollo:

- no toca el backend en marcha cuando solo hay que vaciar `incremental/`
- no compite con `rustc` mientras compila
- mantiene una salida manual (`clean:cargo:hard`) para recuperacion fuerte

## Gotchas

- Si `C:/tmp/glory-target` no existe, ambos scripts salen sin error.
- El watcher esta pensado para Windows/PowerShell, igual que los scripts npm del proyecto.
- El tope de 4096 MB es conservador; si el proyecto cambia de escala puede ajustarse sin tocar el flujo.

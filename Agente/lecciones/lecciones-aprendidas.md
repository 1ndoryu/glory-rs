# Lecciones aprendidas

- 2026-04-29: En limpieza de `C:\tmp\glory-target`, no basta con borrar `incremental/`; si `deps/` concentra el peso, el script debe validar el tamano final contra `MaxTotalMB` y escalar a caches regenerables antes de imprimir `OK`.
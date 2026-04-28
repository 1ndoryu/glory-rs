# Prevencion directory-size legacy - 2026-04-28

- **Origen:** Code Sentinel reporta `directory-size` al tocar archivos dentro de `src/handlers/` y `src/services/`, aunque el cambio nuevo ya usa subdirectorios de dominio (`services/contribuciones`, `services/dev`).
- **Problema:** La regla apunta al archivo editado, no al acto de crear un archivo nuevo en un directorio plano. Esto genera falsos positivos cuando se modifica un modulo raiz legacy necesario para registrar rutas/servicios.
- **Regla a mejorar:** `directory-size` debe distinguir entre agregar archivos nuevos al directorio saturado y editar archivos existentes obligatorios (`mod.rs`, handlers legacy ya presentes). Para estos casos debe emitir recomendacion informativa o sugerir migracion gradual, no error bloqueante.
- **Caso original:** `src/handlers/admin_contribuciones.rs` y `src/services/mod.rs` durante 274A-23..26+48.

# 283A-36: Reportar error + Coolify fix + Zoom sincronizado

## Tareas: 29, 30, 31

### Tarea 29 — Coolify apps duplicadas
- Compose actualizado: eliminada aplicación `rust-app` fantasma
- Solo quedan `app` (port 3000) + `postgres` en el compose
- Ejecutado PATCH via API Coolify + restart del servicio
- Variables de entorno verificadas: DB_PASSWORD y JWT_SECRET existen

### Tarea 30 — Botón reportar errores
- Agregado botón "Reportar error" (icono Bug) en sidebar secundario
- Abre modal con textarea para describir el problema
- Envía POST a `/api/reportar-error` (endpoint existente desde 283A-26)
- El backend envía el reporte por email al admin

### Tarea 31 — Zoom sincronizado plano de sala
- Creado `stores/zoomStore.ts` (Zustand) con persistencia en localStorage
- Zoom compartido entre PlanoSala (editor) y PlanoOcupacion (reservas)
- Mínimo: 50%, Máximo: 200%, Paso: 10%
- Controles de zoom (+/-) agregados también en PlanoOcupacion
- El zoom se mantiene entre sesiones del navegador

## Archivos modificados
- `frontend/src/components/app-sidebar.tsx` — botón reportar error + modal
- `frontend/src/stores/zoomStore.ts` — NUEVO, store Zustand zoom compartido
- `frontend/src/componentes/plano-sala/usePlanoSala.ts` — usa zoomStore
- `frontend/src/componentes/PlanoSala.tsx` — zoom min 0.5, step 0.1
- `frontend/src/componentes/PlanoOcupacion.tsx` — zoom + controles
- `scripts/update-coolify-compose.ps1` — sin rust-app
- `roadmap.md` — tareas 29, 30, 31 marcadas done

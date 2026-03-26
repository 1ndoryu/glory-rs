# Plan: Plano de Sala (263A-14)

## Estado: EN PROGRESO

## Fases

### Fase A — Backend (migración + CRUD)
- [ ] Migración: `zonas_sala`, `mesas`, `combinaciones_mesas`, `combinacion_mesa_items`, `ALTER reservas ADD mesa_id`
- [ ] Modelos: `ZonaSala`, `Mesa`, `CombinacionMesas`, requests
- [ ] Repositorios: CRUD zonas, mesas (con posición), combinaciones, consulta plano completo
- [ ] Servicios: orquestación plano completo, exportar/importar JSON
- [ ] Handlers: endpoints REST con OpenAPI tags
- [ ] Validación: cargo check + clippy

### Fase B — Frontend constructor
- [ ] Instalar `@dnd-kit/core` + `@dnd-kit/utilities`
- [ ] Componente `PlanoSala.tsx`: canvas con zonas, mesas arrastrables
- [ ] Componente `ConfigMesa.tsx`: modal/panel de configuración de mesa
- [ ] Componente `GestorZonas.tsx`: tabs para zonas, crear/editar/eliminar
- [ ] Componente `CombinacionesMesas.tsx`: gestión de combinaciones
- [ ] CSS responsive
- [ ] Rutas y sidebar

### Fase C — Integración vista día
- [ ] Mostrar plano en ListaReservas con mesas coloreadas (ocupada/libre)
- [ ] Hover → detalle reserva, click → abrir reserva
- [ ] Export/import JSON del plano

## Diseño DB

```sql
zonas_sala: id, user_id, nombre, orden, ancho, alto, created_at, updated_at
mesas: id, zona_id, numero, pos_x, pos_y, ancho, alto, forma, min_personas, max_personas, activa, created_at, updated_at
  UNIQUE(zona_id, numero)
combinaciones_mesas: id, user_id, nombre, max_personas, min_personas, created_at
combinacion_mesa_items: combinacion_id, mesa_id (PK compuesta)
reservas: ADD COLUMN mesa_id UUID REFERENCES mesas(id)
```

## Endpoints planificados

- GET /api/plano-sala → plano completo (zonas + mesas + combinaciones)
- POST /api/plano-sala/zonas → crear zona
- PATCH /api/plano-sala/zonas/{id} → editar zona
- DELETE /api/plano-sala/zonas/{id} → eliminar zona
- POST /api/plano-sala/mesas → crear mesa
- PATCH /api/plano-sala/mesas/{id} → editar mesa (incluye posición)
- DELETE /api/plano-sala/mesas/{id} → eliminar mesa
- PATCH /api/plano-sala/mesas/posiciones → batch update posiciones (drag-and-drop)
- POST /api/plano-sala/combinaciones → crear combinación
- DELETE /api/plano-sala/combinaciones/{id} → eliminar combinación
- GET /api/plano-sala/export → JSON exportable
- POST /api/plano-sala/import → importar JSON

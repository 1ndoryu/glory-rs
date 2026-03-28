# Plan — Marketing Fase 4a: Campañas Manuales (263A-23)

## Referencia
- `cliente/Data III/mensajes.md` — requisitos textuales del cliente
- `cliente/Data III/WhatsApp Image...` — screenshot Cover Manager SMS
- `cliente/Data III/WhatsApp Video...` — video explicativo Cover Manager

## Alcance
CRUD completo de campañas de marketing multi-canal (SMS, email, WhatsApp).
Segmentación de clientes por actividad (última visita).
Contador de caracteres SMS. Opción de baja. Preview de destinatarios.
Envío real: stub (logs) — las APIs externas (SMS gateway, WhatsApp Business API) se configuran después.

## Arquitectura

### Tablas
1. `campanas` — Campaign master
   - id UUID PK, user_id FK, nombre, descripcion_interna, cuerpo_mensaje TEXT
   - canales TEXT[] (array de strings: 'sms', 'email', 'whatsapp')
   - segmento VARCHAR(50) (habitual, sin_1m, sin_3m, sin_6m, sin_9m, sin_1a, sin_mas_1a, todos)
   - incluir_baja BOOLEAN (opt-out en SMS)
   - telefono_baja VARCHAR(30) (teléfono/enlace para darse de baja)
   - estado VARCHAR(20) (borrador, enviada, cancelada)
   - total_destinatarios INT, total_enviados INT, total_fallidos INT
   - created_at, updated_at TIMESTAMPTZ

2. `campana_destinatarios` — Recipients por campaña
   - id UUID PK, campana_id FK, cliente_id FK
   - canal VARCHAR(20) ('sms', 'email', 'whatsapp')
   - estado VARCHAR(20) (pendiente, enviado, fallido)
   - enviado_at TIMESTAMPTZ nullable

### Segmentación (query SQL)
La segmentación se basa en la última reserva completada/confirmada de cada cliente.
- `habitual`: última reserva en los últimos 30 días
- `sin_1m`: última reserva hace 30-90 días
- `sin_3m`: última reserva hace 90-180 días
- `sin_6m`: última reserva hace 180-270 días
- `sin_9m`: última reserva hace 270-365 días
- `sin_1a`: última reserva hace 365-730 días
- `sin_mas_1a`: última reserva hace >730 días
- `todos`: todos los clientes

Filtro adicional: solo clientes con consentimiento según canal seleccionado.

### Endpoints
- POST /api/campanas — Crear
- GET /api/campanas — Listar (paginado)
- GET /api/campanas/:id — Detalle
- PUT /api/campanas/:id — Actualizar
- DELETE /api/campanas/:id — Eliminar
- GET /api/campanas/segmentos/preview?segmento=X — Preview conteo
- POST /api/campanas/:id/enviar — Enviar (genera destinatarios + stub envío)

### Frontend
- Ruta /marketing/campanas — ListaCampanas
- Ruta /marketing/campanas/nueva — FormularioCampana
- Sidebar: sección Marketing con icono Megaphone

## Fases de implementación
1. Migración SQL + modelos Rust
2. Repositorio (CRUD + segmentación query)
3. Servicio (lógica de negocio + envío stub)
4. Handlers + OpenAPI + rutas
5. Frontend: lista + formulario + hooks
6. Seed con campañas de demo
7. Validar, testear, commit

## Estado
- [ ] Fase 1: Migración + modelos
- [ ] Fase 2: Repositorio
- [ ] Fase 3: Servicio
- [ ] Fase 4: Handlers
- [ ] Fase 5: Frontend
- [ ] Fase 6: Seed
- [ ] Fase 7: Validar + commit

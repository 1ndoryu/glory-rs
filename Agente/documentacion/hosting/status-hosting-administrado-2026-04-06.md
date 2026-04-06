# Status: Hosting Administrado — Nakomi Studio

> **Fecha:** 2026-04-06
> **Tarea:** 064A-11

## Resumen ejecutivo

El servicio de hosting administrado tiene backend y panel admin funcionales, pero la página pública sigue mostrando "Próximamente". Faltan integraciones externas (VPS2, DNS, Google Drive OAuth) y la página de venta pública.

---

## Estado por capa

### ✅ Backend (100%)

- **Repositorio:** `src/repositories/hosting.rs`
- **Handlers:** `src/handlers/hosting.rs`
- **5 endpoints REST:**
    - `GET /api/hosting/subscriptions` — listar todas (admin) o propias (cliente)
    - `POST /api/hosting/subscriptions` — crear suscripción
    - `GET /api/hosting/subscriptions/{id}` — detalle
    - `PATCH /api/hosting/subscriptions/{id}/status` — actualizar status (admin)
    - `GET /api/hosting/subscriptions/{id}/events` — historial de eventos
- **BD:** Tablas `hosting_subscriptions` y `hosting_events` creadas en migraciones
- **Planes definidos en código:**

| Plan      | Precio USD/mes  | Storage |
| --------- | --------------- | ------- |
| básico    | $15             | 5 GB    |
| pro       | $35             | 20 GB   |
| ecommerce | $60             | 50 GB   |
| custom    | $0 (cotización) | 100 GB  |

### ✅ Panel Admin (100%)

- **Componente:** `frontend/src/components/panel/SeccionHosting.tsx`
- **Funcionalidad:** CRUD de suscripciones, cambio de status, eventos, modal de creación
- **API Client:** `frontend/src/api/hosting.ts`

### 🟡 Página pública (0%)

- **Estado actual:** `SolucionPlaceholderIsland.tsx` muestra "En construcción — Próximamente"
- **Ruta:** `/soluciones/hosting`
- **Planes en planes.ts:** NO existen como producto independiente. Solo "hosting incluido" en planes de diseño web (3-6 meses).

### 🟡 Coolify Manager RS (no integrado)

- Plan existe en `Agente/planes/plan-hosting-coolify-2026-04-04.md`
- 5 fases planificadas, fases 3-4 completadas, fases 1-2-5 bloqueadas

---

## Lo que falta para tenerlo listo

### Prioridad Alta (necesario para lanzar)

1. **Página pública de hosting** — Reemplazar `SolucionPlaceholderIsland` por una página real con:
    - Hero con propuesta de valor
    - Componente de planes (reusar `SeccionPlanesServicio` con datos de hosting)
    - FAQ
    - CTA que abra chat o lleve al checkout
2. **Datos de planes hosting en planes.ts** — Crear entradas para básico, pro, ecommerce, custom con features, precios y CTA
3. **Checkout/Stripe para hosting** — Flujo de pago para suscripciones mensuales (la infra de Stripe ya existe para otros servicios)

### Prioridad Media (necesario para operación real)

4. **Provisioning automático** — Integrar Coolify Manager RS para crear sitios automáticamente al activar suscripción
5. **Health checks automáticos** — Monitoreo periódico de sitios de clientes
6. **Notificaciones** — Email/notificación cuando el status cambia (pending → active, suspended, etc.)

### Prioridad Baja (mejora futura, bloqueado por infraestructura)

7. **VPS2 standby** — Servidor de respaldo para failover (Fase 1 del plan)
8. **DNS propio (Contabo)** — Gestión de dominios de clientes
9. **Google Drive OAuth** — Backups automáticos a Drive del cliente
10. **Onboarding automatizado** — Templates por tipo de sitio (WordPress, static, Node, etc.)

---

## Archivos clave

| Archivo                                              | Propósito                               |
| ---------------------------------------------------- | --------------------------------------- |
| `src/repositories/hosting.rs`                        | Queries BD para suscripciones y eventos |
| `src/handlers/hosting.rs`                            | Endpoints REST                          |
| `frontend/src/components/panel/SeccionHosting.tsx`   | Panel admin                             |
| `frontend/src/api/hosting.ts`                        | Cliente API frontend                    |
| `frontend/src/islands/SolucionPlaceholderIsland.tsx` | Placeholder actual (a reemplazar)       |
| `frontend/src/data/planes/planes.ts`                 | Datos de planes (falta agregar hosting) |
| `Agente/planes/plan-hosting-coolify-2026-04-04.md`   | Plan maestro de infraestructura         |

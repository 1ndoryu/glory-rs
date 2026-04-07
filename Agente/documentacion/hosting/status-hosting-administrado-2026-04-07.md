# Status: Hosting Administrado — Nakomi Studio

> **Fecha:** 2026-04-07 (actualizado desde 2026-04-06)
> **Tareas relacionadas:** 064A-11 (backend+admin), 064A-32 (página pública+panel cliente), 064A-51 (seed data), 064A-64 (i18n 3 idiomas)

## Resumen ejecutivo

El servicio de hosting administrado tiene backend, panel admin, panel cliente y página pública funcionales. Traducido a 3 idiomas (es/en/ja). Datos de prueba disponibles via SeedService (3 suscripciones test). Faltan integraciones externas para operación real (VPS2 con Coolify, DNS, Google Drive OAuth) y checkout Stripe para hosting.

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
- **Seed data (064A-51):** 3 suscripciones test via `SeedService::create_seed_hosting()`:
    - Básico/active — dominio: mitienda-test.com
    - Pro/provisioning — dominio: app-demo.nakomi.dev
    - E-commerce/suspended — sin dominio
    - Cada suscripción con evento `created` + evento de cambio de status con JSON details
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

### ✅ Panel Cliente (100%) — Nuevo (064A-32)

- **Componente:** `frontend/src/components/panel/SeccionHosting.tsx` (role-aware)
- **Funcionalidad:** Vista de solo lectura de suscripciones propias del cliente
- **Diferencia con admin:** Oculta crear suscripción y cambio de status

### ✅ Página pública (100%) — Nuevo (064A-32 + 064A-64)

- **Componente:** `frontend/src/islands/SolucionHostingIsland.tsx`
- **Estilos:** `frontend/src/islands/SolucionHostingIsland.css`
- **Ruta:** `/soluciones/hosting`
- **Contenido:**
    - Hero con propuesta de valor
    - Grid de 6 features (uptime, soporte, SSL, backups, rendimiento, migración)
    - Grid de 4 planes con precios y características
    - CTA que abre chat (provisioning automático no disponible aún)
- **Datos:** `frontend/src/data/planes/planes-hosting.ts` — 4 planes alineados con backend
- **i18n (064A-64):** Traducido a es/en/ja via `contentTranslations.ts` + sección `hosting_page` en locale JSONs

### 🟡 Coolify Manager RS (no integrado)

- Plan existe en `Agente/planes/plan-hosting-coolify-2026-04-04.md`
- 5 fases planificadas, fases 3-4 completadas, fases 1-2-5 bloqueadas
- Ver también: `Agente/planes/plan-pendientes-consolidado-2026-04-07.md` (sección 4)

---

## Lo que falta para tenerlo listo

### Prioridad Alta (necesario para lanzar comercialmente)

1. ~~**Página pública de hosting**~~ ✅ Completado (064A-32)
2. ~~**Datos de planes hosting**~~ ✅ Completado (064A-32, `planes-hosting.ts`)
3. **Checkout/Stripe para hosting** — Flujo de pago para suscripciones mensuales (la infra de Stripe ya existe para otros servicios, falta adaptarla a recurrencia)

### Prioridad Media (necesario para operación real)

4. **Provisioning automático** — Integrar Coolify Manager RS para crear sitios automáticamente al activar suscripción
5. **Health checks automáticos** — Monitoreo periódico de sitios de clientes
6. **Notificaciones** — Email/notificación cuando el status cambia (pending → active, suspended, etc.)

### Prioridad Baja (mejora futura, bloqueado por infraestructura)

7. **VPS2 standby** — Servidor de respaldo para failover (Fase 1 del plan)
8. **DNS propio** — Gestión de dominios de clientes (ver `Agente/planes/plan-dominios-2026-04-07.md`)
9. **Google Drive OAuth** — Backups automáticos a Drive del cliente
10. **Onboarding automatizado** — Templates por tipo de sitio (WordPress, static, Node, etc.)

---

## Archivos clave

| Archivo                                              | Propósito                                    |
| ---------------------------------------------------- | -------------------------------------------- |
| `src/repositories/hosting.rs`                        | Queries BD para suscripciones y eventos      |
| `src/handlers/hosting.rs`                            | Endpoints REST                               |
| `src/services/seed.rs`                               | Datos de prueba (3 suscripciones hosting)    |
| `frontend/src/components/panel/SeccionHosting.tsx`   | Panel admin + cliente (role-aware)           |
| `frontend/src/api/hosting.ts`                        | Cliente API frontend                         |
| `frontend/src/islands/SolucionHostingIsland.tsx`     | Página pública de hosting                    |
| `frontend/src/islands/SolucionHostingIsland.css`     | Estilos de la página pública                 |
| `frontend/src/data/planes/planes-hosting.ts`         | Datos de 4 planes de hosting                 |
| `frontend/src/i18n/contentTranslations.ts`           | Traducciones de contenido (incluye hosting)  |
| `Agente/planes/plan-hosting-coolify-2026-04-04.md`   | Plan maestro de infraestructura              |
| `Agente/planes/plan-dominios-2026-04-07.md`          | Plan de servicio de dominios (complementario)|

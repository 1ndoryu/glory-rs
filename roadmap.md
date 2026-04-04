Objetivo: Nakomi Studio — sitio web de agencia creativa. Migrado de WordPress a Rust (Axum) + React SPA.
Rama: glory-rust-nakomi
Roadmap de tareas del proyecto: App/roadmap.md

## Estado: 044A-1 completada (migración SPA)

## Stack implementado

| Capa                 | Herramienta                                    |
| -------------------- | ---------------------------------------------- |
| Framework web        | Axum 0.7                                       |
| OpenAPI              | utoipa 4 + utoipa-swagger-ui 7                 |
| Serialización        | serde                                          |
| Base de datos        | SQLx 0.8 (PostgreSQL)                          |
| Migraciones          | SQLx migrate                                   |
| Validación           | validator 0.18                                 |
| Variables de entorno | dotenvy                                        |
| Logging              | tracing + tracing-subscriber                   |
| Errores              | thiserror 2                                    |
| Auth                 | jsonwebtoken + argon2                          |
| CORS                 | tower-http                                     |
| Linter               | clippy (deny all + warn pedantic)              |
| Frontend             | React 18 + TypeScript + Vite                   |
| State                | React Query + Zustand                          |
| Codegen              | Orval 8 (reemplaza openapi-typescript-codegen) |

## Pendientes



# Nakomi Studio — Roadmap

## Contexto
Proyecto migrado de WordPress a Rust (Axum) + React SPA. El frontend React de App/React/ se integra en frontend/src/. El backend PHP se reemplaza por el template Rust.

---

## Pendientes (por prioridad — lo más difícil primero)

> Plan maestro: `Agente/planes/plan-marketplace-2026-04-04.md` (11 fases)
> Plan de chat: `Agente/planes/plan-live-chat-2026-04-04.md` (5 fases)
> Plan de hosting: `Agente/planes/plan-hosting-coolify-2026-04-04.md` (5 fases)

### Marketplace — Fase 0: Modelo de datos y migración
- Nueva migración SQL con todas las tablas del marketplace (orders, payments, phases, refunds, reviews, delegations, notifications, activity_log)
- Extender users con role, active_role, email_verified, status
- Crear user_profiles y employee_profiles
- Modelos Rust y repositorios
- Seed admin@admin.com como role='admin'

### Marketplace — Fase 1: Roles y auth extendido
- JWT claims con role + active_role
- Middleware RequireRole guards
- Endpoint switch-role para admin
- Frontend: authStore con role, botón switch en esquina inferior izquierda
- Panel dinámico por rol (tabs diferentes por admin/empleado/cliente)
- Redirección / → /panel si logueado

### Marketplace — Fase 2: CRUD de órdenes
- Endpoints de órdenes: crear, listar, detalle, cancelar
- Endpoints de fases: listar, entregar, aprobar, revisión
- Máquina de estados de orden
- Frontend: flujo de contratación desde /servicios/:slug
- Frontend: panel cliente → "Mis Proyectos" con progreso visual

### Marketplace — Fase 3: Pagos con Stripe
- Stripe PaymentIntent con capture manual (escrow)
- Webhook handler con verificación de firma
- 3 modos: completo (20% desc), 50/50 (10% desc), fases
- Frontend: checkout Stripe Elements
- Historial de pagos

### Marketplace — Fase 4: Asignación y delegación
- Panel admin: órdenes sin asignar
- Empleado: tomar orden
- Auto-asignación 24h background task
- Delegación y solicitud de ayuda

### Marketplace — Fase 5: Chat integrado con órdenes
- Vincular chat_sessions con orders
- Chat específico por orden
- Chat pre-venta (IA)
- Contexto de orden en IA

### Marketplace — Fase 6: Entregables y revisiones
- Upload de archivos
- Entrega por fase
- Aprobación / revisión con límite

### Marketplace — Fase 7: Reembolsos
- Solicitud con razón
- Admin aprueba/rechaza
- Stripe refund automático

### Marketplace — Fase 8: Reviews
- Rating 1-5 + comentario
- Respuesta del empleado

### Marketplace — Fase 9: Notificaciones
- BD + endpoints + WebSocket real-time
- Email para críticas

### Marketplace — Fase 10: Dashboard admin
- Revenue, métricas, alertas
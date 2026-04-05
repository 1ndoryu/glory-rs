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
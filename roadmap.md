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

## Otras tareas

- Termina todos los planes.

- Todos los repositories estan usando query_as sin macro, sin usar sqlx::query_as! Necesito que la verificación sea en compilacion en todos lados!!.

- Con el plan de chat, revisa que pude haberme olvidado, tambien el plan de servicio de hosting que me olvide, queremos ofrecer hosting y que los clientes pueda comprar hosting y gestionarlos. Y trabaja en todo eso que posiblemente olvide. 

- En el panel de admin hace falta algo para ver los usuarios registrados, con buscador, y filtro, con capacidad de banear, cambiar de cliente a empleado, etc.

- No se si ya existe pero hace falta un toast de notificaciones. 

- Respecto al diseño de ordenCard, quita la numeración, el nombre del plan, la barra de progreso y fases, falta imagen del servicio.

- Revisa que en todo el codebase se usen los componentes Input y Textarea en vez de los nativos.
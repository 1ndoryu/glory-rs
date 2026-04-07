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

> Plan maestro: `Agente/planes/plan-marketplace-2026-04-04.md` (11 fases) — ✅ completado
> Plan de chat: `Agente/planes/plan-live-chat-2026-04-04.md` (5 fases) — ✅ completado (streaming IA = mejora futura)
> Plan de hosting: `Agente/planes/plan-hosting-coolify-2026-04-04.md` (5 fases) — Fases 3-4 ✅, Fases 1-2-5 bloqueadas por infraestructura externa (VPS2, DNS, Google Drive OAuth)
> Status hosting: `Agente/documentacion/hosting/status-hosting-administrado-2026-04-07.md`

### Bugs y UX del usuario

- 074A-2: Datos de prueba completos para vista cliente y empleado (falta seed data visible).
- 074A-3: Verificar/corregir datos de prueba de hosting (064A-51 los creó, puede que no se vean).
- 074A-15: hostingFormCrear tiene un padding innecesario.
- 074A-16: Padding innecesario en usuariosContenedor y proyectosContenedor.
- 074A-17: Reembolso sigue diciendo "Request failed with status code 404".
- 074A-18: Quitar .chatWidgetBubble del panel (mala idea haberlo puesto ahí).
- 074A-19: chatInputArea se ve feo — usar como referencia el input+botón de chatWidgetBubble.
- 074A-20: Estilos de títulos inconsistentes en chat del panel ("Conversaciones", "Chat general", "Info de visitante") — todos deben ser: font-size: var(--text-sm), font-weight: 600, color: var(--brand-black), flex: 1.
- 074A-21: En "Info del visitante" la forma en la que se ordena la información se ve fatal.
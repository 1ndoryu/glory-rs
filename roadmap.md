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

### CMS Admin (plan: plan-cms-admin-2026-04-07.md)

- 074A-6: CMS Infra — Upload endpoint backend + componentes base (UploadImage, SlugInput)
- 074A-7: CMS Infra — RichTextEditor (tiptap) + ContentSection/ContentEditor base
- 074A-8: CMS Servicios — Backend (migración + CRUD endpoints admin)
- 074A-9: CMS Servicios — Frontend (SeccionContenido + EditorServicio)
- 074A-10: CMS Blog — Backend (tabla blog_posts + CRUD)
- 074A-11: CMS Blog — Frontend (editor admin + páginas públicas)
- 074A-12: CMS Proyectos — Full stack (migrate showcase.ts → BD)
- 074A-13: CMS Equipo — Full stack (migrate miembros.ts → BD)

### SEO y mejoras (plan: plan-seo-completo-2026-04-04.md)

- 074A-14: SEO Fase 2 — Performance (lazy loading, image optimization, Core Web Vitals)

## 

(ASEGURATE DE BORRAR TAREAS QUE DE VERDAD HAYA HECHO)
- El headerPanelSubmenu se ve diferente a como es en realidad el componente 
- chatMensajes necesita un ancho maximo que haga que no se salga de la pantalla para que funcione el scroll interno. 
- En la pagina de reembolso Request failed with status code 404
- ya se ven datos de prueba en la vista de empleado pero ajam
orders.ts:85  GET http://localhost:3000/api/orders/2e504954-1e1d-49b6-93a9-b2e85ae1092b 403 (Forbidden)
orders.ts:85  GET http://localhost:3000/api/orders/2e504954-1e1d-49b6-93a9-b2e85ae1092b 403 (Forbidden)
igualmente con la vista de cliente http://localhost:3000/api/orders/2e504954-1e1d-49b6-93a9-b2e85ae1092b 403 (Forbidden)
- Borra .entregablesUpload, los entregables tienen que marcarse de otra forma, todo debe ser por el chat, el boton de entrega se hace sin enviar nada, el boton debe ser boton primario, no de exito.
- Cuando abro el chat el scroll se va hacia abajo (dentro del os detalles de un pedido), en vez del scroll interno
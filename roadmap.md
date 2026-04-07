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
- 074A-23: No se puede cambiar el nombre de usuario después de registrarse — verificar si existe y si no, agregar.

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

- EL boton de salir sigue deslogeando en vez de ir al home y no veo ningún submenu a dar click a la foto de perfil.
- Al cambiar el tamaño de la pantalla algo hace que los bordes de los badge de abajo se recorten un poco en carruselItem.
- Sigue diciendo payments.ts:37  POST http://localhost:3000/api/orders/2e504954-1e1d-49b6-93a9-b2e85ae1092b/pay 500 (Internal Server Error), dice "Ocurrió un error interno" sin explicar por qué. 
- El boton de enviar en chatInputArea dejalo sin background. 
- Busca todos los border-color: que esten usando borde negro y cambialo a     border-color: var(--border-default);
- Hay chat que aparecen sin mensajes en el panel, no tiene sentido. 
- a .sidebarItem agrega margin-left: -1rem;
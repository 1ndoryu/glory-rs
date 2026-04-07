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

- 074A-12: CMS Proyectos — Full stack (migrate showcase.ts → BD)
- 074A-13: CMS Equipo — Full stack (migrate miembros.ts → BD)

### SEO y mejoras (plan: plan-seo-completo-2026-04-04.md)

- 074A-14: SEO Fase 2 — Performance (lazy loading, image optimization, Core Web Vitals)

### Bugs / UX reportados por usuario

- EL SELECT DE proyectosFiltros SIGUE SIENDO EL INCORRECTO!! ESTO NO LO DETECTA GLORY SENTINEL??????? Ese select no es valido, no se debe usar ese select!! Tiene que ser el que esta en usuariosFiltros. Borra ese tipo de select en proyectosFiltros para que no se vuelva a usar.

- ANTERIORMENTE DIJE "Al cambiar al usuario cliente no veo un proyecto que este en proceso (no en proceso de pago), tiene que haber uno que este entre el admin y cliente, y otro entre el cliente y empleado." SIGO SIGO SIN VER PEDIDOS ACTIVOS AL CAMBIAR AL USUARIO EMPLEADO!!!!!!!!!!! ESO NECESITO DATOS DE PRUEBAS REALES; UN PEDIDO AL MENOS ACITVO QUE ESTE PARA EL USUARIO EMPLEADO AL QUE ACCESO DESDE EL AMDIN; NECESITO 2 O 3 PEDIDOS ACTIVOS UNO ENTRE EL ADMIN Y EL CLIENTE; UNO ENTRE EL EMPLEADO Y EL CLIENTE. ELIMINA TODOS LOS DATOS DE PRUEBA LOS QUE HAY NO ME SIRVEN.

- La pagina de disponibles para el usuario empleado se ve mal, hay que reahacerla desde cero, no la entiendo. Igual la pagina delegaciones, porque carajo las letras son blancas, esto es incoherente. 
- chatMensajes SIGUE SIN UN ANCHO mAXIMO QUE EVITE QUE chatContenedor se salga de l pantalla.
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
> Status hosting: `Agente/documentacion/hosting/status-hosting-administrado-2026-04-06.md`

### Funcionalidad grande

- 064A-32: Hosting + dominio panel cliente: página funcional, plan para compra de dominios.

### Funcionalidad media

- 064A-45: Hosting doc pendientes (status-hosting-administrado-2026-04-06.md).
- 064A-44: Revisar todos los planes, mover completados, crear plan nuevo para pendientes.
- 064A-51: Datos de prueba para hosting panel.


### Tooling

- 064A-43: VarSense shadow detection rule.

###

- 064A-59: Stripe: no preguntar correo en modal de checkout (usar el del usuario logueado), revisar flujo completo de pago, crear plan detallado de lo que falta para que Stripe funcione end-to-end.
- 064A-60: Selección de modo de pago (completo, por fases, 50/50) — ¿dónde se elige?
- 064A-61: Menú hamburguesa mobile: quitar accionCabecera, modal centrado estilo chatWidgetPanel, soporte submenús, botón para volver al menú.
- 064A-62: Tab de configuración en panel: opciones para recrear/borrar datos de prueba.
- 064A-63: Plan detallado para edición de contenidos admin: servicios, blog, proyectos editables desde el front con modal + tabs laterales.
- 064A-64: Traducción de categorías, servicios, planes, descripciones de equipo, soluciones a los 3 idiomas (enfoque automatizado).

### 

- 064A-72: No se puede ver la informacion del usuario para distinguirlo al menso, debera haber un panel lateral en el chat disponible para ver informacion del usuario como su ip, ubicación, dispositivo y toda la información posible, y para agregar notas, cambiar el nombre, el nombre deberia actualizarse automaticamente cuando la IA se lo pida. 
- 064A-73: Auditoría de seguridad detallada — primero plan con todo lo que se va a revisar, luego ir por etapa.
- 064A-74: Las imágenes de las tarjetas en los pedidos tienen que ser 1:1 proporción.
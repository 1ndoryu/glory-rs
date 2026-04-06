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

- 064A-31: Chat dentro de pedidos: botón que abre chat con el proveedor del servicio para comunicación cliente↔empleado.
- 064A-32: Hosting + dominio panel cliente: página funcional, plan para compra de dominios.

### Funcionalidad media

- 064A-35: Vista cliente/empleado sin pedidos visibles.
- 064A-36: Página reembolso 404.
- 064A-45: Hosting doc pendientes (status-hosting-administrado-2026-04-06.md).
- 064A-44: Revisar todos los planes, mover completados, crear plan nuevo para pendientes.
- 064A-50: Órdenes ordenadas por status, canceladas en tab de historial separada.
- 064A-51: Datos de prueba para hosting panel.


### Tooling

- 064A-43: VarSense shadow detection rule.

###

- El icono de minimizar en el chat no se ve.
- En el chat hay un espacio vacío abajo del input de chat
- El input del chat debería tener 
    background: unset;
    border-radius: 104px;
En realidad ningún input debe tener background
El input debe estar 100% expandido haciendo que el boton de enviar mensaje este encima. 
- Quita el borde top de chatWidgetInputArea.
- Quita el del chat la logica de perdir el nombre y que abra el chat directamente, el nombre se lo pedira el agente al usuario. 
- Agrega un poquito mas de padding a los lados en .chatWidgetBubble
- Sigo con este fucking problema una y otra vez y lo unico que haces borrar la tarea sin solucionarlo carajo "064A-21: Repito, esto lo vuelvo a repetir, en los datos de prueba hay una inconsistencia, porque hay ordenes con pago unico pendiente de pagar si un pago unico no debe generar pendiente de pagos pues se paga una sola vez para iniciar el pedido, un fucking pedido de pago unico no puede iniciarse sin ya estar pagado, igualmente la primera fase siempre tiene que estar paga" SIGO VIENDO LOS MISMOS DATOS MAL Y SIGO VIENDO los titulos con numeracion." Es la fucking 4 vez que lo comento
- La imagen por defecto de cuando se crea un usuario no me gusta, elige otra.
- No debería preguntar el correo en el modal para continuar con el pedido, el correo tiene que el que se use en la compra de stripe, si la contraseña se puede pedir en el formulario de stripe, mejor, sino, debe aparecer para elegirse en el panel al regresar. Tambien hay que revisar que falta para que funcione con stripe porque hice una prueba y da 404, haz un plan sobre esto.
- parece que panelUsuario tiene un padding botton innecesario.
- Que fatal que hayas borrado alguans tareas, habia dicho que los select en usuariosFiltros se ven fatal, no usan el color de borde correcto, no tienen padding a los lados, el texto n ose ve centrado. Ningun menu tiene que usar sombras.
- Noto que cuando se elige un servicio, no se esta eligiendo si pagar completo, por fases o 50/50. ¿Donde se elige esto?
- sidebarUsuario es redundante que aparezca sabiendo que ya aparezce la foto perfil en el nav del panel, la foto de perfil debería ser un poquito mas pequeña, 2 px mas pequeña.
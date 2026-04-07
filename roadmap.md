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

- 064A-45: Hosting doc pendientes (status-hosting-administrado-2026-04-06.md).
- 064A-44: Revisar todos los planes, mover completados, crear plan nuevo para pendientes.
- 064A-50: Órdenes ordenadas por status, canceladas en tab de historial separada.
- 064A-51: Datos de prueba para hosting panel.


### Tooling

- 064A-43: VarSense shadow detection rule.

###

- No debería preguntar el correo en el modal para continuar con el pedido, el correo tiene que el que se use en la compra de stripe, si la contraseña se puede pedir en el formulario de stripe, mejor, sino, debe aparecer para elegirse en el panel al regresar. Tambien hay que revisar que falta para que funcione con stripe porque hice una prueba y da 404, haz un plan sobre esto.
- Noto que cuando se elige un servicio, no se esta eligiendo si pagar completo, por fases o 50/50. ¿Donde se elige esto?
- En movil hay que quitar accionCabecera y dejar solo el boton de hamburgueza, tambien ajustar lo que se ve al abrir el boton de hamburgueza, tiene que ser un modal con el estilo de .chatWidgetPanel pero centrado, con los botones de menu en el centrol tiene que soportar los botones con menu y mostrar las opciones internas en vez de navegar directamente, agregar un boton para ver el menu porque tambien se puede navegar directamente una pagina con submenu.
- Agrega una tab en el panel de configuracion. Alli habra opciones para recrear los datos de prueba, borrarlos. 
- Para el admin, todos los contenidos deben ser editables en el front, esto es una tarea gigantezca, requiere un plan detallado. Nos podemos enfocar en los servicios, habrá un boton en la esquina para editar, abrira un modal con tab lateral para modificar cada cosa, titulo, descripción, planes, precios, fases, etc, todo lo que sea modificable, enlaces. Tambien para los blog, y proyectos. 

- Las categorías no estan traducidas, deben traducirse a los 3 idiomas. Los contenidos de los servicios, planes, tambien, no creo que sea bueno reescribir todo en los 3 idiomas lo ideal es una forma automatica y segura porque es contenido que va modificarse. Las descripciones de los miembros de equipo no estan traducidas.
- Los servicios en soluciones tampoco estan traducidos.
- porque pagar la orden de prueba da payments.ts:36  POST http://localhost:3000/orders/78b71f93-c594-4636-82b4-1c366e6f4511/pay 404 (Not Found), y el boton de continuar pago es excesivamente grande, no debería. 
- No la tarea 
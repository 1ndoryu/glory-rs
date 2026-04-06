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

- 054A-19: Arregla todo lo que indica Code Sentinel - Reporte de Workspace, y los falsos positivos arreglalos en la extension
- 054A-20: El modal de chat de ia no tiene sentido que sea el fondo de color negro
- 054A-21: Todos los servicios tienen que tener 3 planes: Basico, Medio, y Avanzado (a todos les falta 1 plan)
- 064A-4: El globo de chat no se ve redondo y esta vacío, y el bton de botonBase botonTexto botonMediano chatWidgetSendBtn tambien se ve vacío, estirado, con un color que no va y horrible.
- 064A-5: La pagina de contacto ya no tiene sentido, todos los botones de contacto deben abrir el chat. 
- 064A-7: Las imagenes de galería de los proyectos tienen que ser mas grande. 
- 064A-8: proyectoHero, alli se podría colocar detalles tecnicos, tecnología, enlaces (con iconos de donde este disponible, github, web, etc). 
- 064A-11: ¿Cual es el status de Hosting administrado? Sigue diciendo "Próximamente -Estamos trabajando en esta solución. Pronto tendrás toda la información que necesitas." Crea un md con todo lo que falta para tenerlo listo. 
- 064A-13: los tarjetaArticulo tarjetaArticuloDestacado se ven mal, deberían verse como las tarjetas ende blog en el inicio.
- 064A-15: En la vista de cliente no tengo proyectos para ver ni en la vista de empleado, tienen que ser cosas legitimas para probar la funcionalidad real. 
- 064A-20: El boton ordenDetalleTopbar no me parece un componente, se ve mal, no esta usando icono svg parece. 
- 064A-21: Repito, esto lo vuelvo a repetir, en los datos de prueba hay una inconsistencia, porque hay ordenes con pago unico pendiente de pagar si un pago unico no debe generar pendiente de pagos pues se paga una sola vez para iniciar el pedido.
- 064A-22: Lo de ordenDetalleNotas no se para que sirve pero no le veo mucho sentido. 
- 064A-23: Los titulos no deben estar enumerados si es que se hizo automaticamente, no debe ordenDetalleTitulo
- 064A-24: La info en ordenDetalleMetaRow toda debería estar separada por badges.

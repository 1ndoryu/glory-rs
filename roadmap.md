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

- No arreglaste el problema de que los mensajes en el y la parte de chat tienen fondo negro y se ven mal.
- TE DIJE QUE BORRAR LA PAGINA DE EMPLEADO!!! Y TAMBIEN BORRA LA PAGINA DE SERVICIO (DENTRO DE PANEL) ESTAS PAGINAS ESTAN MAL.
- El boton de 3 puntos es mas grande que el boton de pagar y se ve mal ambos de diferentes tamaños en ordenDetalleAcciones. 
- A pesar que te dije "064A-21: Repito, esto lo vuelvo a repetir, en los datos de prueba hay una inconsistencia, porque hay ordenes con pago unico pendiente de pagar si un pago unico no debe generar pendiente de pagos pues se paga una sola vez para iniciar el pedido." SIGO VIENDO LOS MISMOS DATOS MAL Y SIGO VIENDO los titulos con numeracion. 
- Los detalles de los proyecto se notan que carecen de demasiadas cosas, necesitamos un cuadro con los detalles del pedido, freelancer, servicio, numero de orden, empezado (fecha y hora), precio, el boton de 3 puntos, falta la opción de reportar, para los freelancer falta la opcion de pedir extensión de tiempo del pedido al cliente, y la resolución de problemas paara reportar problemas como en fiverr. Tambien falta un cuadro de historial, para ver el progreso, cuando se inicia el proyecto, entregas, revisiones, hay que asegurarse los clientes cuando reciben una entrega puedan pedir revisiones. 
- En la vista de cliente y empleado sigo sin ver pedidos para poder probar como se ven los pedidos.
- el padding de .modalBaseContenedor debería ser aproximadamente de 30px (usa variables)
- Los select que estan en la parte de usuario (panel) se ven mal, les falta padding a los lados, los bordes son negros, este problema de usar bordes negro es recurrrente, los bordes tienen que ser (--bg-item-active);. 
- Tenemos que agregar la pagina de hosting y dominio en el panel de usuario de los clientes, tienen que ser funcional, haz plan de todo lo que se necesita hacer para que los clientes puedan comprar dominios. 
- El boton de 3 puntos en la tabla de usuario de no debe tener borde
- Los badge de usuariosRolBadge usuariosRolemployee siguen teniendo colores, dije que bagde sin colores, todos de escala gris. No aparecen los nombre de usuario. Probablemente no los tengan porque son datos que se crearon, entonces ponles. 
- Dentro de los pedidos debe haber un boton de chat que habra el chat con el que provee el servicio y el cliente para que el cliente le hable o el empleado le hable al cliente. 
- La pagina de reembolso dice Request failed with status code 404
- El boton de chat acercalo mas a la esquina, que use border: 1px solid var(--bg-item-active); background-color: var(--bg-accent); quita ese icono feo y que diga chat con una foto que deje en frontend\public\assets\random\85a51ba9a4233272662e744b48f97d67.jpg, el fonde al abrir tiene que ser del mismo color. No tiene que haber sombra, las sombras estan prohibidas, varsense tiene que detectar el uso de sombras para que dejes de usar sombras, este tambien es un problema frecuente de que no se como evitar que lo hagas si editando tus intrucciones o alguna regla, chatWidgetHeader es innecesario, ningun modal lleva header ni boton de cerrar, el estandar de diseño minimalista lo prohihibe, tal vez un apartado de diseño minimalista en tu instrucciones te ayude la proxima vez, tambien esta prohibido usar sombras, .inputBase esta usando un borde oscuro en vez de var(--bg-item-active), en este caso el color de los botones debe ser #e8e6e2 (botones secundarios) sin borde, el boton de Iniciar conversacion debe ser secundario, no se que clase boton ese que tiene pero eliminalo, el boton de enviar mensajes tambien debe ser #e9e7e4, se me ocure mejor que el fondo del chat se puede hacer asi

    background-color: rgba(245, 242, 239, 0.8);
    min-height: 440px;
    box-shadow: rgba(78, 50, 23, 0.04) 0px 6px 16px 0px, rgba(0, 0, 0, 0.1) 0px 0px 0px 1px;

y un boton de minizar en la esquina con icono negro y fondo #e8e6e2 redondo, la animacion de maxizar el chat minimizar debe ser suave. 
- Veo que chat no perdura al reiniciar la pestaña, es fatal, y necesito probar ya el chat con groq hay 3 grop api para probar, necesito rotacion de api por cada mensaje y usar el modelo mas inteligente, averigua cual es, revisa todo el proceso de IA ver si esta listo. 

- Trabaja en todo lo que esta pendiente en status-hosting-administrado-2026-04-06.md.
- Revisa todos los planes y que hay pendiente, lo que este listo muevelo, lo que falta, dejalo pendiente en un plan nuevo. 
- en modalBaseContenedor modalCompraContenido quita el precio del texto y dejalo solo en el boton, y hace falta una separacion entre el boton y el texto.
- Te habia dicho que en todos los servicios la imagen que aparece en el grid de servicio debe aparecer tambien dentro del servicio, grande. con proporcion de pantalla de laptop.
- El boton de suscribir tiene borde blanco en el footer, se ve mal, tambien hay que revisar que las suscripciones funcionen y se guarde la informacion para usarlas despuees.
- No veo el boton de cambiar idioma en el footer, antes estaba.
- El bordel chat en el panel no se ve, debería ser var(--bg-item-active)
- El favicon no debe ser una N, sino un punto como el que esta en headerPanelLogo, puede ser con fondo blanco y el punto de color negro.
- Las ordenes deberían ordenarse por status. Las canceladas en otra tab de historial. 
- La altura de panelSidebar no debería cambiar al cambiar de pagina, hay que ajustar para que tome la altura completa siempre.
- Necesito que en el panel de hosting haya datos de prueba para ver como se ven.
- El toast aparece arriba, tiene que se abajo y el fondo siempre debe ser de background-color: var(--bg-accent)
- El boton de pagar fase botonBase botonExito botonPequeno faseBtn el icono y le texto no tiene separación y dije que los badge no deben de ser color y veo faseBadge faseBadge--pending-payment tiene color



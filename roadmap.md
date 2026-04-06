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
- 054A-22: Nada en modalCompraResumen necesita (--font-serif)
- 054A-23: Las cosas en modalCompraResumen deberían estar separadas gap y en columna, no con margenes
- Creo que la mejor forma que en vez pedir que el usuario se registre, simplemente se cree una cuanta apartir del correo con el que se ponga en la compra y al comprar, ya este logeado, luego el usuario puede elegir o cambiar su contraseña. 
- El globo de chat no se ve redondo y esta vacío, y el bton de botonBase botonTexto botonMediano chatWidgetSendBtn tambien se ve vacío, estirado, con un color que no va y horrible.
- La pagina de contacto ya no tiene sentido, todos los botones de contacto deben abrir el chat. 
- Las imagenes de galería de los proyectos tienen que ser mas grande. 
- proyectoHero, alli se podría colocar detalles tecnicos, tecnología, enlaces (con iconos de donde este disponible, github, web, etc). 
- seccionContacto en la pagian de nosotros necesita mas padding botton y tambien en soluciones.
- ¿Cual es el status de Hosting administrado? Sigue diciendo "Próximamente -Estamos trabajando en esta solución. Pronto tendrás toda la información que necesitas." Crea un md con todo lo que falta para tenerlo listo. 
- los tarjetaArticulo tarjetaArticuloDestacado se ven mal, deberían verse como las tarjetas ende blog en el inicio.
- Cuando inicie sesion no me redirigio al panel. 
- En la vista de cliente no tengo proyectos para ver ni en la vista de empleado, tienen que ser cosas legitimas para probar la funcionalidad real. 
- Despues de cambiar la imagen de perfil la imagen no carga 
- Empleados en el panel se ve falta, elimina eso, no es lo que espero, espero una tabla de usuarios, usuariosFiltros se ve falta, sebería los select ser componentes personalizados y usar el componente de menu para mostrar las opciones, glory sentinel debería reportar esto para que no vuelva a suceder, siempre sucede.
- El submenu que se abre de 3 puntos en la lista de usuario se ve dentro de la tabla y genera un scroll, no debe generar sobra ningun menu y algunos botones del submenu se ven con texto centrado, no debería.
- El boton ordenDetalleTopbar no me parece un componente, se ve mal, no esta usando icono svg parece. 
- Repito, esto lo vuelvo a repetir, en los datos de prueba hay una inconsistencia, porque hay ordenes con pago unico pendiente de pagar si un pago unico no debe generar pendiente de pagos pues se paga una sola vez para iniciar el pedido.
- Lo de ordenDetalleNotas no se para que sirve pero no le veo mucho sentido. 
- Los titulos no deben estar enumerados si es que se hizo automaticamente, no debe ordenDetalleTitulo
- La info en ordenDetalleMetaRow toda debería estar separada por badges.
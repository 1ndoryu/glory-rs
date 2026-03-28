# Plataforma de Gestión de Restaurantes — Roadmap

> **URL producción:** http://app-b8s0cks444o0sogo8kg8wcgw.66.94.100.241.sslip.io (Swagger UI: /swagger-ui/)
> **URL directa:** http://66.94.100.241:3001
> **Servidor:** 66.94.100.241 (Coolify, servicio UUID: b8s0cks444o0sogo8kg8wcgw)
> **Deploy:** `scripts/deploy-server.sh` via SSH (requiere DB_PASSWORD y JWT_SECRET como env vars)
> **Repositorio:** 1ndoryu/glory-rs, rama glory-rs-rest

## Stack

| Capa            | Herramienta                          |
| --------------- | ------------------------------------ |
| Framework web   | Axum 0.7                             |
| OpenAPI         | utoipa 4 + utoipa-swagger-ui 7       |
| Base de datos   | SQLx 0.8 (PostgreSQL 18)             |
| Validación      | validator 0.18                       |
| Auth            | jsonwebtoken + argon2 + SHA-256 (API keys) |
| Frontend        | React 18 + TypeScript + Vite + Tailwind v4 + shadcn/ui |
| State           | React Query + Zustand                |
| Codegen         | Orval 8 (tags-split)                 |
| Linter          | clippy (deny all + warn pedantic)    |

## Notas del cliente

- Diseño minimalista, simple, intuitivo.
- Una instancia por propietario/restaurante (no multi-tenant con registro público).
- Si conseguimos más negocios, clonar la plataforma para cada nuevo cliente usando glory-rs como template.
- Parte económica simplificada: solo Gastos, Ventas y Margen (Ventas - Gastos).
- Secciones omitidas/pausadas: Dashboard analítico avanzado, Administración de documentos, Conciliación, Incidencias, Tesorería, Bancos.

## Features pendientes

### Marketing — Fase 4e (futuro)
- Métricas de marketing: medir cuántos clientes vinieron al restaurante después de recibir un mensaje de campaña.

## Tareas pendientes

~~1. Canvas plano de sala 800×600 → 100% ancho, panel lateral derecho, mesa draggable, combo al toolbar, validación duplicados.~~ → 283A-17
~~1.1 Panel config mesa al lado derecho~~ → 283A-17
~~1.2 Crear mesas arrastrando~~ → 283A-17
~~1.3 Combinación arreglada + botón al toolbar~~ → 283A-17
~~1.4 Error 500 por mesa duplicada → ahora devuelve Conflict con toast~~ → 283A-17

~~2. Resumen y General combinados, gráficos en 1 sola fila, padding de gráficos eliminado.~~ → 283A-18

3. ~~En ventas, reservas, clientes, faltan filtros, filtrar por fecha, falta un buscador en tiempo real.~~ → 283A-28

~~4. Distribución por hora y Por turno en grid de 2, Procedencia (canal) abajo en bloque propio.~~ → 283A-18

~~5. Debería poder editarse los gastos y ventas por si se comente un error, creo.~~ → 283A-22

6. ~~En la parte de recordatorio (y creo que esto sucede en el resto de paginas una similar estructura) <p class="text-muted-foreground text-xs mt-0.5 line-clamp-1">Estimado/a {nombre}, le recordamos que tiene una reserva programada para mañana a las {hora}. Si necesita cancelar o modificar, responda a este email.</p> cuando es muy largo hace que la pagina supere el ancho y se ve mal.~~ → 283A-30

7. ~~Cuando un dia tiene reservas en el calendario se ve mal, se ve gris y como el fondo es blanco no constrasta bien.~~ → 283A-30

8. ~~No veo configuraciones para la api de chatbot, donde se genera la api para conectarse, tambien deberia haber un enlace para la documentacion, configuraciones, etc.~~ → 283A-27

~~9. Ya lo habia dicho antes, el modal de Nuevo Gasto debe ser mas grande, un poco.~~ → 283A-19

10. ~~Cumple con # Plan: Glory-RS como Framework Reutilizable.~~ → 283A-32

11. Supongo que lo de  /* [283A-17] Catch unique constraint (zona_id, numero) para devolver
         * Conflict en vez de 500 cuando el número de mesa ya existe en la zona. */ 
Es un error que se puede evitar de nuevo con alguna regla en sentinel, pero si no, ignorar por ahora.

~~12. Las cosas de marketing y campaña como se conectan? o sea, cuales son las configuraciones que hay que hacer para conectarlas? Veo que todo esta, pero ajam presiento que falta un monton de cosas, revisa que falta, que configuraciones hay que hacer, para que todo funcione, esto es general, se requiere revision de todo porque si hay un hueco aca, nada quita que haya en otros lados.~~ → 283A-21 (auditoría en Agente/documentacion/marketing/)

~~13. El cliente no lo pidio pero estaría bien que hayan notificaciones en el panel en tiempo real para cosas como nuevas reservas, o lo que sea que se haga mediante el chat bot, porque ajam si se reciben cosas externamente, pues, deberían haber notifaciones en tiempo real.~~ → 283A-20

14. ~~Los graficos no estan ocupando el ancho completo del bloque en donde estan.~~ → 283A-29

15. ~~Los graficos en la la pestaña de General ponlos en grid de 2 columnas, y agrega otro grafico, no se cual, cualquier cosa que se pueda agregar en un grafico.~~ → 283A-29

~~16. El modal de gastos no cambio de tamaño.~~ → 283A-19

~~17. No, el modal de "nuevo gasto" no cambio, no cambia. No importa que agregues un valor nuevo, sigue igual. y estoy viendo en modo dev.~~ → 283A-19

18. ~~Ok ya cambio el tamaño pero como se intento varias veces ahora el tamaño es muy grande, reducelo un poco, a 512 como el resto de modales.~~ → 283A-30

19. ~~Vi que hiciste la auditoria de marketing, haz lo que falta.~~ → 283A-23

~~20. Las mesas no deberían salirse del plano, tambien veo que las mesas ahora no se actualizan en reserva y que el mensae de error al crear una mesa con un numero usado es "ocurrio un error en la base de datos"~~ → 283A-24

21. ~~Los cuadros de meses deberian tener un icono de mesa.~~ → 283A-30

22. ~~Agregar una opcion para reportar errores, los erores llegan a mi correo andoryyu@gmail.com~~ → 283A-26

23. ~~283A-24, si bien las mesas ya no se salen, recuerda que habiamos dicho que el plano ahora debería expandirse el 100% de ancho pero ajam, parece que no puedo mover mas alla de 600px de ancho, y en reservas el plano no esta ocupando el 100% del ancho, revisa eso.~~ → 283A-25

23.1 ~~Creo que podemos evitar problemas futuros en donde haya espacio para mesas agregando la funcionalidad de zoom, esto para que la funcionalidad anterior no quede inutil sería simplemnente reducir el tamaño de las mesas o agrandar segun el zoom.~~ → 283A-25

24. ~~En ventas no aparece todas las ventas cuando en realidad y ventas.~~ → 283A-31

25. ~~Las tablas todas deberían tener para filtrar y ordenar. En gastos no hay para filtrar por fecha.~~ → 283A-34

26. ~~Asegurate que en el servidor esten los datos de prueba.~~ → 283A-35

27. ~~Vi que te costo el despliegue, asegurate de que sea sencillo y facil la proxima vez.~~ → 283A-35

28. ~~Los graficos siguen teniendo un espacio a la izquierda inncesario y Distribución por canal debe mostrar las leyenda.~~ → 283A-34

29. ~~En coolify veo

Rust App ()
restaurante.wandori.us
Exited
Settings
App ()
http://app-b8s0cks444o0sogo8kg8wcgw.66.94.100.241.sslip.io:3...
Exited
Postgres (postgres:16-alpine)
Exited

tambien veo que en App dice

Required Port: 3000
This service requires port 3000 to function correctly. All domains must include this port number (or any other port if you know what you're doing).

Example: http://app.coolify.io:3000

2 app, puse en la Rust App la url que voy a usar pero veo otra que dice App,~~ → 283A-36

30. ~~No veo la opcion de reportar errores en ningún lado~~ → 283A-36

31. ~~primero, es que debajo de 50% se ve mal, limitarlo hasta ahi, que el salto sea de 10 en 10, tambien espero que el zoom se aplique con reservas, tiene que estar sincronizado, no solo por pagina sino tambie en general para todos los usuarios que usen la misma cuenta porque sino se puede ver diferente para un usuario y otro.~~ → 283A-36

Tarea Final. ~~Actualiza el despliege con coolify-manager-rs~~ → 283A-33

~~32. demo@restaurante.com demo1234 no funciona, dice "Credenciales incorrectas" los datos de prueba tienen que estar desplegados y un boton para eliminar los datos de prueba y tambien para volver a desplegar los datos de prueba.~~ → 283A-39

~~33. Antes habiamos dicho que /glory-rs en https://github.com/1ndoryu/glory-rs y el proyecto en si en https://github.com/1ndoryu/glory-rs-template en la rama glory-rs-rest, aun no veo github actualizado, esto implica actualizar como se actualiza el despliegue supongo.~~ → 283A-40

~~34. El zoom en reservas solo cambia el tamaño de las letras no el cuadro en el plano.~~ → 283A-41

~~35. Despues de las ultimas tareas actualizar el despliegue con los cambios.~~ → 283A-42

~~283A-43: PlanoOcupacion aspect-ratio fijo + drag no guarda + zoom inconsistente plano/reservas~~ → completada
~~283A-44: Optimizar build Docker (deploy lento)~~ → completada
~~283A-45: Ancho formulario plantilla/campaña = 322px → normal~~ → completada
~~283A-46: Select.Item value="" en nueva reserva~~ → completada
~~283A-47: "manana" en vez de "mañana" en turnos~~ → completada
~~283A-48: Despliegue final~~ → completada
~~283A-49: baseURL localhost:3000 hardcodeada en frontend producción~~ → completada
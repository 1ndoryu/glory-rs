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

2. Lo de resumen y general deberia convinarse, y los graficos no poderlos en un grid de 3, dejar cada uno en una 1 sola fila, los graficos todos tienen un padding a la izquierda que hace que tengan un espacio innecesario.  

3. En ventas, reservas, clientes, faltan filtros, filtrar por fecha, falta un buscador en tiempo real.

4. Distribución por hora y Por turno solos en grid de 2, no 3, haciendo que Procedencia (canal) este abajo en un solo bloque.

5. Debería poder editarse los gastos y ventas por si se comente un error, creo. 

6. En la parte de recordatorio (y creo que esto sucede en el resto de paginas una similar estructura) <p class="text-muted-foreground text-xs mt-0.5 line-clamp-1">Estimado/a {nombre}, le recordamos que tiene una reserva programada para mañana a las {hora}. Si necesita cancelar o modificar, responda a este email.</p> cuando es muy largo hace que la pagina supere el ancho y se ve mal. 

7. Cuando un dia tiene reservas en el calendario se ve mal, se ve gris y como el fondo es blanco no constrasta bien. 

8. No veo configuraciones para la api de chatbot, donde se genera la api para conectarse, tambien deberia haber un enlace para la documentacion, configuraciones, etc. 

9. Ya lo habia dicho antes, el modal de Nuevo Gasto debe ser mas grande, un poco. 

10. Cumple con # Plan: Glory-RS como Framework Reutilizable. 

11. Supongo que lo de  /* [283A-17] Catch unique constraint (zona_id, numero) para devolver
         * Conflict en vez de 500 cuando el número de mesa ya existe en la zona. */ 
Es un error que se puede evitar de nuevo con alguna regla en sentinel, pero si no, ignorar por ahora.

Final. Actualiza el despliege con coolify-manager-rs

12. Las cosas de marketing y campaña como se conectan? o sea, cuales son las configuraciones que hay que hacer para conectarlas? Veo que todo esta, pero ajam presiento que falta un monton de cosas, revisa que falta, que configuraciones hay que hacer, para que todo funcione, esto es general, se requiere revision de todo porque si hay un hueco aca, nada quita que haya en otros lados. 

13. El cliente no lo pidio pero estaría bien que hayan notificaciones en el panel en tiempo real para cosas como nuevas reservas, o lo que sea que se haga mediante el chat bot, porque ajam si se reciben cosas externamente, pues, deberían haber notifaciones en tiempo real. 
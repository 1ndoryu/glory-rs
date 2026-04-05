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

> Plan maestro: `Agente/planes/plan-marketplace-2026-04-04.md` (11 fases)
> Plan de chat: `Agente/planes/plan-live-chat-2026-04-04.md` (5 fases)
> Plan de hosting: `Agente/planes/plan-hosting-coolify-2026-04-04.md` (5 fases)

### Marketplace — Fase 9: Notificaciones
- BD + endpoints + WebSocket real-time
- Email para críticas

### Marketplace — Fase 10: Dashboard admin
- Revenue, métricas, alertas

## Otras tareas

- Cuando le doy a empezar en los planes de cualquier servicio debería empezar algo similar a fiver. Esto requiere restructurar un poco como se ven la pagina de los servicios, principalmente al principio debería verse la portada, tambien abajo de los botones de comenzar el plan tiene que ir el boton de conversar. Todos los servicios deben tener un plan basico, uno medio y uno pro, quita los de "A medida", se va a resumir con el boton de conversar, obviamente se tiene entender que el cliente dio click en ese boton porque es informacion util para cuando se abra el chat y la IA lo atendienda en primer lugar.

- Cuando se de a comprar un servicio abre un modal que muestre la info brevemente del servicio, y con un boton abajo de continuar ($Precio), abajo un pequeño texto de Aun no se te cobrara. Luego de continuar va a la pasarela de pago. Al volver su el servicio aparecera en su panel, por supuesto, para que esto sea sencillo, si el usuario no esta registrado tenemos que hacer que eso no sea un impedimiento para comprar, en el mismo modal le podemos pedir que ponga su correo y una contraseña o en la pasarela, no lo se pero me entiendes el proposito.

- Parece que las paginas se estan recargando al navegar entre ellas, eso no debería de pasar.

- Todos los repositories estan usando query_as sin macro, sin usar sqlx::query_as! Necesito que la verificación sea en compilacion en todos lados!!.

- Con el plan de chat, revisa que pude haberme olvidado, tambien el plan de servicio de hosting que me olvide, queremos ofrecer hosting y que los clientes pueda comprar hosting y gestionarlos. Y trabaja en todo eso que posiblemente olvide. 

- En el panel de admin hace falta algo para ver los usuarios registrados, con buscador, y filtro, con capacidad de banear, cambiar de cliente a empleado, etc.
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

- Antes habia pedido un plan completo para que el servicio de hosting funcionara, hacer todo lo necesario aprovechando que existe coolify manager-rs, esto es una tarea gigante asi que requiere una planificación buena.
- borra el servicio de chatbot y tambien

- Planifica la tarea de integrar un chat a Nakomi donde los clientes puedan hablar directamente con los encargados, por supuesto, esto es una tarea complicada, significa muchas cosas, 1. Que los clientes pueden escribir sin estar logeados. 2. Integrar inteligencia artificial para que responda cuando no haya personal disponible. 3. Implica que el personal puede intervenir en una conversacion que esta siendo atentida por una IA 4. Implica gestionar bien un panel para el personal para ver las conversaciones. 5. El personal puede ver lo que escribe el usuario antes de enviar.
- el diseño de blogCard es muy feo, tiene ser mas minimalista, solo el titulo y el tilo sobre la iamgen.
- Quita el selectorIdioma del header
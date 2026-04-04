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

(sin tareas pendientes)



- Pon en .proyectoHeroMeta font weight 500 !important
- Quita de .heroTitulo span la linea. 
- .tarjetaPlanDescripcion necesita 17px y .tarjetaPlanItemIncluido .tarjetaPlanItemTexto sin font serif
- Hay botones con <button class="botonBase botonPrimario botonMediano ctaBotonPrimario">Contratar desde $997</button> que estan puestos y no son coherentes con el precio minimo del servicio que aparece. 
- Cuando hago un arrastre a carruselTitulo automaticamente se da click
- Las imagenes de la galería dentro de los proyecto sigue sin ser cuadradas. 
- de .skillDescripcion p quita borra los estilos de  /* font-family: var(--font-serif); *//* font-size: var(--text-lg);
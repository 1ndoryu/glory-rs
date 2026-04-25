## Stack implementado

| Capa                 | Herramienta                                    |
| -------------------- | ---------------------------------------------- |
| Framework web        | Axum 0.7                                       |
| OpenAPI              | utoipa 4 + utoipa-swagger-ui 7                 |
| SerializaciÃ³n        | serde                                          |
| Base de datos        | SQLx 0.8 (PostgreSQL)                          |
| Migraciones          | SQLx migrate                                   |
| ValidaciÃ³n           | validator 0.18                                 |
| Variables de entorno | dotenvy                                        |
| Logging              | tracing + tracing-subscriber                   |
| Errores              | thiserror 2                                    |
| Auth                 | jsonwebtoken + argon2                          |
| CORS                 | tower-http                                     |
| Linter               | clippy (deny all + warn pedantic)              |
| Frontend             | React 18 + TypeScript + Vite                   |
| State                | React Query + Zustand                          |
| Codegen              | Orval 8 (reemplaza openapi-typescript-codegen) |

## MisiÃ³n principal â€” MigraciÃ³n Kamples PHP â†’ Rust

**Objetivo:** Portar el proyecto `glorytemplate` (Kamples sobre WordPress + PHP + React Islands) a `glory-rust-template` (Axum + SQLx + React + Vite + Orval), manteniendo paridad funcional total y mejorando arquitectura.

**Origen:** `C:\Users\Owner\OneDrive\Documentos\WP\app\public\wp-content\themes\glorytemplate`
**Destino:** `C:\Users\Owner\OneDrive\Documentos\glory-rust-template`

**Alcance del origen (estimaciÃ³n):**
- ~60 controladores REST (`App/Kamples/Api/Controladores/`)
- ~42 repositorios (`App/Kamples/Database/Repositories/`)
- ~38 servicios de dominio (`App/Kamples/Services/`)
- 74 migraciones SQL acumuladas (`App/Kamples/Database/migrations/v001..v074`)
- Auth propia + Google OAuth (web y mobile) + JWT
- Audio pipeline: FFmpeg, BPM, tonalidad, embeddings pgvector 128d, deduplicaciÃ³n
- IA: moderaciÃ³n (4 capas), generaciÃ³n, embeddings (Groq + OpenAI)
- Pagos Stripe + revenue share + cÃ³digos gratis + transacciones idempotentes
- WebSocket Bun standalone (notificaciones tiempo real)
- Push: FCM (Firebase) + Web Push
- Blog completo (artÃ­culos, likes, moderaciÃ³n, feed)
- DAW web (Channel Rack + Mixer + Piano Roll)
- Sync changelog + desktop (Tauri) + mobile (Android APK)
- SEO dinÃ¡mico + sitemaps + JSON-LD
- Algoritmo de descubrimiento (6 seÃ±ales, planificador, precomputador feed)
- Subcolecciones, contribuciones, mensajes multimedia, reacciones, reportes legales

**Frontend:** Ya existe y NO se recrea desde cero. Se migra del modelo "React Islands en WordPress" a SPA pura (Vite + Orval). Toda la UI/UX, componentes, estilos y stores existentes (`App/React/`) se preservan; lo que cambia es la capa de datos (servicios PHP fetch â†’ cliente Orval generado desde OpenAPI Rust).

**Aclaraciones del usuario (2026-04-17):**
- **Algoritmo de descubrimiento:** debe recrearse en Rust con los mismos detalles del proyecto anterior (6 seÃ±ales, embeddings 128d, planificador, precomputador feed, selector candidatos). Ver `App/docs (ignorar)/algoritmo.md` y `App/Kamples/Services/{MotorRecomendacion, PlanificadorAlgoritmo, PrecomputadorFeed, SelectorCandidatos, ConstructorSenales, GeneradorEmbeddings}.php` como referencia funcional.
- **Una sola base de datos:** PostgreSQL Ãºnico (con pgvector). No replicar el split WordPress MySQL + Postgres del legado. Todo va a la BD Postgres del template Rust.
- **Sistemas a portar (incluidos):** scraper (`kamples-scraper/`), mezclador/DAW (`Mezclador/`), mobile WebView (`mobile/` Android), WebSocket (consolidar Bun standalone â†’ Axum WS o mantener Bun segÃºn decisiÃ³n tÃ©cnica del plan).
- **Frontend ya hecho:** reusar tal cual; solo regenerar capa de servicios desde el cliente Orval.

**Principios de la migraciÃ³n:**
1. Lo agnÃ³stico va a `glory-rs/` (framework reutilizable). Lo especÃ­fico de Kamples va al proyecto.
2. OpenAPI (`utoipa`) como contrato Ãºnico. Frontend NO escribe tipos a mano â€” todo viene de Orval.
3. Migraciones SQLx versionadas; consolidar las 74 migraciones legacy en un schema base limpio + migraciones nuevas a partir de ahÃ­ (no replicar la historia).
4. Repositorios PHP â†’ mÃ³dulos Rust con `sqlx::query_as!` (validaciÃ³n compile-time).
5. Servicios PHP â†’ traits + structs Rust con DI explÃ­cita.
6. Controladores PHP â†’ handlers Axum delgados; toda lÃ³gica en services.
7. Newtypes para IDs de dominio (`SampleId`, `UserId`, `ColeccionId`...).
8. Cero parches: si el diseÃ±o PHP era subÃ³ptimo, rediseÃ±ar â€” no portar deuda tÃ©cnica.

### Fase 19 â€” Despliegue
- Nota: un error pasado hizo que se borrara la base de datos, hay que reforzar para que no vuelva a suceder, el error problemente fue causado al desplegar directamente o alguna otra razón, se debe tomar todo los medios necesarios para evitar la perdida de datos, tanto de la base de datos tanto como los archivos fisicos que deben perdurar, esto debe tomarse muy en serio porque ya es un error que se cometio en el pasado.
- 174A-116 â€” MigraciÃ³n inicial automÃ¡tica + healthcheck (no hacerlo hasta mi confirmacion)
- 174A-117 â€” Deploy via `coolify-manager-rs`(no hacerlo hasta mi confirmacion)
- 174A-118 â€” Smoke test producciÃ³n + rollback procedure (no hacerlo hasta mi confirmacion)

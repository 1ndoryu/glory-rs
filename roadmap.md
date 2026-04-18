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

## Pendientes

> **Plan maestro:** [`Agente/planes/plan-migracion-kamples-rust-2026-04-17.md`](Agente/planes/plan-migracion-kamples-rust-2026-04-17.md) â€” 118 tareas en 19 fases.
> Cada tarea atÃ³mica de abajo se ejecuta como un commit independiente siguiendo los 10 pasos del protocolo. La descripciÃ³n detallada de cada una estÃ¡ en el plan.

### Fase 0 â€” Bootstrap del proyecto Rust

### Fase 1 â€” Schema base PostgreSQL

### Fase 2 â€” Auth y usuarios

### Fase 3 â€” Storage + uploads
- nota: me di cuenta que hay lugares en donde no se esta usando query! y en vez se usa sin !, necesitamos comprobaciones reales en tiempo real para ir chqueando que todo va quedando bien, corrige todo los query posibles.

### Fase 4 â€” Audio pipeline (lo mÃ¡s difÃ­cil)
- 174A-31 â€” `audio/bpm.rs` (symphonia + onset + autocorrelaciÃ³n)
- 174A-32 â€” `audio/key.rs` (cromagramas + scale)
- 174A-33 â€” `audio/embeddings.rs` (vector 128d determinista)
- 174A-34 â€” Service orquestador `audio_pipeline.rs`
- 174A-35 â€” Worker async que consume `pending_processing`
- 174A-36 â€” Tests con audio fixtures

### Fase 5 â€” IA pipeline
- 174A-37 â€” Cliente Groq con rotaciÃ³n 3 keys + retry + fallback chain modelos
- 174A-38 â€” Cliente OpenAI (fallback final)
- 174A-39 â€” `JsonRepairer`
- 174A-40 â€” `prompts.rs`
- 174A-41 â€” Service IA (analiza y devuelve tags/gÃ©neros/instrumentos/emociones)
- 174A-42 â€” Worker `ia_queue_worker.rs` (rate limit + backoff)
- 174A-43 â€” Service moderaciÃ³n (4 capas)

### Fase 6 â€” Samples CRUD + bÃºsqueda
- 174A-44 â€” `GET /samples` (paginaciÃ³n + filtros)
- 174A-45 â€” `GET /samples/{slug}` + `GET /samples/random`
- 174A-46 â€” `PATCH /samples/{slug}` + `DELETE /samples/{slug}` (owner check, soft-delete)
- 174A-47 â€” BÃºsqueda fuzzy trigram
- 174A-48 â€” BÃºsqueda similitud pgvector (`GET /samples/{id}/similar`)

### Fase 7 â€” Algoritmo de descubrimiento (lo mÃ¡s difÃ­cil #2)
- 174A-49 â€” `algorithm/signals.rs` (6 seÃ±ales con pesos exactos)
- 174A-50 â€” `algorithm/profile.rs` (PerfilUsuario, TTL 30min)
- 174A-51 â€” `algorithm/candidates.rs` (SelectorCandidatos)
- 174A-52 â€” `algorithm/recommender.rs` (MotorRecomendacion + cache stale-while-revalidate)
- 174A-53 â€” `algorithm/precompute.rs` (PrecomputadorFeed + bulk LIMIT*3)
- 174A-54 â€” `tag_affinity.rs`
- 174A-55 â€” Worker `algo_planner.rs`
- 174A-56 â€” Endpoints `GET /feed`, `GET /me/feed`, `GET /samples/random`
- 174A-57 â€” MÃ©tricas algo_timing + endpoint admin

### Fase 8 â€” Reproducciones, likes, follows, downloads
- 174A-58 â€” `POST /samples/{id}/play`
- 174A-59 â€” Likes polimÃ³rficos
- 174A-60 â€” Follows + Blocks
- 174A-61 â€” Downloads (lÃ­mites por plan + tracking)
- 174A-62 â€” `POST /downloads/{sample_id}/zip`
- 174A-63 â€” Stream con range (`GET /downloads/{sample_id}/stream`)

### Fase 9 â€” Colecciones y sociales
- 174A-64 â€” Colecciones CRUD + M2M con orden
- 174A-65 â€” Merge colecciones
- 174A-66 â€” Saved collections
- 174A-67 â€” Posts + reposts + likes
- 174A-68 â€” Comments polimÃ³rficos + likes + multimedia

### Fase 10 â€” MensajerÃ­a + WebSocket
- 174A-69 â€” `glory-rs/backend/websocket` hub + ticket HMAC
- 174A-70 â€” `GET /ws` upgrade + `GET /ws/ticket`
- 174A-71 â€” Conversations + messages
- 174A-72 â€” WS events emitidos
- 174A-73 â€” Multi-instancia: Redis pub/sub

### Fase 11 â€” Notificaciones (5 canales)
- 174A-74 â€” Tabla notifications + service base
- 174A-75 â€” Web Push VAPID (registro + envÃ­o)
- 174A-76 â€” FCM Android (service-account + envÃ­o)
- 174A-77 â€” Email SMTP + plantillas
- 174A-78 â€” Pipeline `notify(user, event)` integrado

### Fase 12 â€” Pagos
- 174A-79 â€” Wrapper Stripe + planes Kamples
- 174A-80 â€” `GET /pagos/planes`
- 174A-81 â€” Checkout suscripciÃ³n + sample + portal
- 174A-82 â€” Webhook con HMAC + idempotencia
- 174A-83 â€” Connect onboarding + revenue share
- 174A-84 â€” CÃ³digos gratis CRUD + uso

### Fase 13 â€” Reportes, blog, bÃºsqueda global, catÃ¡logo
- 174A-85 â€” Reportes (legales, contenido, errores)
- 174A-86 â€” Blog (artÃ­culos CRUD + comentarios + categorÃ­as)
- 174A-87 â€” BÃºsqueda global (`GET /search?q=...`)
- 174A-88 â€” CatÃ¡logo canciones/artistas + relaciones

### Fase 14 â€” ModeraciÃ³n, admin, dashboard
- 174A-89 â€” Panel admin endpoints
- 174A-90 â€” Dashboard creador
- 174A-91 â€” Sync changelog (`GET /sync/changelog?since=...`)

### Fase 15 â€” Workers
- 174A-92 â€” `cleanup_expired_subscriptions`
- 174A-93 â€” `precompute_feeds`
- 174A-94 â€” `process_ia_queue` (90s)
- 174A-95 â€” `process_scraping_queue`
- 174A-96 â€” `recompute_user_profiles`
- 174A-97 â€” MÃ©tricas opcionales

### Fase 16 â€” SEO
- 174A-98 â€” `/sitemap.xml` dinÃ¡mico
- 174A-99 â€” Endpoint metadata SEO

### Fase 17 â€” Frontend SPA (reuso de `App/React/` legacy)
- 174A-100 â€” Configurar `frontend/orval.config.ts` (`tags-split`)
- 174A-101 â€” Generar primer cliente Orval + type-check verde
- 174A-102 â€” Migrar islands desde `App/React/islands/` a `frontend/src/features/{dominio}/`
- 174A-103 â€” Integrar React Router para navegaciÃ³n SPA
- 174A-104 â€” Reemplazar servicios manuales por hooks Orval/React Query
- 174A-105 â€” Hook `useWebSocket()` actualizado para Axum WS
- 174A-106 â€” `useAuth` contra nuevo backend
- 174A-107 â€” Smoke test SPA full-flow

### Fase 18 â€” Scraper / Mezclador / Mobile / Desktop
- 174A-108 â€” Adaptar scraper Python a nueva API
- 174A-109 â€” Adaptar Mezclador (Tauri DAW)
- 174A-110 â€” Adaptar mobile (Capacitor) + FCM + deep links
- 174A-111 â€” Adaptar desktop (Tauri) + auto-updates

### Fase 19 â€” Despliegue
- 174A-112 â€” Dockerfile multi-stage Rust
- 174A-113 â€” Dockerfile frontend (Vite â†’ nginx)
- 174A-114 â€” `docker-compose.yml` Coolify
- 174A-115 â€” Secrets management
- 174A-116 â€” MigraciÃ³n inicial automÃ¡tica + healthcheck
- 174A-117 â€” Deploy via `coolify-manager-rs`
- 174A-118 â€” Smoke test producciÃ³n + rollback procedure

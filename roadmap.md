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

## Misión principal — Migración Kamples PHP → Rust

**Objetivo:** Portar el proyecto `glorytemplate` (Kamples sobre WordPress + PHP + React Islands) a `glory-rust-template` (Axum + SQLx + React + Vite + Orval), manteniendo paridad funcional total y mejorando arquitectura.

**Origen:** `C:\Users\Owner\OneDrive\Documentos\WP\app\public\wp-content\themes\glorytemplate`
**Destino:** `C:\Users\Owner\OneDrive\Documentos\glory-rust-template`

**Alcance del origen (estimación):**
- ~60 controladores REST (`App/Kamples/Api/Controladores/`)
- ~42 repositorios (`App/Kamples/Database/Repositories/`)
- ~38 servicios de dominio (`App/Kamples/Services/`)
- 74 migraciones SQL acumuladas (`App/Kamples/Database/migrations/v001..v074`)
- Auth propia + Google OAuth (web y mobile) + JWT
- Audio pipeline: FFmpeg, BPM, tonalidad, embeddings pgvector 128d, deduplicación
- IA: moderación (4 capas), generación, embeddings (Groq + OpenAI)
- Pagos Stripe + revenue share + códigos gratis + transacciones idempotentes
- WebSocket Bun standalone (notificaciones tiempo real)
- Push: FCM (Firebase) + Web Push
- Blog completo (artículos, likes, moderación, feed)
- DAW web (Channel Rack + Mixer + Piano Roll)
- Sync changelog + desktop (Tauri) + mobile (Android APK)
- SEO dinámico + sitemaps + JSON-LD
- Algoritmo de descubrimiento (6 señales, planificador, precomputador feed)
- Subcolecciones, contribuciones, mensajes multimedia, reacciones, reportes legales

**Frontend:** Ya existe y NO se recrea desde cero. Se migra del modelo "React Islands en WordPress" a SPA pura (Vite + Orval). Toda la UI/UX, componentes, estilos y stores existentes (`App/React/`) se preservan; lo que cambia es la capa de datos (servicios PHP fetch → cliente Orval generado desde OpenAPI Rust).

**Aclaraciones del usuario (2026-04-17):**
- **Algoritmo de descubrimiento:** debe recrearse en Rust con los mismos detalles del proyecto anterior (6 señales, embeddings 128d, planificador, precomputador feed, selector candidatos). Ver `App/docs (ignorar)/algoritmo.md` y `App/Kamples/Services/{MotorRecomendacion, PlanificadorAlgoritmo, PrecomputadorFeed, SelectorCandidatos, ConstructorSenales, GeneradorEmbeddings}.php` como referencia funcional.
- **Una sola base de datos:** PostgreSQL único (con pgvector). No replicar el split WordPress MySQL + Postgres del legado. Todo va a la BD Postgres del template Rust.
- **Sistemas a portar (incluidos):** scraper (`kamples-scraper/`), mezclador/DAW (`Mezclador/`), mobile WebView (`mobile/` Android), WebSocket (consolidar Bun standalone → Axum WS o mantener Bun según decisión técnica del plan).
- **Frontend ya hecho:** reusar tal cual; solo regenerar capa de servicios desde el cliente Orval.

**Principios de la migración:**
1. Lo agnóstico va a `glory-rs/` (framework reutilizable). Lo específico de Kamples va al proyecto.
2. OpenAPI (`utoipa`) como contrato único. Frontend NO escribe tipos a mano — todo viene de Orval.
3. Migraciones SQLx versionadas; consolidar las 74 migraciones legacy en un schema base limpio + migraciones nuevas a partir de ahí (no replicar la historia).
4. Repositorios PHP → módulos Rust con `sqlx::query_as!` (validación compile-time).
5. Servicios PHP → traits + structs Rust con DI explícita.
6. Controladores PHP → handlers Axum delgados; toda lógica en services.
7. Newtypes para IDs de dominio (`SampleId`, `UserId`, `ColeccionId`...).
8. Cero parches: si el diseño PHP era subóptimo, rediseñar — no portar deuda técnica.

## Pendientes

> **Plan maestro:** [`Agente/planes/plan-migracion-kamples-rust-2026-04-17.md`](Agente/planes/plan-migracion-kamples-rust-2026-04-17.md) — 118 tareas en 19 fases.
> Cada tarea atómica de abajo se ejecuta como un commit independiente siguiendo los 10 pasos del protocolo. La descripción detallada de cada una está en el plan.

### Fase 0 — Bootstrap del proyecto Rust

### Fase 1 — Schema base PostgreSQL
- 174A-7 — Migración 0001 extensions (pgvector, pg_trgm)
- 174A-8 — Migración 0002 users + trigger updated_at
- 174A-9 — Migración 0003 samples + trigger enriched_tags + índices (gin/trgm/hnsw)
- 174A-10 — Migración 0004 collections + collection_samples
- 174A-11 — Migración 0005 social graph (follows, blocks, likes, plays, downloads, comments, posts, reposts)
- 174A-12 — Migración 0006 messaging (conversations, messages)
- 174A-13 — Migración 0007 notifications + push_subscriptions + fcm_tokens
- 174A-14 — Migración 0008 payments (subscriptions, transactions, free_codes, upload_idempotency)
- 174A-15 — Migración 0009 blog (articles, article_likes)
- 174A-16 — Migración 0010 catalog (artists, songs, sample_song_relations)
- 174A-17 — Migración 0011 algo + colas + sync (algo_state, user_tag_scores, batch_jobs, ia_queue, scraping_*, moderation_queue, reports, legal_reports, duplicate_pairs, sync_changelog, audit_log)

### Fase 2 — Auth y usuarios
- 174A-18 — `glory-rs/backend/auth` (argon2, JWT, refresh tokens Redis)
- 174A-19 — Middleware `AuthLayer` + extractor `CurrentUser`
- 174A-20 — Endpoints register/login/logout/refresh + tests
- 174A-21 — OAuth Google ID token (web)
- 174A-22 — OAuth Google PKCE (desktop)
- 174A-23 — OAuth Google mobile callback
- 174A-24 — Endpoints perfil (`GET /me`, `PATCH /me`, `GET /users/{username}`)
- 174A-25 — Suspensión / bloqueo / eliminación (admin)

### Fase 3 — Storage + uploads
- 174A-26 — Trait `FileStorage` + `LocalFs`
- 174A-27 — `S3Storage` (feature-gated)
- 174A-28 — `POST /samples/check-duplicate` (SHA-256 streaming)
- 174A-29 — `POST /samples/upload` (multipart + idempotency + MIME)

### Fase 4 — Audio pipeline (lo más difícil)
- 174A-30 — `audio/ffmpeg.rs` (detect, duration, convert MP3/FLAC, waveform peaks)
- 174A-31 — `audio/bpm.rs` (symphonia + onset + autocorrelación)
- 174A-32 — `audio/key.rs` (cromagramas + scale)
- 174A-33 — `audio/embeddings.rs` (vector 128d determinista)
- 174A-34 — Service orquestador `audio_pipeline.rs`
- 174A-35 — Worker async que consume `pending_processing`
- 174A-36 — Tests con audio fixtures

### Fase 5 — IA pipeline
- 174A-37 — Cliente Groq con rotación 3 keys + retry + fallback chain modelos
- 174A-38 — Cliente OpenAI (fallback final)
- 174A-39 — `JsonRepairer`
- 174A-40 — `prompts.rs`
- 174A-41 — Service IA (analiza y devuelve tags/géneros/instrumentos/emociones)
- 174A-42 — Worker `ia_queue_worker.rs` (rate limit + backoff)
- 174A-43 — Service moderación (4 capas)

### Fase 6 — Samples CRUD + búsqueda
- 174A-44 — `GET /samples` (paginación + filtros)
- 174A-45 — `GET /samples/{slug}` + `GET /samples/random`
- 174A-46 — `PATCH /samples/{slug}` + `DELETE /samples/{slug}` (owner check, soft-delete)
- 174A-47 — Búsqueda fuzzy trigram
- 174A-48 — Búsqueda similitud pgvector (`GET /samples/{id}/similar`)

### Fase 7 — Algoritmo de descubrimiento (lo más difícil #2)
- 174A-49 — `algorithm/signals.rs` (6 señales con pesos exactos)
- 174A-50 — `algorithm/profile.rs` (PerfilUsuario, TTL 30min)
- 174A-51 — `algorithm/candidates.rs` (SelectorCandidatos)
- 174A-52 — `algorithm/recommender.rs` (MotorRecomendacion + cache stale-while-revalidate)
- 174A-53 — `algorithm/precompute.rs` (PrecomputadorFeed + bulk LIMIT*3)
- 174A-54 — `tag_affinity.rs`
- 174A-55 — Worker `algo_planner.rs`
- 174A-56 — Endpoints `GET /feed`, `GET /me/feed`, `GET /samples/random`
- 174A-57 — Métricas algo_timing + endpoint admin

### Fase 8 — Reproducciones, likes, follows, downloads
- 174A-58 — `POST /samples/{id}/play`
- 174A-59 — Likes polimórficos
- 174A-60 — Follows + Blocks
- 174A-61 — Downloads (límites por plan + tracking)
- 174A-62 — `POST /downloads/{sample_id}/zip`
- 174A-63 — Stream con range (`GET /downloads/{sample_id}/stream`)

### Fase 9 — Colecciones y sociales
- 174A-64 — Colecciones CRUD + M2M con orden
- 174A-65 — Merge colecciones
- 174A-66 — Saved collections
- 174A-67 — Posts + reposts + likes
- 174A-68 — Comments polimórficos + likes + multimedia

### Fase 10 — Mensajería + WebSocket
- 174A-69 — `glory-rs/backend/websocket` hub + ticket HMAC
- 174A-70 — `GET /ws` upgrade + `GET /ws/ticket`
- 174A-71 — Conversations + messages
- 174A-72 — WS events emitidos
- 174A-73 — Multi-instancia: Redis pub/sub

### Fase 11 — Notificaciones (5 canales)
- 174A-74 — Tabla notifications + service base
- 174A-75 — Web Push VAPID (registro + envío)
- 174A-76 — FCM Android (service-account + envío)
- 174A-77 — Email SMTP + plantillas
- 174A-78 — Pipeline `notify(user, event)` integrado

### Fase 12 — Pagos
- 174A-79 — Wrapper Stripe + planes Kamples
- 174A-80 — `GET /pagos/planes`
- 174A-81 — Checkout suscripción + sample + portal
- 174A-82 — Webhook con HMAC + idempotencia
- 174A-83 — Connect onboarding + revenue share
- 174A-84 — Códigos gratis CRUD + uso

### Fase 13 — Reportes, blog, búsqueda global, catálogo
- 174A-85 — Reportes (legales, contenido, errores)
- 174A-86 — Blog (artículos CRUD + comentarios + categorías)
- 174A-87 — Búsqueda global (`GET /search?q=...`)
- 174A-88 — Catálogo canciones/artistas + relaciones

### Fase 14 — Moderación, admin, dashboard
- 174A-89 — Panel admin endpoints
- 174A-90 — Dashboard creador
- 174A-91 — Sync changelog (`GET /sync/changelog?since=...`)

### Fase 15 — Workers
- 174A-92 — `cleanup_expired_subscriptions`
- 174A-93 — `precompute_feeds`
- 174A-94 — `process_ia_queue` (90s)
- 174A-95 — `process_scraping_queue`
- 174A-96 — `recompute_user_profiles`
- 174A-97 — Métricas opcionales

### Fase 16 — SEO
- 174A-98 — `/sitemap.xml` dinámico
- 174A-99 — Endpoint metadata SEO

### Fase 17 — Frontend SPA (reuso de `App/React/` legacy)
- 174A-100 — Configurar `frontend/orval.config.ts` (`tags-split`)
- 174A-101 — Generar primer cliente Orval + type-check verde
- 174A-102 — Migrar islands desde `App/React/islands/` a `frontend/src/features/{dominio}/`
- 174A-103 — Integrar React Router para navegación SPA
- 174A-104 — Reemplazar servicios manuales por hooks Orval/React Query
- 174A-105 — Hook `useWebSocket()` actualizado para Axum WS
- 174A-106 — `useAuth` contra nuevo backend
- 174A-107 — Smoke test SPA full-flow

### Fase 18 — Scraper / Mezclador / Mobile / Desktop
- 174A-108 — Adaptar scraper Python a nueva API
- 174A-109 — Adaptar Mezclador (Tauri DAW)
- 174A-110 — Adaptar mobile (Capacitor) + FCM + deep links
- 174A-111 — Adaptar desktop (Tauri) + auto-updates

### Fase 19 — Despliegue
- 174A-112 — Dockerfile multi-stage Rust
- 174A-113 — Dockerfile frontend (Vite → nginx)
- 174A-114 — `docker-compose.yml` Coolify
- 174A-115 — Secrets management
- 174A-116 — Migración inicial automática + healthcheck
- 174A-117 — Deploy via `coolify-manager-rs`
- 174A-118 — Smoke test producción + rollback procedure

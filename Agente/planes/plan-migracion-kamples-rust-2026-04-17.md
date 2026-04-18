# Plan de Migración Kamples (PHP/WordPress) → Rust (Axum + SQLx)

**Tarea raíz:** 174A-1
**Fecha:** 2026-04-17
**Origen:** `c:\Users\Owner\OneDrive\Documentos\WP\app\public\wp-content\themes\glorytemplate`
**Destino:** `c:\Users\Owner\OneDrive\Documentos\glory-rust-template`
**Estado:** ACTIVO — la planificación es viva, se actualiza en cada fase.

---

## 0. Resumen ejecutivo

Migrar Kamples (plataforma de samples musicales) de WordPress + PHP + React Islands a un backend Rust + Axum + SQLx + PostgreSQL puro, con frontend SPA (React + Vite) consumiendo cliente generado por Orval desde OpenAPI.

**Volumen origen:**
- 59 controladores REST PHP
- 40+ servicios de dominio
- 42 repositorios
- 74 migraciones SQL acumuladas (consolidar a un schema base limpio)
- ~40 islands React, ~38 stores Zustand, ~25 servicios HTTP frontend
- 6+ servicios externos: PostgreSQL, Redis, Stripe, Groq, OpenAI, FCM, VAPID Web Push, SMTP, S3 (opcional), FFmpeg
- WebSocket Bun standalone
- Scraper Python (yt-dlp + Scrapy)
- Mezclador (DAW Tauri desktop)
- Mobile (Capacitor Android WebView)
- Desktop (Tauri)

**Decisiones macro irrevocables (acordadas con usuario 2026-04-17):**
1. **Una sola base de datos:** PostgreSQL único (con `pgvector` + `pg_trgm`). No hay MySQL/WordPress paralelo. Los datos de wp_users + usuarios_ext se consolidan en una sola tabla `users`.
2. **Algoritmo:** se replica con los mismos detalles (6 señales, embeddings 128d, cache stale-while-revalidate, bulk-fetch optimization).
3. **Scraper, Mezclador, Mobile (WebView), WebSocket:** se incluyen en la migración.
4. **Frontend ya existe:** se reusa tal cual desde `App/React/`. Sólo se reemplaza la capa de servicios HTTP (manuales) por el cliente generado por Orval contra el OpenAPI Rust.
5. **Cero parches:** no se replica deuda técnica del PHP. Si una decisión PHP fue subóptima, se rediseña.

---

## 1. Stack final consolidado

| Capa | Herramienta | Notas |
|------|-------------|-------|
| HTTP framework | `axum 0.7` | con `tower-http` (CORS, trace, compression) |
| OpenAPI | `utoipa 4` + `utoipa-swagger-ui 7` | contrato único; frontend sólo consume tipos generados |
| Async runtime | `tokio 1` | multi-thread |
| BD | `sqlx 0.8` (Postgres) | macros `query!`/`query_as!` (compile-time check) |
| Migraciones | `sqlx::migrate!` | versionadas, schema consolidado nuevo |
| Vector | `pgvector 0.4` (crate) + extensión Postgres `pgvector` | embeddings 128d |
| Trigram | extensión Postgres `pg_trgm` | búsqueda fuzzy |
| Cache | `deadpool-redis 0.18` + `redis 0.27` | rate limit, sesiones, cache feed, locks |
| JWT | `jsonwebtoken 9` | access (1h) + refresh (7d) |
| Hash password | `argon2 0.5` | reemplaza `wp_authenticate` |
| Validación | `validator 0.18` | derive macros |
| Errores dominio | `thiserror 2` | tipados |
| Errores app | `anyhow 1` | en boundaries internos |
| Logging | `tracing` + `tracing-subscriber` + `tracing-axum` | spans por request |
| Config | `dotenvy 0.15` + `figment 0.10` | env + TOML opcional |
| HTTP cliente externo | `reqwest 0.12` (rustls) | Stripe, Groq, OpenAI, FCM |
| WebSocket | `axum 0.7` (built-in WS) | consolidamos Bun → Axum WS (decisión técnica) |
| Stripe | `async-stripe 0.39` | Billing + Connect + webhooks |
| OAuth | `oauth2 4.4` | Google web/desktop/mobile |
| FCM | `reqwest` + `gcp_auth` | Firebase HTTP v1 + service-account |
| Web Push (VAPID) | `web-push 0.10` | self-hosted VAPID |
| Email | `lettre 0.11` | SMTP |
| Storage | `tokio::fs` (local) + `aws-sdk-s3 1.x` (opcional) | abstraído por trait |
| Audio decode | `symphonia 0.5` | duración + decodificación |
| FFT/DSP | `rustfft 6` + `ndarray 0.16` | BPM, key detection custom |
| FFmpeg | proceso externo (`tokio::process::Command`) + `ffmpeg-sidecar` opcional | conversión, waveform, MP3/FLAC export |
| ID corto | `nanoid 0.4` | reemplaza `GeneradorIdCorto.php` |
| Slugs | `slug 0.1` | URL-safe |
| Hash archivos | `sha2 0.10` + `tokio::io` | streaming SHA-256 |
| HMAC | `hmac 0.12` + `sha2` | tickets WS |
| Lint | `clippy` (`-D warnings -W clippy::pedantic`) | parte de `self-check` |
| **Frontend** | | |
| Build | `vite 6` + `react 18` + `typescript 5` | ya existe en `frontend/` |
| State server | `@tanstack/react-query 5` | cache server state |
| State client | `zustand 5` | state global UI |
| Codegen | `orval 8` | tags-split, `react-query` mode |
| **Submódulo agnóstico** | | |
| Glory framework Rust | `glory-rs/` (carpeta clonada) | recibe todo lo agnóstico (auth, ws, fcm, vapid, stripe abstractions, file upload, cache, config, logging, fixtures, codegen helpers) |

---

## 2. Arquitectura de carpetas destino

```
glory-rust-template/
  Cargo.toml                    # workspace
  src/                          # binario principal kamples-api
    main.rs                     # bootstrap: tracing, db, redis, router
    lib.rs
    config/                     # carga de env + tipos Config
    handlers/                   # axum handlers, agrupados por dominio
      auth/
      samples/
      colecciones/
      pagos/
      ...
    middleware/                 # auth, rate_limit, request_id
    repositories/               # acceso a datos (sqlx)
    services/                   # lógica de dominio
    models/                     # structs de dominio + DTOs
      ids.rs                    # newtypes UserId, SampleId, etc.
    errors/                     # AppError (thiserror) + IntoResponse
    domain/                     # tipos enum, validators, value-objects
    audio/                      # pipeline audio (BPM, key, embeddings)
    algorithm/                  # 6 señales, recomendación
    workers/                    # background tasks (cola IA, precomputador feed, sync)
    ws/                         # WebSocket handler + hub
  migrations/                   # SQLx migrations (schema consolidado)
  frontend/                     # SPA Vite (ya estructurada)
    src/
      api/                      # generado por Orval (NO TOCAR)
      features/                 # islands portadas → ahora componentes SPA
      components/
      hooks/
      stores/
      styles/
  glory-rs/                     # framework agnóstico (clonado, no submódulo en repo del proyecto)
    backend/
      auth/                     # JWT, OAuth helpers, password hashing
      websocket/                # hub multi-conexión, ticket HMAC
      cache/                    # Redis abstractions
      storage/                  # trait FileStorage + Local + S3 impls
      payments/                 # Stripe abstractions (no específico de Kamples)
      push/                     # FCM + Web Push
      email/                    # SMTP + templates
      config/                   # figment helpers
      errors/                   # base AppError pattern
      observability/            # tracing setup
      fixtures/                 # seeders genéricos
    frontend/                   # componentes UI agnósticos
  scripts/
    self-check.ps1
    check-roadmap.mjs
    generate-openapi.ps1        # corre el binario para emitir openapi.json y dispara orval
  Agente/
    completados/
    planes/
    documentacion/
    prevencion/
  roadmap.md
```

**Regla clara:** `glory-rs/` es agnóstico. NUNCA mete tipos de dominio Kamples (Sample, Coleccion, BPM, etc.). Si algo en `glory-rs/` se usaría sólo en este proyecto, vive en `src/` del proyecto.

---

## 3. Schema PostgreSQL consolidado (V1 base)

Las 74 migraciones legacy se consolidan en **una migración base limpia** + nuevas migraciones a partir de ahí. No replicamos la historia (consolidaciones, fixes, columnas que se añadieron y luego se borraron). El criterio es el **estado final** del schema en producción.

### 3.1 Tablas core

```
extensions: pgvector, pg_trgm

users
  id BIGSERIAL PK
  username TEXT UNIQUE NOT NULL                -- antes wp_user_login
  email TEXT UNIQUE NOT NULL
  password_hash TEXT NOT NULL                  -- argon2id
  email_verified_at TIMESTAMPTZ
  google_sub TEXT UNIQUE                       -- para OAuth
  display_name TEXT
  bio TEXT
  avatar_url TEXT
  city TEXT, country TEXT, website TEXT
  is_creator BOOL NOT NULL DEFAULT false
  stripe_customer_id TEXT UNIQUE
  stripe_connect_id TEXT UNIQUE
  paypal_email TEXT
  registration_ip INET
  is_suspended BOOL NOT NULL DEFAULT false
  suspension_reason TEXT
  is_blocked BOOL NOT NULL DEFAULT false
  block_reason TEXT
  role TEXT NOT NULL DEFAULT 'user'            -- user | moderator | admin
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
  deleted_at TIMESTAMPTZ

samples
  id BIGSERIAL PK
  short_id TEXT UNIQUE NOT NULL                -- nanoid 12 chars
  creator_id BIGINT NOT NULL REFERENCES users(id)
  slug TEXT UNIQUE NOT NULL
  title TEXT NOT NULL
  description TEXT
  audio_url TEXT NOT NULL
  audio_sha256 BYTEA UNIQUE NOT NULL           -- 32 bytes
  cover_url TEXT
  duration_seconds REAL NOT NULL
  bpm INT CHECK (bpm BETWEEN 30 AND 250)
  music_key SMALLINT                           -- 0..11 (C..B)
  scale TEXT CHECK (scale IN ('major','minor'))
  sample_type TEXT NOT NULL DEFAULT 'loop'     -- loop|one_shot|vocal|fx|preset
  is_premium BOOL NOT NULL DEFAULT false
  download_price NUMERIC(6,2) DEFAULT 2.99
  metadata JSONB NOT NULL DEFAULT '{}'         -- {genero, instrumentos, emocion, artista_vibes, tags}
  tags TEXT[] NOT NULL DEFAULT '{}'
  enriched_tags TEXT[] NOT NULL DEFAULT '{}'   -- recalculado por trigger
  status TEXT NOT NULL DEFAULT 'draft'         -- draft|published|rejected|deleted|pending_moderation
  rejection_reason TEXT
  embedding vector(128)
  published_at TIMESTAMPTZ
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
  deleted_at TIMESTAMPTZ

  INDEX gin (metadata jsonb_path_ops)
  INDEX gin (tags)
  INDEX gin (enriched_tags)
  INDEX (status, published_at DESC)
  INDEX (creator_id)
  INDEX (bpm) WHERE bpm IS NOT NULL
  INDEX (music_key, scale)
  INDEX gin (title gin_trgm_ops)
  INDEX hnsw (embedding vector_cosine_ops)     -- pgvector >= 0.5
```

### 3.2 Resto de tablas (resumen, ver detalle en cada migración)

- **collections, collection_samples** (M2M con `position`)
- **likes** (polimórfica: `target_type` + `target_id`, UNIQUE por user/target)
- **plays** (reproducciones; última por user/sample con `last_progress`)
- **downloads** (compras + tracking; UNIQUE por user/sample/transaction)
- **comments** (polimórficas: sample/post/article)
- **comment_likes**
- **posts** (publicaciones sociales)
- **reposts** (UNIQUE por user/post)
- **follows** (PK compuesta)
- **blocks** (bloqueo entre users)
- **conversations + messages** (chat 1:1)
- **notifications** (in-app, polimórficas con `actor_id` + `target_*`)
- **push_subscriptions** (Web Push VAPID)
- **fcm_tokens** (Android push)
- **subscriptions** (planes free/pro/premium)
- **transactions** (pagos + revenue share + idempotency_key)
- **free_codes + free_code_uses**
- **articles + article_likes**
- **artists + songs + sample_song_relations** (catálogo del scraper)
- **algo_state** (cache global/por-usuario del feed)
- **user_tag_scores** (afinidad tag por usuario)
- **ia_queue** (cola procesamiento Groq/OpenAI con backoff)
- **scraping_queue + scraping_log** (cola del scraper)
- **moderation_queue** (samples/comments/posts pendientes)
- **reports** (denuncias usuario→contenido/usuario)
- **legal_reports** (DMCA/copyright)
- **duplicate_pairs** (resolución manual)
- **sync_changelog** (offline sync mobile/desktop)
- **batch_jobs** (procesos masivos: backfill embeddings, recompute profiles)
- **audit_log** (acciones admin)

### 3.3 Triggers / funciones SQL

- `tg_recalc_enriched_tags()` — antes de INSERT/UPDATE en `samples`, recalcula `enriched_tags` a partir de `tags` + `metadata->'genero'` + `metadata->'instrumentos'` + `metadata->'emocion'` + `metadata->'artista_vibes'` + `metadata->'tags'`.
- `tg_update_updated_at()` — trigger genérico para todas las tablas con `updated_at`.

### 3.4 Vistas materializadas (opcional fase 2)

- `mv_trending_samples_24h` — refresh cada 5 min via worker.
- `mv_creator_stats` — descargas, ingresos, followers por creador, refresh cada 1h.

---

## 4. Mapeo PHP → Rust por dominio

### 4.1 Dominios identificados (orden de implementación)

| # | Dominio | Riesgo | Dependencias previas |
|---|---------|--------|----------------------|
| 1 | **Bootstrap + Config + DB pool + Tracing + OpenAPI scaffold** | bajo | — |
| 2 | **Schema base + migraciones** | medio | bootstrap |
| 3 | **Auth (email/pass + JWT + refresh + middleware)** | medio | schema users |
| 4 | **Users / Profile** | bajo | auth |
| 5 | **OAuth Google (web/desktop/mobile)** | medio | auth, users |
| 6 | **Audio storage + upload (multipart, hash, dedup, validación MIME)** | alto | auth |
| 7 | **Audio pipeline (FFmpeg, BPM, key, embeddings, MP3/FLAC, waveform)** | **MUY ALTO** | upload |
| 8 | **IA pipeline (Groq + OpenAI + cola + JsonRepairer + prompts)** | alto | audio pipeline |
| 9 | **Samples CRUD (read, search, edit, delete, slug)** | medio | upload + IA |
| 10 | **Algoritmo de descubrimiento (6 señales, cache, precomputador)** | **MUY ALTO** | samples + plays + tags |
| 11 | **Reproducciones (plays tracking, completion %)** | bajo | samples |
| 12 | **Likes, follows, blocks** | bajo | users + samples |
| 13 | **Colecciones (CRUD + M2M + merge + guardadas)** | medio | samples |
| 14 | **Comentarios (poli, threads, multimedia)** | medio | samples + posts |
| 15 | **Publicaciones sociales (feed, repost, likes)** | medio | users |
| 16 | **Mensajería (conversations + messages + multimedia)** | medio | users |
| 17 | **WebSocket hub (Axum WS + ticket HMAC + multi-device fanout)** | alto | auth |
| 18 | **Notificaciones (in-app + WS fanout + Web Push + FCM + email)** | alto | WS + push registration |
| 19 | **Pagos Stripe (checkout, portal, webhooks, Connect, idempotencia)** | alto | users + samples |
| 20 | **Códigos gratis** | bajo | descargas |
| 21 | **Descargas (tracking, ZIP, stream con range, límites por plan)** | medio | samples + pagos |
| 22 | **Moderación IA (4 capas, cola, panel admin)** | alto | IA pipeline |
| 23 | **Reportes (legales + bugs + contenido)** | bajo | users |
| 24 | **Blog (artículos + likes + comentarios + categorías)** | bajo | users |
| 25 | **Búsqueda global (samples + users + colecciones + canciones)** | medio | trigram + embeddings |
| 26 | **Catálogo canciones/artistas + relaciones sample-canción** | medio | samples + scraper |
| 27 | **Sync changelog (offline mobile/desktop)** | medio | todos los CRUDs |
| 28 | **SEO + sitemap dinámico** | bajo | samples + users + articles |
| 29 | **Dashboard creador + admin (stats agregadas)** | medio | transactions + plays + downloads |
| 30 | **Workers (cron) — Tokio interval tasks + locks Redis** | medio | Redis + cola IA + precomputador feed |
| 31 | **Scraper Python — adaptar para que escriba contra la nueva API/BD** | medio | samples + colas |
| 32 | **Mezclador (Tauri) — adaptar para nueva API + Orval client** | bajo | samples + colecciones |
| 33 | **Mobile WebView (Capacitor) — adaptar config + FCM + deep links** | bajo | frontend SPA + FCM |
| 34 | **Desktop (Tauri) — auto-updates, deep links** | bajo | frontend + endpoints versions |
| 35 | **Frontend SPA — re-cablear servicios manuales → Orval client + ajustar mounting** | medio | OpenAPI estable |
| 36 | **Despliegue Coolify (Dockerfile + compose + volumes + traefik labels)** | alto | todo lo anterior estable |

**Regla "lo más difícil primero":** dentro de los bloques de prerequisitos cumplidos, atacamos siempre primero el ítem de mayor riesgo (audio pipeline → algoritmo → moderación IA → pagos → WS+notif).

### 4.2 Ejemplo de mapeo (controlador → handler + service + repo)

PHP:
- `App/Kamples/Api/Controladores/SamplesController.php` → registra rutas WP REST.
- `App/Kamples/Database/Repositories/SamplesRepository.php` → wpdb queries.

Rust:
- `src/handlers/samples/mod.rs` (router) + `read.rs` / `write.rs` / `search.rs`.
- `src/services/samples_service.rs` (lógica + autorización).
- `src/repositories/samples_repo.rs` (`sqlx::query_as!` con structs `SampleRow`).
- `src/models/sample.rs` (`Sample` dominio + `SampleId(i64)` newtype + DTOs `SampleResponse`, `SamplePatch`).

Cada controlador PHP se mapea a:
- 1 módulo handlers (router + endpoints anotados con `utoipa::path`).
- 1 service (toda la lógica, sin tocar HTTP).
- 1+ repositorios (sólo SQL).

---

## 5. Decisiones técnicas resueltas

| Tema | Decisión | Justificación |
|------|----------|--------------|
| BD | Postgres único | Acordado con usuario. Simplifica deploy y consistencia. |
| WebSocket | **Axum WS integrado** (eliminamos Bun separado) | Menos infraestructura, mismo proceso, comparte auth/JWT, fanout en memoria + Redis pub/sub para multi-instancia. Bun queda como referencia pero no se despliega. |
| FFmpeg | Proceso externo + `tokio::process::Command` con `escape` de args | Robusto, mismo enfoque que PHP; `ffmpeg-sidecar` opcional para gestión binaria. |
| BPM/key | Rust puro: `symphonia` decode + `rustfft` + `ndarray` | Reescribir `DetectorBpm`/`DetectorTonalidad` en Rust. Mismo algoritmo (onset + autocorrelación + cromagramas). |
| Embeddings | Cálculo determinista en Rust (mismo schema 128d que el PHP) → `pgvector` | Compatibilidad con datos existentes si se migran. |
| pgvector | crate `pgvector 0.4` con feature `sqlx` | Soporta `Vector` con `query_as!` directamente. |
| IA Groq/OpenAI | `reqwest` + cliente con rotación de 3 keys, retry exponencial, fallback chain | Mismos prompts y modelos. `JsonRepairer` se reescribe en Rust con `regex` o un parser tolerante. |
| Stripe | `async-stripe 0.39` (alto nivel) + verificación HMAC manual de webhooks | Mantenimiento + tipado fuerte. |
| OAuth Google | `oauth2 4.4` + verificación de ID token (web) con JWKS de Google (`jsonwebtoken` + `reqwest`) | Soporta los 3 flujos. |
| FCM | `reqwest` + `gcp_auth` (OAuth service account) → HTTP v1 API | Sin SDK oficial Rust. |
| Web Push | `web-push 0.10` | Maduro, soporta VAPID. |
| Email | `lettre 0.11` con SMTP async | SendGrid SMTP funciona idéntico. |
| Cache/Redis | `deadpool-redis 0.18` | Pool async. Patrón cache-aside, locks SETNX, rate limit con INCR+EXPIRE, pub/sub para WS multi-instancia. |
| Storage | trait `FileStorage` con impls `LocalFs` + `S3`. Por defecto local en dev, S3 en prod (vía env). | Abstrae para que controladores y workers sean agnósticos. |
| Idempotencia upload | header `X-Idempotency-Key` + tabla `upload_idempotency` (key, user_id, result_json, expires_at) | Igual que PHP. |
| Soft delete | columna `deleted_at` + `WHERE deleted_at IS NULL` en queries (helper en repos) | Sin extensión, Rust lo gestiona. |
| Auth password | argon2id (`argon2 0.5`) | Reemplazo seguro de `wp_hash_password`. Migración: si en futuro hay datos legacy WP, dual-hash + rehash on login. |
| Sesiones | JWT access (1h) + refresh (7d) en Redis (set + revoke) | Cookies HttpOnly Secure SameSite=Lax. |
| CSRF | Sólo en endpoints sensibles via doble-submit cookie + header. Bearer tokens son inmunes en mayoría. | OWASP. |
| Rate limit | middleware Axum con Redis (`INCR`+`EXPIRE`), buckets por IP + por user_id | Reemplaza rate limiters PHP dispersos. |
| OpenAPI emit | `openapi.json` se emite por el binario en startup en modo dev y por comando CLI `kamples-api openapi > openapi.json` para CI/Orval | Trazable, commiteado. |
| Codegen | Orval `mode: tags-split` → un archivo TS por tag OpenAPI | Regla del protocolo. |
| Workers | tareas Tokio con loops + lock Redis por nombre (evita doble ejecución multi-instancia) + tabla `worker_runs` para auditoría | Reemplaza WP Cron. |
| Tracing | `tracing` + `tracing-subscriber` JSON en prod, pretty en dev. Span por request con `request_id`. | Observabilidad. |
| Errores | `AppError` enum con `thiserror` + `IntoResponse` que traduce a status apropiado y `{error, code, request_id}` JSON | Cero panics en prod (`#![deny(clippy::unwrap_used)]` en módulos críticos). |
| Validación | `validator` derive + custom validators para BPM/key/scale/slug | Validación en boundary HTTP. |

---

## 6. Plan de fases (cada fase = N tareas atómicas en roadmap)

Cada fase produce código compilable, tests verdes y endpoints documentados en OpenAPI. **Cada tarea es un commit y se cierra siguiendo los 10 pasos del protocolo.**

### Fase 0 — Bootstrap del proyecto Rust
- 174A-2: Verificar `Cargo.toml`, dependencias base (axum, sqlx, tokio, tracing, utoipa, dotenvy, jsonwebtoken, argon2, validator, thiserror, anyhow, redis/deadpool, reqwest, sha2, hmac, nanoid, slug, pgvector).
- 174A-3: Crear estructura de carpetas (`config/`, `errors/`, `middleware/`, `handlers/`, `services/`, `repositories/`, `models/`, `domain/`, `audio/`, `algorithm/`, `workers/`, `ws/`).
- 174A-4: Implementar `AppError` global + `IntoResponse` + middleware `request_id` + `tracing-subscriber`.
- 174A-5: Pool SQLx + Redis pool + `AppState` compartido + extractor de estado.
- 174A-6: OpenAPI scaffold (`utoipa`) + Swagger UI en `/docs` + comando CLI `--emit-openapi`.

### Fase 1 — Schema base PostgreSQL
- 174A-7: Migración `0001_init_extensions.sql` (`pgvector`, `pg_trgm`).
- 174A-8: Migración `0002_users.sql` + trigger `tg_update_updated_at`.
- 174A-9: Migración `0003_samples.sql` + trigger `tg_recalc_enriched_tags` + índices (gin/trgm/hnsw).
- 174A-10: Migración `0004_collections.sql` + `collection_samples`.
- 174A-11: Migración `0005_social_graph.sql` (follows, blocks, likes, plays, downloads, comments, comment_likes, posts, reposts).
- 174A-12: Migración `0006_messaging.sql` (conversations, messages).
- 174A-13: Migración `0007_notifications_push.sql` (notifications, push_subscriptions, fcm_tokens).
- 174A-14: Migración `0008_payments.sql` (subscriptions, transactions, free_codes, free_code_uses, upload_idempotency).
- 174A-15: Migración `0009_blog.sql` (articles, article_likes).
- 174A-16: Migración `0010_catalog.sql` (artists, songs, sample_song_relations).
- 174A-17: Migración `0011_algo.sql` (algo_state, user_tag_scores, batch_jobs, ia_queue, scraping_queue, scraping_log, moderation_queue, reports, legal_reports, duplicate_pairs, sync_changelog, audit_log).

### Fase 2 — Auth y usuarios
- 174A-18: Module `glory-rs/backend/auth` (password hashing argon2, JWT issue/verify, refresh tokens en Redis).
- 174A-19: Middleware `AuthLayer` (extrae `CurrentUser`, valida JWT, soporta cookie + header).
- 174A-20: Handlers `POST /auth/register`, `POST /auth/login`, `POST /auth/logout`, `POST /auth/refresh`. Tests integración con BD.
- 174A-21: OAuth Google ID token (`POST /auth/google`) — verificación JWKS, crea/asocia user.
- 174A-22: OAuth Google PKCE desktop (`POST /auth/google/desktop`).
- 174A-23: OAuth Google mobile callback (`POST /auth/google/mobile-callback` + `GET /auth/google/mobile-status`).
- 174A-24: Endpoints perfil: `GET /me`, `PATCH /me`, `GET /users/{username}`.
- 174A-25: Suspensión / bloqueo / eliminación (admin).

### Fase 3 — Storage + uploads
- 174A-26: Trait `FileStorage` + impl `LocalFs` (`glory-rs/backend/storage`).
- 174A-27: Impl `S3Storage` (opcional, feature-gated).
- 174A-28: Handler `POST /samples/check-duplicate` (SHA-256 streaming).
- 174A-29: Handler `POST /samples/upload` (multipart + idempotency + validación MIME + límites + persistencia inicial `status=pending_processing`).

### Fase 4 — Audio pipeline (LO MÁS DIFÍCIL)
- 174A-30: Módulo `audio/ffmpeg.rs` (detect, duration, convert MP3/FLAC, waveform peaks).
- 174A-31: Módulo `audio/bpm.rs` (decode con symphonia, onset detection, autocorrelación).
- 174A-32: Módulo `audio/key.rs` (cromagramas, detección key + scale).
- 174A-33: Módulo `audio/embeddings.rs` (vector 128d determinista — mismo schema PHP).
- 174A-34: Service `audio_pipeline.rs` orquestador (semáforo concurrencia 2, etapas con timeouts, persistencia parcial).
- 174A-35: Worker async que consume `pending_processing` y ejecuta pipeline.
- 174A-36: Tests con audio fixtures (WAV cortos: 4 bars 120 BPM en C major).

### Fase 5 — IA pipeline
- 174A-37: `glory-rs/backend/llm` (no): mejor en `src/audio/ia/` por ser específico — cliente Groq con rotación de 3 keys, retry, fallback chain de modelos.
- 174A-38: Cliente OpenAI (fallback final).
- 174A-39: `JsonRepairer` (regex + parser tolerante).
- 174A-40: `prompts.rs` (constantes parametrizadas, mismas que PHP).
- 174A-41: Service `ia_service.rs` (analiza nombre + descripción + BPM + key → tags/géneros/instrumentos/emociones).
- 174A-42: Worker `ia_queue_worker.rs` (consume `ia_queue`, respeta rate limits 429 con backoff exponencial).
- 174A-43: Service `moderation_service.rs` (4 capas: pre-filtro local, IA categorización, decisión, panel admin).

### Fase 6 — Samples CRUD + búsqueda
- 174A-44: Repo + service + handlers `GET /samples` (paginación, filtros: BPM, key, type, tags, premium, creator).
- 174A-45: `GET /samples/{slug}` + `GET /samples/random`.
- 174A-46: `PATCH /samples/{slug}` + `DELETE /samples/{slug}` (owner check, soft-delete).
- 174A-47: Búsqueda fuzzy: trigram en `title` + filtros combinados.
- 174A-48: Búsqueda por similitud (pgvector cosine) — endpoint `GET /samples/{id}/similar`.

### Fase 7 — Algoritmo de descubrimiento (LO MÁS DIFÍCIL #2)
- 174A-49: Módulo `algorithm/signals.rs` con las 6 señales (similitud contenido, comportamiento, contexto, tendencias, grafo social, novedad). Pesos exactos: 0.25/0.25/0.15/0.15/0.10/0.10.
- 174A-50: Módulo `algorithm/profile.rs` (PerfilUsuario: tags afinidad, historial, bloqueos, TTL 30min).
- 174A-51: Módulo `algorithm/candidates.rs` (SelectorCandidatos: filtra estados/bloqueos).
- 174A-52: Módulo `algorithm/recommender.rs` (MotorRecomendacion: scoring + ranking + cache stale-while-revalidate 6h).
- 174A-53: Módulo `algorithm/precompute.rs` (PrecomputadorFeed: bulk LIMIT*3 OFFSET 0, split en memoria, cache por página).
- 174A-54: Service `tag_affinity.rs` (recalc por interacciones).
- 174A-55: Worker `algo_planner.rs` (planificador, recálculo por usuario activo).
- 174A-56: Endpoints `GET /feed?page=N`, `GET /samples/random?seed=X`, `GET /me/feed`.
- 174A-57: `algo_timing` métricas + endpoint `GET /admin/algo-timing` (admin only).

### Fase 8 — Reproducciones, likes, follows, downloads
- 174A-58: `POST /samples/{id}/play` (registra/actualiza reproducción + porcentaje + completada).
- 174A-59: Likes polimórficos (`POST/DELETE /likes/{type}/{id}`).
- 174A-60: Follows + Blocks (`POST/DELETE /users/{id}/follow`, `block`).
- 174A-61: Downloads (estructura + límites por plan + tracking IP).
- 174A-62: Endpoint `POST /downloads/{sample_id}/zip` (multi-sample).
- 174A-63: Endpoint stream con range (`GET /downloads/{sample_id}/stream`).

### Fase 9 — Colecciones y sociales
- 174A-64: CRUD colecciones + M2M con orden.
- 174A-65: Endpoint merge (`POST /collections/merge`).
- 174A-66: Saved collections (favoritos de otros).
- 174A-67: Posts (publicaciones sociales) CRUD + reposts + likes.
- 174A-68: Comments polimórficos + comment likes + multimedia.

### Fase 10 — Mensajería + WebSocket
- 174A-69: Módulo `glory-rs/backend/websocket` (hub `Arc<DashMap<UserId, Vec<SocketTx>>>`, ticket HMAC, auth on connect, ping/pong, fanout).
- 174A-70: Handler `GET /ws` (upgrade) + `GET /ws/ticket`.
- 174A-71: Conversations + messages CRUD + multimedia.
- 174A-72: WS events: `mensaje_nuevo`, `notificacion`, `new_like`, `new_comment`, `new_repost`, `new_follower`, `actualizacion_*`.
- 174A-73: Multi-instancia: Redis pub/sub para fanout entre nodos.

### Fase 11 — Notificaciones (5 canales)
- 174A-74: Tabla notifications + service.
- 174A-75: Web Push (VAPID): registro + envío.
- 174A-76: FCM Android: service-account JSON + envío v1.
- 174A-77: Email: SMTP + plantillas (welcome, compra, notificaciones opt-in).
- 174A-78: Pipeline integrado `notify(user, event)` → fanout in-app + WS + push + email (según preferencias).

### Fase 12 — Pagos
- 174A-79: Wrapper Stripe (`async-stripe`) en `src/services/stripe_service.rs` (no agnóstico — específico de planes Kamples).
- 174A-80: Endpoints planes: `GET /pagos/planes`.
- 174A-81: Checkout suscripción + checkout sample + portal cliente.
- 174A-82: Webhook (`POST /pagos/webhook`) con verificación HMAC + idempotencia.
- 174A-83: Connect onboarding + revenue share.
- 174A-84: Códigos gratis (CRUD + uso + tracking).

### Fase 13 — Reportes, blog, búsqueda global, catálogo
- 174A-85: Reportes (legales, contenido, errores).
- 174A-86: Blog (artículos CRUD + categorías + likes + comentarios + feed editorial).
- 174A-87: Búsqueda global (`GET /search?q=...&types=samples,users,collections,songs`).
- 174A-88: Canciones / artistas / relaciones sample-canción CRUD + lectura.

### Fase 14 — Moderación, admin, dashboard
- 174A-89: Panel admin endpoints (stats, queues, users gestión).
- 174A-90: Dashboard creador (descargas, ingresos, trending).
- 174A-91: Sync changelog (`GET /sync/changelog?since=...`).

### Fase 15 — Workers (cron) + observabilidad
- 174A-92: Worker `cleanup_expired_subscriptions`.
- 174A-93: Worker `precompute_feeds`.
- 174A-94: Worker `process_ia_queue` (90s).
- 174A-95: Worker `process_scraping_queue`.
- 174A-96: Worker `recompute_user_profiles`.
- 174A-97: Métricas Prometheus opcionales.

### Fase 16 — SEO
- 174A-98: `GET /sitemap.xml` dinámico (samples, articles, users; paginado).
- 174A-99: Endpoint metadata (`GET /seo/sample/{slug}` para SSR si aplica).

### Fase 17 — Frontend SPA
- 174A-100: Configurar `frontend/orval.config.ts` apuntando a `openapi.json`. `mode: tags-split`.
- 174A-101: Generar primer cliente Orval. Verificar `npm run type-check`.
- 174A-102: Migrar islands desde `App/React/islands/` a `frontend/src/features/{dominio}/` como componentes SPA. Stores Zustand y componentes UI se copian tal cual.
- 174A-103: Integrar React Router para navegación SPA (reemplaza WordPress page routing).
- 174A-104: Reemplazar servicios manuales (`apiSamples.ts`, etc.) por hooks generados de Orval/React Query.
- 174A-105: Hook `useWebSocket()` actualizado para `wss://kamples.com/ws` con ticket Axum.
- 174A-106: Reemplazar `useAuth` por sesión vs nuevo backend.
- 174A-107: Smoke test full SPA: login → feed → reproducir → like → seguir → comentar → mensaje.

### Fase 18 — Scraper / Mezclador / Mobile / Desktop
- 174A-108: Adaptar `kamples-scraper/` (Python) — usa nueva API REST para crear samples y consultar cola.
- 174A-109: Adaptar `Mezclador/` (Tauri DAW) — login + cliente API + librería samples.
- 174A-110: Adaptar `mobile/` (Capacitor Android WebView) — config nuevo dominio + FCM + deep links.
- 174A-111: Adaptar `desktop/` (Tauri) — auto-updates contra `GET /versions/desktop/latest`.

### Fase 19 — Despliegue
- 174A-112: Dockerfile multi-stage Rust (cargo-chef cache + binario release).
- 174A-113: Dockerfile frontend (Vite build → nginx alpine static).
- 174A-114: `docker-compose.yml` Coolify (api, web, postgres pgvector, redis, traefik labels, volumes).
- 174A-115: Variables de entorno + secrets management.
- 174A-116: Migración inicial automática + healthcheck.
- 174A-117: Deploy via `coolify-manager-rs` (un solo servicio o múltiples + postgres+redis externos).
- 174A-118: Smoke test producción + rollback procedure.

---

## 7. Qué va a `glory-rs/` (agnóstico)

| Módulo | Qué hace | Por qué es agnóstico |
|--------|----------|---------------------|
| `auth` | Argon2id, JWT issue/verify, refresh tokens Redis, OAuth Google helpers | Cualquier app necesita esto |
| `websocket` | Hub `DashMap<UserId, Vec<Tx>>` + ticket HMAC + Redis pub/sub | Reutilizable |
| `cache` | Wrapper Redis (get/set/setnx/incr+expire/json get/set), pattern cache-aside | Genérico |
| `storage` | Trait `FileStorage` + LocalFs + S3 | Genérico |
| `payments::stripe` | Wrapper checkout + portal + webhook signature + customer | Las primitivas son agnósticas; los planes específicos no. |
| `push::fcm` + `push::web_push` | Envío Web Push VAPID + FCM HTTP v1 | Genérico |
| `email` | SMTP + render template Tera/Askama | Genérico |
| `errors` | `BaseAppError` trait + macros `IntoResponse` | Patrón |
| `observability` | Setup tracing + JSON formatter + request_id middleware | Patrón |
| `openapi` | Helpers `utoipa` + emit JSON CLI | Genérico |
| `config` | Carga .env + figment + validation | Genérico |
| `fixtures` | Trait `Fixture` + loader TOML/JSON | Genérico |
| `rate_limit` | Middleware Axum + Redis bucket | Genérico |

Lo que NO va: dominio de samples, algoritmo, BPM, embeddings, prompts IA, planes Kamples, scraper-específico.

---

## 8. Estrategia de datos

**Greenfield asumido por defecto.** No migramos datos de producción WordPress en esta fase. La nueva instancia arranca con BD vacía + seeders opcionales.

**Si se decide migrar datos** (futura tarea):
- Script Rust one-shot que lea de Postgres legacy (mismo motor) y reescriba en el nuevo schema mapeando columnas.
- Re-hash de passwords NO posible (argon2 vs phpass) → forzar reset por email a todos los usuarios o esquema dual-hash en login (compatibilidad temporal).
- IDs: mantener BIGINT, mapear con tabla `legacy_ids` si necesario.

---

## 9. Riesgos y mitigaciones

| Riesgo | Impacto | Mitigación |
|--------|---------|-----------|
| BPM/key detection en Rust no alcanza calidad PHP | Alto | Mantener fallback opcional vía CLI `aubio`/`librosa` como proceso externo. Calibrar contra dataset de prueba con BPM conocidos. |
| Migración sin datos perdería usuarios reales | Alto | Greenfield aceptado. Si producción activa: fase específica de migración + dual-hash auth. |
| pgvector versión <0.5 sin HNSW | Medio | Requerir Postgres 16 + pgvector 0.7+. Documentar en `docker-compose`. |
| Stripe webhooks idempotencia | Alto | Tabla `webhook_events(id, event_id UNIQUE, processed_at)`. Procesar dentro de transacción. |
| WebSocket multi-instancia | Medio | Redis pub/sub canal `ws:user:{id}` con todos los nodos suscritos. |
| Groq rate limits inesperados | Medio | Cola con backoff + rotación 3 keys + fallback OpenAI + métricas. |
| FFmpeg no instalado en imagen | Alto | Dockerfile incluye `apt-get install -y ffmpeg`. Healthcheck verifica. |
| Frontend rompe al cambiar contrato | Alto | OpenAPI commiteado + Orval regenera + `npm run type-check` en self-check. CI bloquea PRs que no compilen. |
| Pérdida de SEO por SPA pura | Medio | Pre-render server-side de páginas críticas (samples, perfiles, artículos) con worker que cachea HTML, o adoptar Next/SSR si hace falta. **Decisión postergada a Fase 16.** |
| Integración mobile WebView con cookies cross-origin | Medio | Usar `Authorization: Bearer` en lugar de cookies en mobile. JWT se persiste en Capacitor Storage. |

---

## 10. Definición de "hecho" por fase

Cada fase está completa cuando:
1. Todas sus tareas atómicas tienen commit + push individual.
2. Todas archivadas en `Agente/completados/tareas-YYYY-MM-DD.md` con plantilla protocolar.
3. `cargo check && cargo clippy -- -D warnings && cargo test` verde.
4. `npm run type-check` (frontend) verde si la fase tocó frontend.
5. OpenAPI regenerado y commiteado.
6. Documentación actualizada en `Agente/documentacion/{categoria}/`.
7. Lecciones aprendidas registradas en `Agente/lecciones/lecciones-aprendidas.md` y comentarios del código.

---

## 11. Estado actual

- [x] 174A-1 Planificación (este documento).
- [ ] 174A-2..174A-118 (118 tareas atómicas distribuidas en 19 fases).

Próximo paso: cerrar 174A-1, archivarla, registrar las tareas atómicas en `roadmap.md` y comenzar 174A-2.

---

## 12. Convenciones de tarea

- Cada tarea = `174A-N` (incrementando), o cuando cambie el día se reinicia con la fecha (`184A-1`, etc.).
- Cada tarea = un commit. Mensaje: `174A-N: descripcion breve`.
- Cuando una tarea descubre subtareas no previstas, se añaden al final del roadmap como `174A-N.a`, `174A-N.b`.


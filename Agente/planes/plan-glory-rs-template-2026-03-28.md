# Plan: Glory-RS como Framework Reutilizable — 2026-03-28

## Contexto

El cliente quiere que cada nuevo restaurante tenga su propia instancia desplegada.
El proyecto actual debe servir como plantilla reutilizable. Las partes genéricas
se extraen a glory-rs (submodulo) para no reescribir en cada despliegue.

## Estado actual de glory-rs/

- Solo frontend: 5 componentes React (Boton, Input, Select, Textarea, Modal)
- CSS basado en variables
- Peer deps: react>=18, lucide-react>=0.400
- Alias: @glory/\* → ../glory-rs/frontend/

## Módulos genéricos identificados (backend Rust)

### Prioridad 0 — Extraíbles sin riesgo

| Módulo                  | Ubicación actual  | Complejidad |
| ----------------------- | ----------------- | ----------- |
| AppError + IntoResponse | src/errors/mod.rs | BAJA        |
| AppConfig + from_env    | src/config/mod.rs | BAJA        |
| AppState (struct)       | src/lib.rs        | BAJA        |

### Prioridad 1 — Requieren adaptación menor

| Módulo                     | Ubicación actual               | Complejidad |
| -------------------------- | ------------------------------ | ----------- |
| AuthService (JWT + Argon2) | src/services/auth.rs           | MEDIA       |
| AuthUser extractor         | src/middleware/auth.rs         | BAJA        |
| ApiKeyAuth extractor       | src/middleware/api_key_auth.rs | BAJA        |
| EmailService (Lettre)      | src/services/email.rs          | MEDIA       |
| User model + DTOs          | src/models/user.rs             | BAJA        |
| UserRepository             | src/repositories/user.rs       | MEDIA       |
| SecurityAddon (utoipa)     | src/handlers/mod.rs            | BAJA        |

### Prioridad 2 — Patrones extraíbles como traits/genéricos

| Módulo                  | Descripción                         |
| ----------------------- | ----------------------------------- |
| Repository pattern base | Trait/macro para CRUD genérico      |
| Service pattern base    | Convention pattern                  |
| Handler composition     | create_router() base + api_routes() |
| OpenAPI setup           | utoipa derive setup + swagger UI    |
| SPA fallback            | ServeDir/ServeFile configuration    |
| CORS config             | Standard CorsLayer setup            |

## Módulos que deben PERMANECER en el proyecto

- Reservas, Ventas, Gastos (core restaurant)
- Clientes, Etiquetas (CRM)
- Plano de Sala (layout físico)
- Campañas, Plantillas WhatsApp (marketing)
- Recordatorios (scheduler)
- Dashboard (KPIs)
- Chatbot API (integración externa)

## Fases de implementación

### Fase 1: Setup (ACTUAL)

- [ ] Crear glory-rs/backend/ como crate Rust
- [ ] Mover AppError, ErrorResponse, IntoResponse
- [ ] Mover AppConfig, SmtpConfig, from_env()
- [ ] Mover AppState (genérico con pool + jwt_secret + config)
- [ ] Re-exportar en main project para retrocompatibilidad

### Fase 2: Auth + Email

- [ ] Mover AuthService + middleware
- [ ] Mover EmailService
- [ ] Mover User model + repository
- [ ] Mover migrations base (users table)

### Fase 3: Framework de composición

- [ ] Crear create_router() genérico
- [ ] SecurityAddon (bearer + API key)
- [ ] SPA fallback config
- [ ] CORS preset

### Fase 4: Frontend

- [ ] Evaluar qué componentes shadcn personalizados son reutilizables
- [ ] Mover hooks genéricos (useFormValidation, etc.)
- [ ] Template de Orval config
- [ ] Template de Vite config

### Fase 5: Tooling

- [ ] Script para crear nuevo proyecto desde template
- [ ] Template de docker-compose / Coolify config
- [ ] Template de CI/CD

## Estrategia de migración (sin romper proyecto actual)

1. Crear crate en glory-rs/backend
2. Copiar código genérico (NO mover — copiar primero)
3. Hacer que main project dependa de glory-rs via path
4. Reemplazar imports en main (`use crate::errors` → `pub use glory_backend::errors`)
5. Verificar cargo check + clippy
6. Eliminar código duplicado del main project
7. Commit + test

## Riesgos

- SQLx offline cache (.sqlx/) depende de queries exactos — mover modelos puede invalidar cache
- El submodulo es un repo git separado — los cambios requieren commit + push en ambos repos
- Las migrations base (users) deben estar tanto en glory-rs como en cada proyecto

## Decisiones de diseño

- glory-rs/backend es un **crate library** (lib), no un binario
- Feature flags para módulos opcionales (email, api-key-auth)
- El proyecto principal es el binario que compone los módulos
- Naming: `glory-backend` (crate name)

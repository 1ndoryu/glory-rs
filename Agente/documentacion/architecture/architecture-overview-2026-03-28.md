# Arquitectura Glory-RS Template — 2026-03-28

## Visión general
Plataforma de gestión de restaurantes con backend Rust (Axum) + frontend React.
Diseñada para ser deployable como instancia independiente por restaurante.

## Capas del backend

```
HTTP Request
     │
     ▼
┌──────────────────────────────────┐
│  Handlers (src/handlers/)        │  REST endpoints + OpenAPI docs
│  Middleware (src/middleware/)     │  JWT auth, API key auth
├──────────────────────────────────┤
│  Services (src/services/)        │  Lógica de negocio
├──────────────────────────────────┤
│  Repositories (src/repositories/)│  Acceso a BD (SQLx)
├──────────────────────────────────┤
│  Models (src/models/)            │  Structs, DTOs, validación
├──────────────────────────────────┤
│  PostgreSQL (sqlx migrations)    │  Schema tipado
└──────────────────────────────────┘
```

## Módulos por dominio

### Genéricos (reutilizables entre proyectos)
- **Auth**: JWT login/register, password reset, AuthUser extractor
- **Email**: SMTP async con Lettre, fallback a logging
- **Config**: AppConfig desde variables de entorno
- **Errors**: AppError enum → HTTP status + JSON response
- **API Keys**: Autenticación para integraciones externas (chatbot)

### Específicos de restaurante
- **Reservas**: CRUD + filtros turno/estado, conteo, resumen mensual
- **Ventas/Gastos**: Registro financiero con IVA, categorías, pagos
- **Clientes**: CRM con etiquetas, merge de duplicados
- **Plano de Sala**: Constructor de mesas/zonas, ocupación
- **Marketing**: Campañas multi-canal, plantillas WhatsApp
- **Recordatorios**: Scheduler automático de reservas
- **Dashboard**: KPIs económicos y de reservas
- **Chatbot API**: Endpoints para chatbot externo (disponibilidad, reservas)

## Patrones de código

### Handler
```rust
#[utoipa::path(post, path = "/api/recurso", tag = "Tag", ...)]
pub async fn crear(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CrearRequest>,
) -> Result<(StatusCode, Json<Modelo>), AppError> {
    req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let resultado = MiService::create(&state.pool, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(resultado)))
}
```

### Service
```rust
pub struct MiService;
impl MiService {
    pub async fn create(pool: &PgPool, user_id: Uuid, req: Request) -> Result<T, AppError> {
        let data = MiRepository::create(pool, &mapped_data).await?;
        Ok(data)
    }
}
```

### Repository
```rust
pub struct MiRepository;
impl MiRepository {
    pub async fn create(pool: &PgPool, data: &NuevoItem) -> Result<Item, sqlx::Error> {
        sqlx::query_as!(Item, "INSERT INTO items (...) VALUES (...) RETURNING *", ...)
            .fetch_one(pool).await
    }
}
```

### Model
```rust
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct Modelo { ... }

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CrearModeloRequest { ... }
```

## Autenticación
- **JWT**: Header `Authorization: Bearer <token>`, extractor `AuthUser`
- **API Key**: Header `X-API-Key`, SHA-256 hash en BD, extractor `ApiKeyAuth`

## Frontend
- React 18 + TypeScript + Vite
- Tailwind CSS v4 + shadcn/ui
- Orval genera clientes API desde OpenAPI spec (tags-split mode)
- ThemeProvider custom (dark/light/system, localStorage)
- Alias: `@/*` → `src/`, `@glory/*` → `../glory-rs/frontend/`

## Deploy
- Docker vía Coolify
- `scripts/deploy-server.sh` para SSH directo
- Variables requeridas: `DATABASE_URL`, `JWT_SECRET`, `APP_URL`
- Frontend se compila a `static/` y se sirve con SPA fallback

# Framework Glory-RS — Documentación Completa

## Descripción
Glory-RS es un framework interno reutilizable diseñado para construir plataformas de gestión web. Combina:
- **Backend (Rust):** Crate `glory-backend` con error handling, configuración y patrones base para Axum + SQLx + PostgreSQL
- **Frontend (React):** Componentes UI atómicos agnósticos con CSS variables

## Arquitectura

```
glory-rs/
├── backend/                  # Crate Rust (library)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs            # Re-exporta módulos públicos
│       ├── errors.rs         # AppError → HTTP status + JSON
│       └── config.rs         # AppConfig + SmtpConfig desde env vars
├── frontend/
│   ├── componentes/
│   │   └── ui/
│   │       ├── Boton.tsx
│   │       ├── Input.tsx
│   │       ├── Select.tsx
│   │       ├── Textarea.tsx
│   │       ├── Modal.tsx
│   │       └── index.ts      # Barrel export
│   └── estilos/
│       └── Componentes.css   # Design system con CSS vars
├── package.json              # Peer deps: react ≥18, lucide-react ≥0.400
└── README.md
```

## Backend — `glory-backend`

### Dependencias
```toml
[dependencies]
axum = "0.7"
sqlx = "0.8"
serde = { version = "1", features = ["derive"] }
thiserror = "1"
utoipa = { version = "4", features = ["axum_extras"] }
tracing = "0.1"
```

### `AppError` — Error handling unificado
Enum con variantes mapeadas a HTTP status codes:

| Variante | HTTP Status | Uso |
|----------|-------------|-----|
| `NotFound(String)` | 404 | Recurso no encontrado |
| `BadRequest(String)` | 400 | Input inválido |
| `Unauthorized` | 401 | Sin credenciales o inválidas |
| `Conflict(String)` | 409 | Colisión de datos |
| `Internal(String)` | 500 | Error interno (loguea detalles) |
| `Database(sqlx::Error)` | 500 | Error de BD (loguea, no expone) |
| `Validation(String)` | 422 | Validación fallida |

Implementa `IntoResponse` de Axum → respuesta JSON automática:
```json
{
  "error": "not_found",
  "message": "Cliente no encontrado"
}
```

### `AppConfig` — Configuración desde entorno
Carga todo desde variables de entorno:

| Variable | Requerida | Default | Descripción |
|----------|-----------|---------|-------------|
| `DATABASE_URL` | Sí | — | PostgreSQL connection string |
| `JWT_SECRET` | Sí | — | Secreto para firmar JWT |
| `HOST` | No | `127.0.0.1` | Bind address |
| `PORT` | No | `3000` | Puerto del servidor |
| `APP_URL` | No | `http://localhost:5173` | URL base de la app |
| `SMTP_HOST` | No | — | Host SMTP |
| `SMTP_PORT` | No | — | Puerto SMTP |
| `SMTP_USER` | No | — | Usuario SMTP |
| `SMTP_PASSWORD` | No | — | Contraseña SMTP |
| `SMTP_FROM_EMAIL` | No | `noreply@app.com` | Email remitente |
| `SMTP_FROM_NAME` | No | `App` | Nombre remitente |

Si las variables SMTP no están presentes, `smtp` es `None` y el sistema funciona sin email.

### Uso en proyecto

```toml
# Cargo.toml del proyecto
[dependencies]
glory-backend = { path = "glory-rs/backend" }
```

```rust
use glory_backend::errors::AppError;
use glory_backend::config::AppConfig;

let config = AppConfig::from_env()?;
```

---

## Frontend — Componentes UI

### Componentes disponibles

#### `Boton`
```typescript
<Boton
  variante="primario" | "secundario" | "peligro" | "exito" | "fantasma"
  tamano="sm" | "md" | "lg"
  cargando={boolean}
  completo={boolean}  // full-width
  deshabilitado={boolean}
  onClick={handler}
>
  Texto
</Boton>
```

#### `Input`
```typescript
<Input
  etiqueta="Nombre"
  tipo="text" | "email" | "password" | "number" | ...
  valor={string}
  onChange={handler}
  error="Mensaje de error"
  tamano="sm" | "md" | "lg"
  requerido={boolean}
/>
```

#### `Select`
```typescript
<Select
  etiqueta="Categoría"
  valor={string}
  onChange={handler}
  opciones={[{ valor: "1", texto: "Bebidas" }]}
  error="Seleccione una opción"
  tamano="sm" | "md" | "lg"
/>
```

#### `Textarea`
```typescript
<Textarea
  etiqueta="Notas"
  valor={string}
  onChange={handler}
  filas={4}
  error="Campo requerido"
  tamano="sm" | "md" | "lg"
/>
```

#### `Modal`
```typescript
<Modal
  abierto={boolean}
  onCerrar={handler}
  titulo="Crear Reserva"
  tamano="sm" | "md" | "lg"
>
  {children}
</Modal>
```
Cierra con Escape, click fuera. En mobile: bottom-sheet animado.

### Integración

**Vite config:**
```typescript
resolve: {
  alias: {
    '@glory': path.resolve(__dirname, '../glory-rs/frontend'),
  }
}
```

**TSConfig:**
```json
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": { "@glory/*": ["../glory-rs/frontend/*"] }
  }
}
```

**Uso:**
```typescript
import { Boton, Input, Modal } from '@glory/componentes/ui';
```

### Design System
CSS variables en `estilos/Componentes.css`:
- Colores, espaciados, tipografía definidos como custom properties
- Soporte dark/light mode
- Animaciones (Modal enter/leave)

---

## Módulos pendientes de extracción (plan de evolución)

El proyecto restaurante contiene módulos genéricos que se extraerán al framework:

### Prioridad 1 — Auth + Email
- `AuthService`: JWT (jsonwebtoken) + Argon2 password hashing
- `AuthUser` extractor: middleware Axum para verificar JWT en header
- `ApiKeyAuth` extractor: autenticación por API key (SHA-256)
- `EmailService`: envío con Lettre (SMTP configurable)
- `User` model + repository

### Prioridad 2 — Patrones genéricos
- Repository pattern base (trait/macro para CRUD)
- OpenAPI setup (utoipa + swagger-ui)
- SPA fallback (ServeDir/ServeFile)
- CORS config estándar
- Router composition (`create_router()` base)

### Prioridad 3 — Frontend
- Hooks genéricos (validación de formularios, etc.)
- Template Orval config (tags-split)
- Template Vite config

---

## Repositorios

| Repo | Contenido | Rama principal |
|------|-----------|----------------|
| `1ndoryu/glory-rs` | Framework (este submódulo) | `master` |
| `1ndoryu/glory-rs-template` | Proyecto template | `main` |
| `1ndoryu/glory-rs` (rama `glory-rs-rest`) | Proyecto restaurante (instancia) | `glory-rs-rest` |

### Reorganización pendiente
Actualmente el framework y el template coexisten en el mismo repo. La reorganización planificada:
1. **`1ndoryu/glory-rs`** → contiene solo el framework (componentes + crate backend)
2. **`1ndoryu/glory-rs-template`** → proyecto template limpio que usa glory-rs como submódulo
3. **Ramas de instancias** → cada cliente tiene su rama (ej: `glory-rs-rest` para el restaurante)

**Precaución:** La rama `glory-rs-rest` contiene todo el proyecto del restaurante con datos y migraciones específicas. La reorganización debe preservar todo el historial de commits.

---

## Cómo crear un nuevo proyecto desde template

1. Clonar template: `git clone https://github.com/1ndoryu/glory-rs-template nuevo-proyecto`
2. Añadir submódulo: `git submodule add https://github.com/1ndoryu/glory-rs glory-rs`
3. Configurar `.env` con DATABASE_URL, JWT_SECRET
4. Ejecutar migraciones: `cargo run` (auto-aplica al iniciar)
5. Instalar frontend: `cd frontend && npm install`
6. Desarrollo: `./dev.ps1`

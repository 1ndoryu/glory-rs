# Lecciones Aprendidas

Registro de errores recurrentes, patrones que funcionaron, y conocimiento adquirido durante el desarrollo.
Cada lección debe ser concisa y accionable.

---

## 2026-03-25 — Emojis Unicode en JSX

**Problema:** Se usaron emojis Unicode directamente en componentes React en vez de SVG/iconos.
**Causa raíz:** No había regla que lo prohibiera ni detección automática.
**Solución:** Regla 18 en protocolo + regla `emoji-en-codigo` en Glory Sentinel.
**Acción preventiva:** Sentinel lo detecta automáticamente ahora.

## 2026-03-25 — node_modules/ raíz sin .gitignore

**Problema:** `npm install` en la raíz creó `node_modules/` que no estaba en `.gitignore`, causando 2596 archivos sin trackear.
**Causa raíz:** `.gitignore` solo excluía `/frontend/node_modules/`, no `node_modules/` global.
**Solución:** Añadido `node_modules/` sin prefijo al `.gitignore`.
**Acción preventiva:** Regla 12.1 en protocolo (git limpio).

## 2026-03-25 — Comentarios JS con glob paths

**Problema:** `/* **/node_modules/** */` dentro de comentarios JS rompe el parser de TypeScript porque interpreta `*/` como cierre de comentario.
**Solución:** No usar patrones glob con `**/` dentro de comentarios JS/TS.

## 2026-03-25 — Orval v8 customInstance signature

**Problema:** Login no funcionaba. `customInstance` retornaba `response.data` pero Orval v8 genera tipos que esperan `{ data, status, headers }`.
**Causa raíz:** Los componentes checaban `respuesta.status === 200` que nunca era true.
**Solución:** Retornar `{ data: response.data, status: response.status, headers: response.headers } as T`.

## 2026-03-25 — PowerShell 5 limitaciones en scripts

**Problema:** Scripts PS1 con em dash (`—`), ternarios inline, o comas entre hashtables en arrays fallan en PS5.
**Causa raíz:** PS5 tiene parser más limitado que PS7.
**Solución:** Solo ASCII, if/else estándar, sin comas entre elementos de array de hashtables.

## 2026-03-25 — Git submodule con ruta local

**Problema:** `git submodule add ../glory-rs-framework glory-rs` resuelve contra la URL remota de GitHub, no contra el filesystem local.
**Causa raíz:** Git interpreta paths relativos en relación al remote origin, no al directorio actual.
**Solución:** Usar path absoluto + `git config --global protocol.file.allow always` (Git moderno bloquea file:// por defecto).

## 2026-03-25 — TypeScript en submódulo necesita sus propias deps

**Problema:** Componentes TSX en un submódulo no resuelven `react` types. Errores como "Property 'children' does not exist on BotonProps" aunque extends `ButtonHTMLAttributes`.
**Causa raíz:** TypeScript busca `node_modules` ascendiendo desde la ubicación del archivo. El submódulo no tiene `node_modules` y el del proyecto consumidor está en `frontend/node_modules`, no en la raíz.
**Solución:** El submódulo necesita su propio `package.json` con devDependencies (react, @types/react, lucide-react) y `npm install`. También necesita `server.fs.allow: ['..']` en Vite.

## 2026-03-26 — validator range() incompatible con rust_decimal::Decimal

**Problema:** `#[validate(range(min=0.0, max=100.0))]` no compila con `rust_decimal::Decimal` — no implementa `ValidateRangeType`.
**Causa raíz:** El crate `validator` solo soporta tipos numéricos primitivos para `range()`.
**Solución:** Crear función de validación custom (`validar_iva(&rust_decimal::Decimal) -> Result<(), ValidationError>`). OJO: el derive de `Validate` con `Option<Decimal>` pasa `&&Decimal`, no `&Option<Decimal>`.

## 2026-03-26 — PowerShell Out-File genera UTF-8 BOM que Orval no parsea

**Problema:** `cargo run --bin dump_openapi | Out-File openapi.json` genera archivo con BOM (byte order mark). Orval falla al parsearlo.
**Causa raíz:** PowerShell 5 usa UTF-8 BOM por defecto con `Out-File` y `>`.
**Solución:** Usar `[System.IO.File]::WriteAllText($path, $output, [System.Text.UTF8Encoding]::new($false))` para escribir sin BOM.

## 2026-03-26 — dump_openapi usa stdout, no argumentos

**Problema:** El binario `dump_openapi` usa `print!` para escribir el JSON, no acepta path como argumento CLI.
**Solución:** Capturar stdout y redirigir manualmente: `$output = cargo run --bin dump_openapi 2>$null; [System.IO.File]::WriteAllText(...)`.

## 2026-03-26 — Orval encoding: ó se corrompe en nombre de archivo schemas

**Problema:** Al regenerar con Orval, `gestiónRestauranteAPI.schemas.ts` se convierte en `gestiNRestauranteAPI.schemas.ts` (ó → N por encoding).
**Causa raíz:** El título del OpenAPI spec tiene `ó` que se corrompe en el filesystem dependiendo del encoding del pipe.
**Solución:** Actualizar el barrel `generated.ts` para importar desde el nombre real del archivo generado. No intentar forzar el nombre — aceptar el que Orval genera.

## 2026-03-26 — TypeScript union types en respuestas Orval requieren narrowing

**Problema:** Hooks Orval retornan `ConfiguracionRestaurante | ErrorResponse`. Acceder a campos de datos sin narrowing causa error TS.
**Causa raíz:** Orval genera tipos union para cubrir respuestas 200 y errores. TypeScript no permite acceder a campos específicos sin discriminar.
**Solución:** Usar `datos.status === 200` como discriminante antes de acceder a campos de datos.

## 2026-03-26 — Coolify beta 460 no soporta dockerfile_inline en servicios

**Problema:** `POST /api/v1/services` acepta un compose con `build: dockerfile_inline:`, pero Coolify transforma el compose reemplazando `dockerfile_inline` con `dockerfile: Dockerfile.rust` y el build nunca se ejecuta.
**Causa raíz:** Limitación de Coolify beta 460 — los servicios no soportan builds inline, solo imágenes pre-construidas.
**Solución:** Construir la imagen Docker directamente en el servidor via SSH (`docker build -t nombre:latest .`) y usar `image: nombre:latest` en el compose de Coolify.
**Acción preventiva:** El template de coolify-manager-rs debería generar un workflow que construye en el servidor, no depender de Coolify para builds.

## 2026-03-26 — Submodulo git con URL local no funciona en Deploy

**Problema:** `.gitmodules` apuntaba a ruta Windows local (`c:\Users\...`). El `git clone --recurse-submodules` en Linux fallaba.
**Solución:** Crear repo público en GitHub para el submodulo y actualizar `.gitmodules` con URL HTTPS.
**Acción preventiva:** Nunca usar paths locales en `.gitmodules` — siempre URLs de repositorios remotos.

## 2026-03-26 — Dockerfile: verificar MSRV antes de elegir imagen Rust

**Problema:** `rust:1.83` y `rust:1.85` no soportaban crates que requieren Rust ≥1.88 (time-core, home).
**Causa raíz:** Las dependencias actualizan su MSRV con cada release, pero la imagen Docker se fijó en una versión antigua.
**Solución:** Usar `rust:1.88-bookworm` (mínimo requerido por las dependencias). Verificar MSRV con `cargo check` localmente.

## 2026-03-26 — Traefik necesita estar en la misma red Docker

**Problema:** Gateway Timeout al acceder via dominio. La app era accesible directamente por puerto pero Traefik no la alcanzaba.
**Causa raíz:** Traefik (coolify-proxy) no estaba conectado a la red del servicio (`b8s0cks...`). Solo podía resolver el contenedor desde redes donde ambos estuvieran presentes.
**Solución:** `docker network connect <red-servicio> coolify-proxy`. También funciona conectar la app a la red `coolify`.

## 2026-03-26 — Coolify API: docker_compose_raw requiere base64

**Problema:** PATCH al campo `docker_compose` retorna 422 "This field is not allowed". El campo correcto es `docker_compose_raw` pero debe ser base64-encoded.
**Solución:** Usar `docker_compose_raw` con el YAML codificado en base64. PATCH envs via `/envs` sin UUID en la ruta.

## 2026-03-26 — Axum 0.7 path params: :param vs {param}

**Problema:** TODAS las rutas con path params devolvían 405 (DELETE) o 404 (GET) porque usaban sintaxis `{id}` en `.route()`.
**Causa raíz:** Axum 0.7.x usa matchit 0.7.x, que solo soporta `:param`. La sintaxis `{param}` es de matchit 0.8+ / Axum 0.8+. Las rutas con `{id}` eran tratadas como paths literales y caían al fallback SPA.
**Solución:** Cambiar `{id}` a `:id` en todas las llamadas `.route()`. Las anotaciones `#[utoipa::path]` mantienen `{id}` (sintaxis OpenAPI).
**Prevención:** No copiar la sintaxis de utoipa a los `.route()`. Agregar test que haga GET/DELETE por ID en CI.

## 2026-03-26 — LEFT JOIN + NULL en SQLx: unexpected null

**Problema:** Query con LEFT JOIN producía `cr.nombre = NULL` para filas sin relación, causando 500 "unexpected null; try decoding as an Option" en SQLx.
**Causa raíz:** `canal_nombre` en la query no tenía `?` override y el struct tenía `Option<String>`, pero la cache `.sqlx` marcaba la columna como `nullable: false` (heredado de la definición de tabla, no del JOIN). En runtime con datos reales, NULL causaba error de decodificación.
**Solución:** `COALESCE(cr.nombre, 'Sin canal')` garantiza non-null. También agregar todos los campos no-agregados al GROUP BY.
**Prevención:** Siempre usar COALESCE en LEFT JOINs o el override `as "col?"` en SQLx para columnas que pueden ser NULL.

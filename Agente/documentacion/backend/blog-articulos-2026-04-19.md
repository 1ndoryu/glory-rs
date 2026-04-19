# 174A-86 â€” Blog (artÃ­culos CRUD + comentarios + categorÃ­as)

## Alcance portado

- `GET /api/articulos`
- `GET /api/articulos/categorias`
- `GET /api/articulos/{slug}`
- `GET /api/articulos/mis-articulos`
- `POST /api/articulos`
- `PUT /api/articulos/{id}`
- `DELETE /api/articulos/{id}`
- `POST /api/articulos/{id}/like`

## Backend

- Se aÃ±adiÃ³ `src/models/article.rs` con DTOs de listado, detalle, delete, like y el request doc multipart para OpenAPI.
- Se aÃ±adiÃ³ `src/repositories/article.rs` y se dividiÃ³ en `src/repositories/article/{types,read,write}.rs` para respetar el lÃ­mite de tamaÃ±o de repositorios.
- Se aÃ±adiÃ³ `src/handlers/articles.rs` y se delegÃ³ parsing/validaciÃ³n en `src/handlers/articles/support.rs` para mantener el controlador bajo el lÃ­mite interno.
- `src/handlers/mod.rs` quedÃ³ cableado con rutas, schemas y paths OpenAPI del dominio de artÃ­culos.

## Paridad legacy relevante

- Listado pÃºblico con filtro opcional por categorÃ­a y respuesta `{ ok, data: { articulos, total, hay_mas } }`.
- Detalle por `slug` con visibilidad de borradores/pendientes solo para autor o admin.
- `mis-articulos` con filtro opcional por `moderacion_estado`.
- CreaciÃ³n por `multipart/form-data`, con `portada` binaria opcional y `portada_url` opcional.
- ActualizaciÃ³n por JSON.
- Like toggle con respuesta `{ ok, liked, total }`.
- Rate limit de creaciÃ³n: `5` artÃ­culos por hora por autor.
- Auto-aprobaciÃ³n para admin y `pendiente` para usuario normal.
- Portada validada contra JPEG, PNG y WEBP con lÃ­mite de `5MB`.
- `embeds` normaliza solo `sample` y `coleccion`, igual que el legado efectivo.

## Comentarios y categorÃ­as

- CategorÃ­as se sirven desde `articulos` agrupando solo publicados aprobados.
- Comentarios no requirieron un handler nuevo en este corte porque el dominio de comentarios ya soportaba `tipo = 'articulo'` y mantiene `total_comentarios` sincronizado sobre la tabla `articulos`.

## ValidaciÃ³n ejecutada

- `cargo check`
- `cargo sqlx prepare`
- `cargo run -- --emit-openapi openapi.json`
- `npm --prefix frontend run codegen`
- `npm --prefix frontend run type-check`

## Observaciones

- El almacenamiento de portada reutiliza `state.storage` y publica URLs bajo `/uploads/...`.
- Se mantuvo `articulos_likes` como tabla fuente de verdad en vez de forzar este dominio sobre el sistema polimÃ³rfico general de likes.
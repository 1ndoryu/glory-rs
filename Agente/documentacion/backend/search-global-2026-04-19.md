# 174A-87 - Busqueda global

## Alcance portado

- `GET /api/search?q=...&types=samples,users,collections,songs`
- `GET /api/busqueda/rapida?q=...`

## Backend

- Se anadio `src/models/search.rs` con los DTOs del endpoint canonico y del alias legacy.
- Se anadio `src/repositories/search.rs` con queries separadas para songs, samples, users, collections y relaciones de sampleo.
- Se anadio `src/services/search.rs` para centralizar normalizacion de query, parseo de `types`, mapping de assets a URLs publicas y scoring legacy de `todos`.
- Se anadio `src/handlers/search.rs` y `src/handlers/mod.rs` quedo cableado con rutas, OpenAPI y schemas del dominio de busqueda.

## Contrato efectivo

- El endpoint canonico del roadmap es `GET /api/search` y devuelve resultados agrupados en `samples`, `users`, `collections` y `songs`.
- Se mantuvo un alias compatible `GET /api/busqueda/rapida` para no perder el contrato del dropdown legacy.
- Consultas con menos de `2` caracteres devuelven listas vacias, igual que el legado.
- El alias legacy conserva `5` resultados por tipo y genera `todos` con score descendente y corte a `12` items.
- `types` acepta aliases utiles (`usuarios`, `colecciones`, `canciones`) pero normaliza al set del roadmap: `samples`, `users`, `collections`, `songs`.

## Paridad legacy relevante

- `songs` reutiliza `canciones` + `artistas_musicales` y busca por titulo, artista y album.
- `samples` busca solo samples activos y visibles en comunidad, con creador resumido.
- `users` excluye `es_seed = true` y ordena por `total_seguidores`.
- `collections` filtra solo colecciones publicas con al menos un sample.
- El alias legacy incluye `sampleos` sobre `relaciones_sample`, con `fuente` y `destino` listos para el dropdown.

## Observaciones

- El schema Rust actual de `colecciones` no expone `slug`; el endpoint canonico responde `slug = null` y el alias legacy usa `id` como fallback de shape hasta que ese campo exista otra vez en la migracion activa.
- No se porto la cache servidor de 6 horas del PHP. El contrato HTTP y el scoring legacy si quedaron portados.
- Las URLs relativas de imagen/avatar se publican bajo `/uploads/...`, igual que en el resto del backend Rust.

## Validacion ejecutada

- `cargo sqlx prepare`
- `cargo check`
- `cargo test search::tests`
- `cargo run -- --emit-openapi openapi.json`
- `npm --prefix frontend run codegen`
- `npm --prefix frontend run type-check`
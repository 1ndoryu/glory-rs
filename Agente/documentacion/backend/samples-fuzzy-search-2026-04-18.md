# 174A-47 — Búsqueda fuzzy trigram en `GET /samples`

## Objetivo

Extender el catálogo público de samples para aceptar búsqueda textual sin abrir todavía un endpoint aparte. La tarea quedó integrada sobre `GET /api/samples`, combinando el término de búsqueda con los filtros ya existentes de BPM, key, type, tags, premium y creator.

## Qué quedó implementado

- `ListSamplesQuery` ahora acepta:
  - `search`
  - alias corto `q`
  - alias legacy `busqueda`
  - `search_normalized`
  - alias legacy `busqueda_norm`
- Normalización de búsqueda en `SampleCatalogService`:
  - trim
  - mínimo `2` caracteres
  - máximo `120`
  - soporte opcional para forma normalizada distinta al término original
- `SampleListFilters` incorpora `SampleTextSearch` para transportar patrones precomputados al repositorio.
- `GET /api/samples` ahora agrega un bloque textual al `WHERE` cuando hay búsqueda:
  - FTS `to_tsvector/plainto_tsquery` sobre `titulo + descripcion`
  - `ILIKE` directo en `titulo`
  - `ILIKE` en `tags`
  - `ILIKE` en `tags_enriquecidos`
  - `word_similarity()` sobre `titulo`
  - `similarity()` sobre `tags` y `tags_enriquecidos`
  - expansión opcional por `search_normalized`
- El orden cuando hay búsqueda cambió a score compuesto:
  - `ts_rank(titulo+descripcion)`
  - boost por match de tags
  - `ts_rank(titulo)`
  - `word_similarity()`
  - bonus por `titulo ILIKE 'query%'`

## Decisiones

- Se reutilizó `GET /api/samples` en lugar de crear un endpoint nuevo porque el roadmap hablaba de “trigram + filtros combinados”, y el catálogo ya era la superficie correcta para mezclar texto con filtros estructurados.
- No se creó migración nueva: el repo ya tenía `pg_trgm`, `idx_samples_titulo_trgm`, `idx_samples_busqueda_fts` e `idx_samples_titulo_fts` desde las tareas de schema previas.
- La implementación usa `QueryBuilder<Postgres>` porque el listado ya era dinámico por naturaleza y la búsqueda agrega más ramas opcionales; forzar macros `query!` aquí significaría duplicar demasiadas variantes del mismo SQL.

## Validación

- `cargo check`
- `cargo test sample_catalog`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `npm run codegen`
- `npm --prefix frontend run type-check`

## Pendiente relacionado

- `174A-48` sigue con similitud vectorial `GET /samples/{id}/similar` sobre `pgvector`.
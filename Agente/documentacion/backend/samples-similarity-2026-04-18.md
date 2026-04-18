# Samples Similarity — 2026-04-18

## Objetivo

- Completar `174A-48` con `GET /api/samples/{id}/similar` sobre el catálogo público.
- Priorizar similitud por `pgvector` (`embedding <=> embedding_base`) y conservar fallback contextual cuando el sample base no tenga embedding o la búsqueda ANN no retorne candidatos.

## Decisiones

- El contrato nuevo usa `id` numérico porque el roadmap pedía explícitamente `GET /samples/{id}/similar`.
- La query acepta `limit` y alias legacy `limite`, con default `5` y tope `50`.
- La respuesta queda en `SimilarSamplesResponse { data: SampleSummary[] }` para mantener el shape ligero de un carrusel y reutilizar el DTO público ya usado por el catálogo.
- El sample base se resuelve solo dentro del catálogo público (`estado = 'activo'`, `mostrar_en_comunidad = TRUE`, `eliminado_en IS NULL`). Si no existe o no es visible, el endpoint responde `404`.

## Estrategia

### Camino 1 — pgvector

- Si el sample base tiene embedding, la consulta ordena por distancia coseno directa:
  - `s.embedding <=> (SELECT embedding FROM samples WHERE id = :sampleId)`
- Se filtra a samples públicos activos, distintos del sample base y con `embedding IS NOT NULL`.
- Si hay resultados, se devuelven inmediatamente, igual que en el legado.

### Camino 2 — fallback contextual

- Si el sample base no tiene embedding o la búsqueda ANN devuelve cero filas, se aplica un score compuesto inspirado en `MotorRecomendacion::samplesSimilares()`:
  - contenido por tags en común
  - contexto técnico (`bpm`, `key`, `tipo`)
  - tendencias por engagement agregado
  - novedad con decay logarítmico
- Pesos usados:
  - contenido `0.55`
  - contexto `0.10`
  - tendencias `0.20`
  - novedad `0.15`
- Sub-pesos de contexto:
  - BPM `0.25`
  - key `0.25`
  - tipo `0.50`

## Implementación

- `src/models/sample.rs`
  - `SimilarSamplesQuery`
  - `SimilarSamplesResponse`
- `src/handlers/sample_catalog.rs`
  - handler `similar_samples`
  - ruta `GET /api/samples/{id}/similar`
- `src/services/sample_catalog/mod.rs`
  - `get_similar_samples()`
  - normalización de `limit`
- `src/repositories/sample_catalog.rs`
  - el repositorio principal quedó reducido a `430` líneas para respetar la regla local de tamaño.
- `src/repositories/sample_catalog/query.rs`
  - concentra `SAMPLE_SUMMARY_SELECT` y el pipeline del listado público.
- `src/repositories/sample_catalog/similar.rs`
  - contiene la resolución del sample fuente, la consulta ANN y el fallback contextual.

## Gotchas

- El repo marca como error usar `sqlx::query_as()` runtime. Para evitar ese drift y no inflar `.sqlx/` con consultas construidas parcialmente, la similitud quedó montada sobre `QueryBuilder`.
- `sample_catalog.rs` ya estaba cerca del techo del repo; para cerrar la tarea hubo que partirlo en submódulos internos (`query.rs`, `similar.rs`) en vez de seguir acumulando lógica en un solo archivo.
- El endpoint reutiliza `SampleSummary`, así que no expone assets privados del owner ni metadata interna del sample base.

## Validación

- `cargo check` OK
- `cargo test sample_catalog` OK (`14` tests)
- `cargo clippy --all-targets -- -D warnings` OK
- `cargo test` OK (`78` tests)
- `npm run codegen` OK
- `npm --prefix frontend run type-check` OK
# Plan completado - Implementacion real de endpoints faltantes legacy

**Origen tareas:** `roadmap.md` seccion "Endpoints faltantes"
**Audit inicio:** 23 missing al crear este plan, 45 missing en el barrido ampliado posterior.
**Audit cierre:** 0 missing (`npm run audit:api`, 2026-04-28)
**Restricciones del usuario:** implementar todo real, sin stubs, con autonomia para migraciones si hacian falta.

## Resultado

El bloque de endpoints faltantes quedo completado por fases y con commits separados:

- `274A-54..58`: endpoints dev reales de scraper, recorte, canciones y publicacion.
- `274A-23+274A-24+274A-25+274A-26+274A-48`: contribuciones reales.
- `274A-29+274A-30+274A-32+274A-33+274A-34`: moderacion admin real.
- `274A-49+274A-50+274A-51+274A-52`: embeddings, experimentos y benchmark admin.
- `274A-43`: backfill real de duplicados por `audio_hash`.
- `274A-46+274A-53`: reactivacion de automatizacion y borrado masivo de samples.

## Decisiones mantenidas

- Se usaron servicios y repositorios nuevos cuando la logica superaba el handler delgado.
- Los endpoints destructivos mantienen guard admin y operaciones transaccionales cuando tocan BD.
- Las consultas nuevas de admin usan SQL runtime parametrizado con suppressions justificadas cuando `SQLX_OFFLINE=true` evita regenerar cache SQLx por cada query.
- `openapi.json` siguio siendo la fuente del codegen Orval.

## Cierre validado

La ultima ronda de cierre ejecuto:

- `cargo fmt --check`
- `cargo check --message-format=short`
- `cargo clippy --all-targets --message-format=short -- -D warnings`
- `cargo test --all-targets`
- `cargo run -- --emit-openapi openapi.json`
- `npm run codegen`
- `npm --prefix frontend run type-check`
- `npm run audit:api`

Resultado del audit final: 110 calls escaneados, 110 matched, 0 missing.
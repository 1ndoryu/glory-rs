# Admin Duplicados — 2026-04-28

## Alcance

- `GET /api/admin/duplicados` lista duplicados pendientes con datos de sample original y duplicado.
- `GET /api/admin/duplicados/contar` devuelve el badge de pendientes.
- `POST /api/admin/duplicados/backfill` ejecuta el backfill manual de `audio_hash` exacto.

## Arquitectura

- `src/handlers/admin_duplicados.rs` queda como capa HTTP/OpenAPI y validacion admin.
- `src/repositories/admin_duplicates.rs` concentra las consultas parametrizadas a `samples`, `usuarios_ext` y `duplicados_pendientes`.
- `src/services/admin_duplicates/mod.rs` contiene el flujo real de backfill y hashing de archivos.

## Backfill

- El request acepta `{ "batch": number }`; se normaliza a `10..500`, con default `100`.
- Solo procesa samples `activo` sin `audio_hash` y sin `eliminado_en`.
- Calcula SHA-256 leyendo `ruta_original` en streaming para no cargar audios grandes completos en memoria.
- Si el archivo no existe o no se puede leer, incrementa `sin_archivo` y deja log con `tracing::warn!`.
- Si el hash no existe en otro sample visible, guarda `audio_hash` y suma `hasheados`.
- Si ya existe otro sample con ese hash, crea `duplicados_pendientes` con tipo `mismo_usuario` o `cross_usuario` y marca el sample duplicado `en_supervision` para sacarlo del backfill activo.

## Diferencia necesaria con legacy

El PHP legacy guardaba el hash incluso cuando detectaba duplicado. En Rust existe un indice unico parcial sobre `samples(audio_hash)` para estados `activo` y `en_supervision`; guardar el mismo hash en ambos samples romperia la constraint. Por eso el port conserva el resultado funcional importante (registro admin pendiente + muestra fuera del flujo activo) sin duplicar el hash en la fila duplicada.

## Validacion

- `cargo fmt --check` OK.
- `cargo check --message-format=short` OK.
- `cargo clippy --all-targets --message-format=short -- -D warnings` OK.
- `cargo test --all-targets` OK: 181 tests.
- `cargo run -- --emit-openapi openapi.json` OK.
- `npm run codegen` OK.
- `npm --prefix frontend run type-check` OK.
- `npm run audit:api` queda en 2 faltantes esperados: automatizacion reactivar y samples todos.

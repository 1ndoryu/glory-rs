# Audio Embeddings — 2026-04-18

## Estado actual

- `174A-33`: existe `src/audio/embeddings.rs` con generación determinista de embeddings `128d` para samples y perfiles de usuario.
- El layout replica el contrato legacy: slots fijos para BPM, key, scale, tipo, duración y premium; desde `22..127` se usan buckets de tags con `CRC32`.
- El módulo expone helpers de ida y vuelta con `pgvector::Vector`, que es el tipo alineado con `samples.embedding vector(128)` en PostgreSQL.

## Estructura del vector

- `0`: BPM normalizado en rango `0..1` con tope `300`.
- `1..12`: one-hot de `music_key` con equivalencias enharmónicas (`C# == Db`, etc.).
- `13..14`: scale mayor/menor; si falta, queda neutral en `0.5 / 0.5`.
- `15..19`: tipo de sample. Por compatibilidad con el algoritmo legacy, solo se activan `loop` y `one_shot`; el resto cae al bucket por defecto.
- `20`: duración log-normalizada con tope `600s`.
- `21`: flag premium.
- `22..127`: `106` buckets de tags con `crc32fast`, acumulados de forma estable e insensible a mayúsculas.

## API pública

- `AudioEmbedding::generate(input) -> AudioEmbedding`
- `AudioEmbedding::to_pgvector() -> pgvector::Vector`
- `AudioEmbedding::from_pgvector(vector) -> Result<AudioEmbedding, EmbeddingError>`
- `AudioEmbedding::from_slice(values) -> Result<AudioEmbedding, EmbeddingError>`
- `AudioEmbedding::build_weighted_profile(embeddings, weights) -> Option<AudioEmbedding>`

## Perfil ponderado

- El embedding raw del sample no se normaliza por L2, igual que en el legado.
- El perfil agregado de usuario sí hace promedio ponderado y normalización L2 final para que la similitud coseno sobre `pgvector` se comporte de forma estable.
- Si todos los pesos son `<= 0` o las longitudes no coinciden, el perfil devuelve `None`.

## Testing validado

- `cargo check` OK.
- `cargo clippy --all-targets -- -D warnings` OK.
- `cargo test audio::embeddings` OK.
- `cargo test` OK.
- Casos cubiertos:
  - determinismo con la misma entrada
  - equivalencia enharmónica (`C#` y `Db`)
  - scale neutral cuando falta metadata
  - hashing de tags case-insensitive
  - roundtrip con `pgvector`
  - normalización L2 del perfil ponderado
  - rechazo de dimensiones inválidas

## Notas de migración

- El detector sigue el layout exacto del algoritmo legacy encontrado en `GeneradorEmbeddings.php`, lo que facilita comparar resultados entre el sistema PHP y el nuevo backend Rust.
- La integración con el pipeline de subida todavía no se hace aquí; queda para `174A-34`, que es donde conviene resolver el contrato entre FFmpeg, BPM, key, IA y persistencia.
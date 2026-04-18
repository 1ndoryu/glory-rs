# Algorithm Signals — 2026-04-18

## Objetivo

- Completar `174A-49` con un módulo puro `src/algorithm/signals.rs` que centraliza las 6 señales del algoritmo de descubrimiento.
- Dejar las fórmulas desacopladas de SQL, perfiles y repositorios para que `174A-50..52` puedan reutilizar una única fuente de verdad.

## Decisión clave

- El comentario viejo en `src/algorithm/mod.rs` y una parte del plan mencionaban pesos placeholder `0.25 / 0.25 / 0.15 / 0.15 / 0.10 / 0.10`.
- El legado vivo (`algoritmoPesos.php` + `ConstructorSenales.php`) ya no usa esa calibración: hoy está en
  - `similitud_contenido = 0.28`
  - `comportamiento = 0.27`
  - `contexto = 0.15`
  - `tendencias = 0.12`
  - `grafo_social = 0.10`
  - `novedad = 0.0`
- El módulo Rust porta esos valores reales para evitar drift funcional desde el arranque de la fase 7.

## Implementación

- `src/algorithm/signals.rs`
  - `AlgorithmSignalConfig`
  - `AlgorithmSignalWeights`
  - subpesos de comportamiento, contexto, tendencias y grafo social
  - normalizadores absolutos de tendencias
  - parámetros `bpm_tolerancia` y `novedad_dias_boost`
  - inputs puros para cada señal
  - helpers de score por señal
  - `SignalScoreBreakdown` con `raw` + `weighted` por señal
- `src/algorithm/signals/tests.rs`
  - cobertura unitaria de pesos legacy, clamps, novedad logarítmica y total ponderado
- `src/algorithm/mod.rs`
  - se actualizó para exponer `signals` y eliminar el comentario desfasado

## Fórmulas portadas

### Similitud de contenido

- Convierte distancia coseno en similitud unitaria:
  - `1 - distancia / 2`
  - bounded a `[0, 1]`

### Comportamiento

- Combina los 5 sub-factores legacy y resta la penalización por dislikes:
  - likes dados `0.30`
  - reproducciones `0.25`
  - tiempo escucha `0.20`
  - descargas `0.15`
  - completadas `0.10`

### Contexto

- Mantiene la prioridad temática del legado actual:
  - creador `0.45`
  - género `0.30`
  - técnico total `0.25` repartido en BPM/key/escala/tipo
- `key`, `escala` y `tipo` usan neutral `0.5` cuando no hay preferencia disponible.

### Tendencias

- Usa normalizadores absolutos, no división por edad del sample:
  - likes `15`
  - reproducciones `30`
  - descargas `20`
  - follows `10`

### Grafo social

- `creador_seguido = 0.60`
- `likeado_por_seguidos = 0.40`, capped con divisor `4`

### Novedad

- Decay logarítmico:
  - `max(0, 1 - ln(max(1, dias)) / ln(dias_boost))`
- En la calibración actual la señal sigue calculándose, pero el peso principal está en `0.0` porque el legado la dejó temporalmente desactivada.

## Gotchas

- El módulo debía quedar puro y reusable; meter SQL o dependencias de perfil en esta fase habría bloqueado `174A-50` y `174A-51` con interfaces prematuras.
- Los pesos principales actuales no suman `1.0` porque `novedad` está desactivada en el legado. El módulo expone `total_weight()` para hacer explícita esa realidad en vez de ocultarla con una renormalización inventada.

## Validación

- `cargo test algorithm::signals` OK (`8` tests)
- `cargo clippy --all-targets -- -D warnings` OK
- `cargo test` OK (`86` tests)
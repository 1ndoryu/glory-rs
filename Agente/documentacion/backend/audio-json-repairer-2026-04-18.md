# 174A-39 — JsonRepairer local para respuestas IA

## Objetivo

Incorporar una capa local y tolerante para rescatar metadata creativa cuando el LLM no devuelve JSON perfecto. El problema real no era solo `json_decode` fallando, sino respuestas con bloque fenced, texto alrededor, comas sobrantes, claves sin comillas o caracteres de control incrustados dentro de strings.

## Implementación

- Se creó `src/audio/ia/json_repairer.rs` con dos entradas principales:
  - `extract_metadata_from_text()` para texto libre del assistant;
  - `extract_metadata_from_provider_response()` para bodies OpenAI-compatible con `choices[0].message.content`.
- El módulo aplica estrategias locales y sin red, en este orden:
  - parseo JSON directo;
  - parseo de bloques fenced ````json ... ````;
  - extracción del primer objeto balanceado `{ ... }` respetando strings y escapes;
  - sanitización de caracteres de control dentro de strings JSON;
  - parseo tolerante con `json5` para soportar comas finales, claves sin comillas y comillas simples.
- La salida se normaliza a `AudioCreativeMetadata`, un struct tipado con los mismos campos creativos que el legacy:
  - `nombre_archivo_base`
  - `tags`, `tags_es`
  - `tipo`
  - `genero`, `emocion`, `emocion_es`
  - `instrumentos`
  - `artista_vibes`
  - `descripcion_corta`, `descripcion_corta_es`
  - `descripcion`, `descripcion_es`
- La validación también tolera que el modelo entregue un string donde se esperaba array, convirtiéndolo a una lista de un solo elemento. Esto hace al parser más resistente que el PHP original sin romper el shape final.

## Decisiones

- No se implementó todavía reparación remota con otro LLM. El roadmap de esta tarea pedía `regex + parser tolerante`, y eso ya cubre la mayoría de fallos reales sin meter latencia, costo ni recursion entre proveedores.
- Se añadió `json5` en lugar de una cascada manual de regexes para trailing commas, keys sin comillas o comillas simples. Es una solución más clara y menos frágil.
- `tipo` se normaliza a `oneshot` o `loop`, manteniendo compatibilidad con el check constraint y con el embedding builder ya migrado.

## Tests

- Casos cubiertos en `src/audio/ia/json_repairer/tests.rs`:
  - JSON directo;
  - bloque fenced;
  - texto alrededor del objeto;
  - salida estilo JSON5;
  - caracteres de control dentro de strings;
  - respuesta OpenAI-compatible completa;
  - límites de longitud/cantidad y default de `tipo`;
  - errores cuando no hay contenido o no existe JSON recuperable.

## Validación

- `cargo test json_repairer`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
# Moderation Service — 2026-04-18

## Objetivo

Introducir un motor de moderación reusable para Kamples antes de conectar una cola o panel admin reales en Rust. La meta de `174A-43` fue fijar el contrato de dominio y las reglas de decisión, no acoplarlo todavía a `publicaciones`, `comentarios` o una `moderation_queue` que aún no existe en el runtime nuevo.

## Capas implementadas

1. **Pre-filtro local**
   - Reglas baratas y deterministas para spam promocional, URLs múltiples, sexual explícito, actividad ilegal y ruido evidente en tags.
   - Rechaza lo obvio sin gastar cuota de LLM.

2. **Categorización IA**
   - Groq primario con cadena específica de modelos de moderación.
   - OpenAI como fallback final si Groq falla o devuelve JSON irreparable.
   - El prompt obliga JSON con `safe`, `category`, `confidence`, `recommended_level`, `reason_code` y `summary`.

3. **Decisión**
   - Combina el hallazgo más restrictivo entre local e IA.
   - `rechazado` de IA solo auto-bloquea con confianza alta (`>= 0.85`).
   - Fallos de proveedores con contenido analizable degradan a `revision`, nunca a éxito silencioso.
   - Si no hay proveedores configurados, el servicio hace fallback explícito a reglas locales y deja trazabilidad en `reason_code=local_only_fallback`.

4. **Payload para panel admin**
   - El resultado ya trae `headline`, `summary`, `badges`, `evidence` y `priority` para que un panel o cola futura no tenga que recomputar nada.
   - También expone `recommended_sample_state()` para enrutar a `activo` o `en_supervision` cuando se conecte al pipeline real.

## Estructura

- `src/services/moderation/mod.rs`: orquestación, prompt, fallback y decisión.
- `src/services/moderation/local_rules.rs`: pre-filtro local.
- `src/services/moderation/types.rs`: request/result/admin payload/failures.
- `src/services/moderation/tests.rs`: contrato del servicio.

## Decisiones de arquitectura

- No se reutilizó `cola_procesamiento_ia` para moderación. En Rust ya fue reapropiada por el pipeline DSP y mezclar ambos dominios volvería a introducir drift arquitectónico.
- No se conectó todavía a repositorios de `publicaciones`/`comentarios`, porque esos repos no están portados. El servicio quedó puro para que una `moderation_queue` o handlers futuros lo consuman sin reescribir reglas.
- La cadena de Groq no reutiliza la de audio: moderación y clasificación creativa tienen perfiles de modelo distintos.

## Validación

- `cargo check`
- `cargo test moderation`
- `cargo clippy --all-targets -- -D warnings`
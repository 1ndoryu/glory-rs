# Handlers `free_codes::*` + bypass de descarga (174A-84) — 2026-04-19

## Alcance
- Tarea: `174A-84 — Códigos gratis CRUD + uso`.
- Se portó el flujo legacy de códigos gratis para samples y colecciones.
- Se integró el uso del código reclamado dentro de descargas individuales y ZIPs.

## Endpoints añadidos
- `POST /api/codigos-gratis/generar`
  - auth admin
  - body: `tipo`, `targetId`
  - devuelve `codigo`
- `GET /api/codigos-gratis/verificar?codigo=...`
  - público
  - diferencia código válido, inexistente/invalido y expirado (`410`)
- `POST /api/codigos-gratis/reclamar`
  - auth usuario
  - idempotente
  - si el código expiró, devuelve `expired=true` y acredita 50 `creditos_bonus` una sola vez
- `POST /api/codigos-gratis/invalidar`
  - auth admin
  - desactiva todos los códigos activos para un `tipo + targetId`

## Persistencia nueva
- Migración: `20260419000032_free_codes.up.sql`
- Tablas:
  - `codigos_descarga_gratis`
    - `codigo`, `tipo`, `target_id`, `activo`, `nombre_item`, `expires_at`
  - `codigos_gratis_usos`
    - reclamo/compensación idempotente por `codigo_id + usuario_id`
    - flag `expirado` para distinguir compensación de habilitación real

## Reglas portadas
- Expiración por defecto: `NOW() + 1 year`
- Reclamo válido:
  - inserta uso con `ON CONFLICT DO NOTHING`
- Reclamo expirado:
  - inserta uso con `expirado = TRUE`
  - incrementa `usuarios_ext.creditos_bonus` en `50`
  - todo dentro de una transacción para no desalinear claim y compensación
- Verificación de uso en descargas:
  - `FreeCodeRepository::can_user_download(...)` exige:
    - código activo
    - tipo/target correctos
    - usuario que ya lo reclamó
    - `cgu.expirado = FALSE`
    - `expires_at > NOW()`

## Integración en descargas
- `POST /api/samples/:id/descargar`
  - ahora acepta body opcional `DownloadGrantRequest { codigoGratis? }`
  - si el código es válido para ese sample:
    - no consume crédito
    - salta bloqueo por sample premium
    - salta bloqueo por compra individual
- `POST /api/colecciones/:id/descargar-zip`
  - mismo body opcional
  - si el código es válido para esa colección:
    - no aplica límite diario free
    - no aplica bloqueo por premium dentro de la colección

## Ajuste adicional heredado
- `DownloadRepository::user_download_allowance(...)` ahora devuelve:
  - `plan`
  - `bonus_credits`
- `GET /api/descargas/limites` expone además:
  - `limiteBase`
  - `creditosBonus`
- Esto alinea Rust con el legado donde `creditos_bonus` expande el límite efectivo diario.

## No portado todavía
- Rate limiting `30/min` por IP en `verificar`
  - el comentario quedó explícito en `src/handlers/free_codes.rs`
  - depende de un RateLimiter global que el backend aún no tiene

## Validación ejecutada
- `cargo fmt`
- `cargo sqlx prepare --workspace`
- `cargo check`
- `cargo test`
- `cargo run -- --emit-openapi openapi.json`
- `npm --prefix frontend run codegen`
- `npm --prefix frontend run type-check`

## Observaciones
- `cargo clippy -- -D warnings` permanece rojo por deuda previa en `comments.rs`, `samples.rs` y `email.rs`; la tarea no añadió nuevos fallos equivalentes salvo un `allow(clippy::too_many_lines)` localizado en el handler ZIP ya existente.
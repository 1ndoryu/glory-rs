# 174A-77 - Email SMTP + plantillas

## Alcance
- Se porto el canal SMTP opcional para emails transaccionales usando config por variables de entorno.
- Se agregaron tres plantillas base: bienvenida, confirmacion de compra y notificacion email opt-in.
- `POST /api/auth/register` ahora dispara el email de bienvenida de forma fire-and-forget cuando SMTP esta configurado.

## Contrato funcional
- No se agregan endpoints nuevos.
- `POST /api/auth/register`
  - mantiene la misma respuesta HTTP.
  - si el usuario se crea correctamente y SMTP existe, se intenta enviar bienvenida sin bloquear el registro.

## Contrato de configuracion
- Variables canonicas:
  - `SMTP_HOST`
  - `SMTP_PORT` (default `587`)
  - `SMTP_USER`
  - `SMTP_PASSWORD`
  - `SMTP_SECURE` (`tls` default, `ssl` opcional)
  - `SMTP_FROM_EMAIL` (default: `SMTP_USER`)
  - `SMTP_FROM_NAME` (default: `Kamples`)
- Aliases legacy compatibles:
  - `SMTP_PASS`
  - `SMTP_FROM`

## Reglas de negocio portadas
- Si no hay SMTP configurado, el canal email queda deshabilitado sin afectar auth ni el resto del backend.
- Si existe config SMTP pero es invalida, el backend falla al arrancar para no esconder una mala configuracion productiva.
- El email de bienvenida replica el tono y CTA del legado y usa `PUBLIC_BASE_URL` cuando existe; si no, cae a `https://kamples.com`.
- Las plantillas de compra y notificacion opt-in quedan listas, pero no se conectan todavia porque los productores reales siguen pendientes (`174A-79+` y `174A-78`).

## Implementacion
- `src/services/email.rs`
  - runtime SMTP con `lettre` y TLS rustls
  - renderer HTML para welcome, compra y notificacion opt-in
  - helper `spawn_welcome` para envio no bloqueante
- `src/config/mod.rs`
  - carga SMTP opcional con aliases legacy
- `src/handlers/auth.rs`
  - trigger de bienvenida tras registro exitoso
- `src/lib.rs`, `src/main.rs`, `src/handlers/mod.rs`
  - wiring del runtime opcional dentro de `AppState`

## Decision tecnica importante
- Se uso `lettre` con `tokio1-rustls-tls` para mantener el mismo criterio del repo: evitar dependencias OpenSSL en Windows.
- No se metio el envio en `AuthService::register`; el trigger vive en el handler para conservar el servicio auth puro y para que el fallo del email no afecte la creacion del usuario.

## Diferido explicito
- No hay fanout de notificaciones por email todavia; eso sigue en `174A-78`.
- No hay envio de compra conectado aun porque el flujo de pagos Rust sigue pendiente (`174A-79+`).

## Validacion ejecutada
- `cargo check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `npm --prefix frontend run type-check`
- `npm run self-check -- -TareaId 174A-77`
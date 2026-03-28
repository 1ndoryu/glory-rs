# Plataforma de Gestión de Restaurantes — Roadmap

> **URL producción:** http://app-b8s0cks444o0sogo8kg8wcgw.66.94.100.241.sslip.io (Swagger UI: /swagger-ui/)
> **URL directa:** http://66.94.100.241:3001
> **Servidor:** 66.94.100.241 (Coolify, servicio UUID: b8s0cks444o0sogo8kg8wcgw)
> **Deploy:** `scripts/deploy-server.sh` via SSH (requiere DB_PASSWORD y JWT_SECRET como env vars)
> **Repositorio:** 1ndoryu/glory-rs, rama glory-rs-rest

## Stack

| Capa            | Herramienta                          |
| --------------- | ------------------------------------ |
| Framework web   | Axum 0.7                             |
| OpenAPI         | utoipa 4 + utoipa-swagger-ui 7       |
| Base de datos   | SQLx 0.8 (PostgreSQL 18)             |
| Validación      | validator 0.18                       |
| Auth            | jsonwebtoken + argon2 + SHA-256 (API keys) |
| Frontend        | React 18 + TypeScript + Vite + Tailwind v4 + shadcn/ui |
| State           | React Query + Zustand                |
| Codegen         | Orval 8 (tags-split)                 |
| Linter          | clippy (deny all + warn pedantic)    |

## Notas del cliente

- Diseño minimalista, simple, intuitivo.
- Una instancia por propietario/restaurante (no multi-tenant con registro público).
- Si conseguimos más negocios, clonar la plataforma para cada nuevo cliente usando glory-rs como template.
- Parte económica simplificada: solo Gastos, Ventas y Margen (Ventas - Gastos).
- Secciones omitidas/pausadas: Dashboard analítico avanzado, Administración de documentos, Conciliación, Incidencias, Tesorería, Bancos.

## Features pendientes

### Marketing — Fase 4e (futuro)
- Métricas de marketing: medir cuántos clientes vinieron al restaurante después de recibir un mensaje de campaña.

## Tareas pendientes

~~35. Falta una documentación de la API para el chatbot.~~ → 283A-15

~~36. Falta crear una documentación completa y detallada del framework glory-rs.~~ → 283A-16 (documentación creada). **Pendiente:** reorganizar repos (glory-rs para framework, glory-rs-template para template). Esta parte requiere operaciones git destructivas entre repos remotos — ver plan en `Agente/planes/plan-glory-rs-template-2026-03-28.md`.

~~37. Dentro del contenido de las tabs no hay gap en el dashboard, todo se ve pegado, debería haber.~~ → 283A-7

~~37.1. Las acciones rápidas están mal posicionadas, deberían ir en la esquina arriba a la misma altura de elegir la fecha, pero a la derecha.~~ → 283A-7

~~37.2 Los botones siguiente y anterior del calendario se ven mal.~~ → 283A-11

~~39. Warning: Function components cannot be given refs (forwardRef) en SidebarMenuButton y Button — cuando entro al panel hay warnings de consola. Además los 401 Unauthorized son porque el backend no estaba corriendo, pero los warnings de forwardRef deben corregirse.~~ → 283A-6

~~40. Los botones de venta y gasto en el panel lateral deberían abrir el modal de crear venta o gasto en vez de ir a la página, ya hay botones para eso.~~ → 283A-10

~~41. Cambiar dark mode a white sigue sin funcionar.~~ → 283A-6

~~42. El titulo de la pestaña en el navegador dice Glory RS, ajustar esto en todos lados para que diga cosas coherente~~ → 283A-11

~~43. El modal de Nuevo Gasto debe ser un poco mas grande.~~ → 283A-11

~~44. Lo de "Funcionalidad de digitalización próximamente disponible" usemos groq IA, y que la api se pueda configurar en configuración, investiga cual IA de groq es la mejor para esto para que funcione bien, como adicional, cualquier herramienta adicional que no sea ia que se pueda complementar usarl~~ → 283A-8a para comparar resultados y mejorar precisión.

~~45. Esta es urgente:

Si dice que el backen no corre porque no lo corre el comando ./dev.ps1 y si lo corre porque da 401 aún? 
PS C:\Users\Owner\OneDrive\Documentos\glory-rust-template> .\dev.ps1
Iniciando modo dev...
Logs guardados en: logs\dev_2026-03-28_0305.log

> dev
> concurrently --names BACK,FRONT --prefix-colors blue,green "cargo run --bin glory-backend" "npm --prefix frontend run dev"

[FRONT] 
[FRONT] > glory-frontend@0.1.0 dev
[FRONT] > vite
[FRONT] 
[BACK]     Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.15s
[BACK]      Running `target\debug\glory-backend.exe`
[BACK] 2026-03-28T07:05:38.234240Z  INFO glory_backend: Servidor iniciando en 127.0.0.1:3000
[BACK] 2026-03-28T07:05:38.234310Z  INFO glory_backend: Swagger UI disponible en http://127.0.0.1:3000/swagger-ui/
[FRONT] 
[FRONT]   VITE v5.4.21  ready in 1728 ms
[FRONT]~~ → 283A-9 
[FRONT]   Ô×£  Local:   http://localhost:5173/
[FRONT]   Ô×£  Network: use --host to expose
[BACK] 2026-03-28T07:05:56.413062Z DEBUG request{method=GET uri=/api/clientes?page=1&per_page=25 version=HTTP/1.1}: tower_http::trace::on_request: started processing request
[BACK] 2026-03-28T07:05:56.426904Z DEBUG request{method=GET uri=/api/clientes?page=1&per_page=25 version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=17 ms status=401

~~46. Las fechas futuras en el dashboard no deberían de ser seleccionables o aparecer~~ → 283A-12

~~47. Veo que hay planes en Agente\planes, haz todo lo que este pendiente, mueve lo que esta pendiente a completados~~ → 283A-14

~~48. Una revisión de todas las solicitudes del cliente vs comparación de lo que se hizo y lo que falta y hacerse todo.~~ → 283A-13
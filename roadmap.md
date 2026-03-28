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

35. Falta una documentación de la API para el chatbot.

36. Falta crear una documentación completa y detallada del framework glory-rs. También hay que reorganizar repos: https://github.com/1ndoryu/glory-rs para el framework y https://github.com/1ndoryu/glory-rs-template para el template. Cuidado con la pérdida de datos — estamos en rama glory-rs-rest (proyecto restaurante), main es donde va el template.

37. Dentro del contenido de las tabs no hay gap en el dashboard, todo se ve pegado, debería haber.

37.1. Las acciones rápidas están mal posicionadas, deberían ir en la esquina arriba a la misma altura de elegir la fecha, pero a la derecha.

39. Warning: Function components cannot be given refs (forwardRef) en SidebarMenuButton y Button — cuando entro al panel hay warnings de consola. Además los 401 Unauthorized son porque el backend no estaba corriendo, pero los warnings de forwardRef deben corregirse.

40. Los botones de venta y gasto en el panel lateral deberían abrir el modal de crear venta o gasto en vez de ir a la página, ya hay botones para eso.

41. Cambiar dark mode a white sigue sin funcionar.

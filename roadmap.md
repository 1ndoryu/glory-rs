# Plataforma de Gestión de Restaurantes — Roadmap

> **URL producción:** http://restaurante.wandori.us (Swagger UI: /swagger-ui/)
> **URL legacy (no funcional):** ~~http://app-b8s0cks444o0sogo8kg8wcgw.66.94.100.241.sslip.io~~
> **Servidor:** 66.94.100.241 (Coolify, servicio UUID: b8s0cks444o0sogo8kg8wcgw)
> **Deploy:** `.\scripts\deploy.ps1` — wrapper que invoca coolify-manager-rs deploy-service (zero-downtime)
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

## Tareas pendientes

### Robustez Haddock (plan: Agente/planes/plan-haddock-robustez-2026-04-06.md)
(todas las fases completadas — 064A-6 a 064A-12, incluido filtro por estado detectado en revisión)

- Sobre "Sin DB local (SQLx offline), los tests de flujo completo (create→sync→check DB) no son posibles" Entonces haz algo para que se pueda testear, claro que tenemos DB local.

- La tarea final sube todo al servidor.
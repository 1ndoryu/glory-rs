Objetivo: crear un template de Rust + Ts React + OpenAPI + Codegen + Clippy nivel paranoia, para crear sitios web, un solo repositorio, con una pagina sencilla de ejemplo, con el gitignore configurado y todo listo para empezar a desarrollar, adelante a todo, comandos, documentación, he creado https://github.com/1ndoryu/glory-rs para lo subas alli, se tiene que pdoer crear varias ramas para varios sitios y que no haya problema de cambiar a un sitio a otro cambiando de rama, aplica todo lo que por defecto vendría siendo optimización, seguridad, rendimiento, experiencia de desarrolo, etc, de base de datos tengo corriendo postgres aca localmente.

## Estado: ✅ Template base completado (253A-1)

Ver `Agente/completados/tareas-2026-03-25.md` para detalles.

## Stack implementado

| Capa | Herramienta |
|------|-------------|
| Framework web | Axum 0.7 |
| OpenAPI | utoipa 4 + utoipa-swagger-ui 7 |
| Serialización | serde |
| Base de datos | SQLx 0.8 (PostgreSQL) |
| Migraciones | SQLx migrate |
| Validación | validator 0.18 |
| Variables de entorno | dotenvy |
| Logging | tracing + tracing-subscriber |
| Errores | thiserror 2 |
| Auth | jsonwebtoken + argon2 |
| CORS | tower-http |
| Linter | clippy (deny all + warn pedantic) |
| Frontend | React 18 + TypeScript + Vite |
| State | React Query + Zustand |
| Codegen | Orval 8 (reemplaza openapi-typescript-codegen) |

## Pendientes

*(Agregar nuevas tareas aquí)*

## Notas

- Configurar coolify-manager-rs para desplegar proyectos Rust (repo separado)
- Prioridades: 1. Velocidad desarrollo, 2. Decisiones futuras, 3. Rendimiento, 4. Seguridad, 5. Popularidad, 6. Facilidad, 7. Docs, 8. Compatibilidad, 9. Flexibilidad, 10. Escalabilidad


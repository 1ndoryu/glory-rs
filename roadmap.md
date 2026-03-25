Objetivo: crear un template de Rust + Ts React + OpenAPI + Codegen + Clippy nivel paranoia, para crear sitios web, un solo repositorio, con una pagina sencilla de ejemplo, con el gitignore configurado y todo listo para empezar a desarrollar, adelante a todo, comandos, documentación, he creado https://github.com/1ndoryu/glory-rs para lo subas alli, se tiene que pdoer crear varias ramas para varios sitios y que no haya problema de cambiar a un sitio a otro cambiando de rama, aplica todo lo que por defecto vendría siendo optimización, seguridad, rendimiento, experiencia de desarrolo, etc, de base de datos tengo corriendo postgres aca localmente.

## Estado: ✅ Template base completado (253A-1)

Ver `Agente/completados/tareas-2026-03-25.md` para detalles.

## Pendientes

si hay alguna herramienta que consideres que falta o que sea mejor, puedes agregarla o remplazar a una existe: prioridad: 1. Velocidad de desarrollo, 2. Facilitar cualquier decisión futura nueva sobre el proyecto por ejemplo, una app nativa. 3. Rendimiento, 4. Seguridad, 5. Popularidad de la herramienta (comunidad, mantenimiento, etc), 6. Facilidad de uso, 7. Documentación, 8. Compatibilidad con otras herramientas, 9. Flexibilidad y personalización, 10. Escalabilidad.

Capa                │ Herramienta         │ ¿Para qué?
────────────────────┼─────────────────────┼──────────────────────────
Framework web       │ Axum                │ HTTP, routing, middleware
OpenAPI generation  │ utoipa + utoipa-swagger-ui │ Genera esquema OpenAPI desde código
Serialización       │ serde               │ JSON ↔ Structs
Base de datos       │ SQLx                │ Queries SQL con verificación en compilación
Migraciones         │ SQLx (integrado)    │ Control de esquema DB
Validación          │ validator           │ Validar inputs del usuario
Variables de entorno│ dotenvy             │ Cargar .env
Configuración       │ config (crate)      │ Settings por entorno
Logging             │ tracing + tracing-subscriber │ Logs estructurados
Manejo de errores   │ thiserror           │ Errores tipados y limpios
Auth (JWT)          │ jsonwebtoken        │ Tokens
Hashing passwords   │ argon2              │ Hashing seguro
CORS                │ tower-http          │ Middleware CORS
Testing             │ cargo test + reqwest│ Tests integración
Linter              │ clippy (paranoia)   │ Código limpio
Codegen frontend    │ openapi-typescript-codegen │ Genera cliente TS
────────────────────┼─────────────────────┼──────────────────────────

Un paso extra sería que falta configurar /coolify-manager-rs (lo he dejado en el entorno), para que se pueda desplegar proyectos rust 


# NOTA: 

planifica mejor este roadmap

## OTRA NOTA:

Veo que el front me carca errores. Todos los errores de backend y front deberían ser detectados con un solo comando para facilitar el desarrollo. 
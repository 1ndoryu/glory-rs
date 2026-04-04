Objetivo: Nakomi Studio — sitio web de agencia creativa. Migrado de WordPress a Rust (Axum) + React SPA.
Rama: glory-rust-nakomi
Roadmap de tareas del proyecto: App/roadmap.md

## Estado: Migración en progreso (044A-1)

## Stack implementado

| Capa                 | Herramienta                                    |
| -------------------- | ---------------------------------------------- |
| Framework web        | Axum 0.7                                       |
| OpenAPI              | utoipa 4 + utoipa-swagger-ui 7                 |
| Serialización        | serde                                          |
| Base de datos        | SQLx 0.8 (PostgreSQL)                          |
| Migraciones          | SQLx migrate                                   |
| Validación           | validator 0.18                                 |
| Variables de entorno | dotenvy                                        |
| Logging              | tracing + tracing-subscriber                   |
| Errores              | thiserror 2                                    |
| Auth                 | jsonwebtoken + argon2                          |
| CORS                 | tower-http                                     |
| Linter               | clippy (deny all + warn pedantic)              |
| Frontend             | React 18 + TypeScript + Vite                   |
| State                | React Query + Zustand                          |
| Codegen              | Orval 8 (reemplaza openapi-typescript-codegen) |

## Pendientes



# Nakomi Studio — Roadmap

## Contexto
Proyecto migrado de WordPress a Rust (Axum) + React SPA. El frontend React de App/React/ se integra en frontend/src/. El backend PHP se reemplaza por el template Rust.

---

## Pendientes (por prioridad — lo más difícil primero)

### 044A-1: Migrar frontend React de islands (WordPress) a SPA (Rust + Vite)
- Mover componentes, hooks, data, types, styles de App/React/ a frontend/src/
- Mover assets de App/Assets/ a frontend/public/assets/
- Reemplazar navegacionSPA.tsx (interceptor PHP) por react-router-dom
- Reemplazar appIslands.tsx por rutas React Router
- Eliminar dependencia de GLORY_CONTEXT (usar datos directos de data/)
- Instalar deps faltantes (lucide-react)
- Verificar compilación: npm run type-check + npm run build

### 044A-2: i18n triple (EN/ES/JP) con selector y detección de navegador
- Implementar sistema de traducciones (react-i18next o similar)
- Selector de idioma en header
- Detección automática del idioma del navegador
- Traducir todos los textos estáticos

### 044A-3: Páginas detalle de proyectos — Kamples y Mabuhay
- Kamples: biblioteca de samples con algoritmo, DAW, funcionalidades de red social, código abierto, online en kamples.com
- Mabuhay: agencia de viajes en España
- Imágenes en App/Assets/Kamples y App/Assets/Mabuhay
- Ajustar proporción de la caja de imágenes al tamaño real (no 1:1)
- Ajustar Capabilities para cada proyecto
- Mejorar redacción y contenido de cada proyecto

### 044A-4: Blog — contenido real sobre Kamples
- Un solo post real por ahora
- Kamples: plataforma con funcionalidades de red social, organiza samples, comparte como Pinterest (colecciones/tableros), "ver más ideas" como Pinterest, funciona como WhoSampled
- Eliminar posts placeholder

### 044A-5: Ajustar textos de servicios + consistencia
- Revisar que todos los servicios tengan textos coherentes
- seccionGaleriaServicio debe ser visualmente igual a carruselContenedorPrincipal

### 044A-6: Quitar clientes y testimonials del home
- Eliminar SeccionClientes del home
- Eliminar SeccionTestimonios del home

### 044A-7: Ocultar páginas VPS y agentes IA
- Ocultar temporalmente de la navegación
- Mantener el código pero no mostrar en el menú ni rutas activas

### 044A-8: Arreglar problema de git push
- Diagnosticar y corregir el issue al subir cambios

### 044A-9: Página de hosting + adaptar Coolify Manager
- Tarea compleja, requiere planificación detallada
- Crear plan en App/Agente/planes/ antes de ejecutar

# syntax=docker/dockerfile:1.6
#
# [174A-113] Dockerfile multi-stage para el frontend Vite -> nginx estatico.
#
# Decisiones:
#  - Builder usa node:20-alpine (npm 10) para evitar mismatch con npm 11
#    del dev local (lección deploy-lessons.md "npm ci vs npm install").
#  - Usamos `npm install --no-audit --no-fund` en lugar de `npm ci` para ser
#    tolerantes a diferencias de lockfile entre node 20/24.
#  - El frontend NO necesita el OpenAPI schema en build-time porque el código
#    generado por Orval (`src/api/generated/`) está versionado en git… o no.
#    Si NO está versionado: el build debe primero correr `npm run codegen`
#    apuntando al backend, lo cual NO se puede en este contenedor aislado.
#    Solución: el codegen se hace en CI antes del build, o el repo lo
#    versiona. Aquí asumimos que `src/api/generated/` ya existe (CI lo
#    genera).
#  - Runtime nginx:alpine con configuración SPA (history fallback) y proxy
#    /api -> backend.

############
# Builder  #
############
FROM node:20-alpine AS builder

WORKDIR /app

# Copiar manifests primero para cachear node_modules
COPY package.json package-lock.json* ./

# install (no ci) para tolerar mismatch de lockfile entre node 20/24
RUN npm install --no-audit --no-fund

# Copiar el resto del código frontend
COPY tsconfig.json tsconfig.node.json* vite.config.ts orval.config.ts index.html ./
COPY src ./src
COPY public ./public

# Copiamos el openapi.json si existe en el contexto (se monta desde la raíz
# del repo en CI o se commitea). Si no está, el codegen falla y por tanto
# el build también — eso es deseable: una imagen frontend sin tipos del API
# es inservible.
COPY openapi.json* ./

# Si no hay src/api/generated/ aún (caso fresh-build), generar.
# El comando `codegen` corre orval contra openapi.json local.
RUN if [ ! -d src/api/generated ] || [ -z "$(ls -A src/api/generated 2>/dev/null)" ]; then \
        echo "Generando cliente Orval desde openapi.json local..." && \
        npm run codegen; \
    else \
        echo "src/api/generated/ ya existe; saltando codegen."; \
    fi

# Build de producción (vite build emite a dist/)
RUN npm run build

############
# Runtime  #
############
FROM nginx:1.27-alpine AS runtime

# Copiar config nginx custom (SPA history fallback + proxy /api/ -> app:3000)
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copiar artefactos estáticos
COPY --from=builder /app/dist /usr/share/nginx/html

# Healthcheck simple: /index.html debe responder 200
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD wget --quiet --spider http://127.0.0.1:80/ || exit 1

EXPOSE 80

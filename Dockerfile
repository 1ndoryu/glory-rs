# [263A-20] Dockerfile multi-stage para compilar y ejecutar la app Rust.
# Stage 1: compilar backend Rust
# Stage 2: compilar frontend React (Vite build)
# Stage 3: imagen final mínima con binario + assets estáticos

# --- Stage 1: Build backend ---
FROM rust:1.83-bookworm AS backend-builder

WORKDIR /app

# Copiar manifests primero para cachear dependencias
COPY Cargo.toml Cargo.lock* ./
COPY .sqlx/ .sqlx/
RUN mkdir src && echo 'fn main() {}' > src/main.rs && \
    mkdir -p src/bin && echo 'fn main() {}' > src/bin/dump_openapi.rs && \
    echo 'fn main() {}' > src/bin/seed.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src

# Copiar fuente real y compilar
COPY src/ src/
COPY migrations/ migrations/
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin glory-backend

# --- Stage 2: Build frontend ---
FROM node:20-slim AS frontend-builder

# Instalar dependencias del submodulo Glory (tipos React)
WORKDIR /app/glory-rs/frontend
COPY glory-rs/frontend/package.json glory-rs/frontend/package-lock.json* ./
RUN npm install --ignore-scripts

# Instalar dependencias del frontend principal
WORKDIR /app/frontend
COPY frontend/package.json frontend/package-lock.json* ./
RUN npm ci --ignore-scripts

# Copiar fuentes y compilar
COPY frontend/ ./
COPY glory-rs/frontend/ /app/glory-rs/frontend/
RUN npm run build

# --- Stage 3: Runtime ---
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Binario backend
COPY --from=backend-builder /app/target/release/glory-backend /app/glory-backend

# Assets frontend (Vite genera dist/)
COPY --from=frontend-builder /app/frontend/dist /app/static

# Migraciones
COPY migrations/ /app/migrations/

# Puerto por defecto (configurable con PORT env var)
ENV HOST=0.0.0.0
ENV PORT=3000

EXPOSE 3000

CMD ["/app/glory-backend"]

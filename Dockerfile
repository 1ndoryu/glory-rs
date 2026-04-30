# syntax=docker/dockerfile:1.6
#
# [174A-112] Dockerfile multi-stage para el backend Axum.
#
# Decisiones:
#  - Builder usa imagen `rust:1-bookworm` (incluye ca-certificates).
#  - Runtime usa `debian:bookworm-slim` para imagen final pequeña con todas
#    las libs que `aws-lc-rs`, `sqlx tls-rustls` y `reqwest` necesitan en
#    runtime (libssl no, pero sí ca-certificates y libgcc).
#  - SQLX_OFFLINE=true en build: usa los snapshots de `.sqlx/` y NO necesita
#    una BD viva durante el build. CRÍTICO: si modificas un `sqlx::query!`
#    debes correr `cargo sqlx prepare` y commitear `.sqlx/` antes de
#    desplegar.
#  - NO usamos `--mount=type=cache,target=/app/target` porque cuando el
#    contexto de build proviene de un git checkout, ese cache puede
#    enmascarar cambios reales del código fuente (lección user memory:
#    deploy-lessons.md "Docker BuildKit Mount Cache CRITICAL").
#  - Sí usamos cache de cargo registry/git porque solo guarda crates
#    descargados.

############
# Builder  #
############
FROM rust:1-bookworm AS builder

WORKDIR /app

# Instalar utilidades necesarias para compilar dependencias nativas
# (aws-lc-rs requiere clang/cmake/perl para compilar AWS-LC desde fuente)
RUN apt-get update && apt-get install -y --no-install-recommends \
        pkg-config \
        clang \
        cmake \
        perl \
        build-essential \
        lld \
    && rm -rf /var/lib/apt/lists/*

# [294A-1] Linker rápido (lld) para Linux. Reduce 30-60s en cada link de
# release. Compatible con todas las deps actuales.
ENV RUSTFLAGS="-C link-arg=-fuse-ld=lld"

# ---------------------------------------------------------------------------
# [294A-1] Capa 1: build solo de dependencias (cacheable).
# Truco "cargo-chef manual": copiamos únicamente los Cargo.toml + un main.rs
# vacío, hacemos build de release, y luego invalidamos sólo nuestro código.
# Cuando solo cambia src/, esta capa se reutiliza y el rebuild tarda
# minutos en lugar de 10+ minutos (compilar ~600 crates).
# ---------------------------------------------------------------------------
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY glory-rs/backend/Cargo.toml ./glory-rs/backend/Cargo.toml

# Stub mínimo para que `cargo build` resuelva el grafo y compile deps.
RUN mkdir -p src glory-rs/backend/src && \
    echo 'fn main() { println!("stub"); }' > src/main.rs && \
    echo '' > glory-rs/backend/src/lib.rs

ENV SQLX_OFFLINE=true
ENV CARGO_TERM_COLOR=always

# Build de dependencias. Cacheamos registry y git de cargo, NO el target/
# (lección: deploy-lessons.md "Docker BuildKit Mount Cache CRITICAL").
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release --bin glory-backend ; \
    rm -rf src glory-rs/backend/src

# ---------------------------------------------------------------------------
# Capa 2: build real con nuestro código fuente.
# ---------------------------------------------------------------------------
COPY .sqlx ./.sqlx
COPY glory-rs ./glory-rs
COPY migrations ./migrations
COPY src ./src

# touch para forzar a cargo a recompilar SOLO nuestros crates (las deps
# precompiladas en la capa anterior siguen cacheadas en target/).
RUN find src glory-rs/backend/src -name '*.rs' -exec touch {} +

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release --bin glory-backend && \
    cp target/release/glory-backend /app/glory-backend

############
# Runtime  #
############
FROM debian:bookworm-slim AS runtime

# ca-certificates: necesario para reqwest/lettre/sqlx TLS hacia servicios
#   externos (Stripe, Groq, OpenAI, FCM, SMTP, S3, etc.).
# libgcc: requerido por binarios Rust que usan stdlib threading.
# tini: PID 1 limpio que reapea zombies y propaga señales.
RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
        libgcc-s1 \
        tini \
        curl \
    && rm -rf /var/lib/apt/lists/*

# Usuario no-root: limita el impacto si el binario es comprometido.
# UID/GID fijos (1000) para que coincidan con el dueño del bind mount
# /data/uploads en el host (Coolify ejecuta como uid 1000 por defecto).
RUN groupadd --system --gid 1000 app && \
    useradd --system --uid 1000 --gid app --create-home --shell /sbin/nologin app

WORKDIR /app

COPY --from=builder /app/glory-backend /usr/local/bin/glory-backend
COPY --from=builder /app/migrations /app/migrations

# Directorios mutables — el storage local va a /data/uploads (volume host),
# los logs van a /app/logs (efímero, opcional volume si se quiere persistir).
RUN mkdir -p /data/uploads /app/logs && \
    chown -R app:app /data/uploads /app/logs /app

USER app

EXPOSE 3000

# Healthcheck: el backend expone /api/health. Si tarda >5s o devuelve no-2xx,
# Docker marca el contenedor unhealthy y Coolify/orquestador puede actuar.
HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
    CMD curl --fail --silent --show-error http://127.0.0.1:3000/api/health || exit 1

ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["glory-backend"]

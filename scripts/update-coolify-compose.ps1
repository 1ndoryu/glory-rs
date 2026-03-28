# [283A-33] Script para actualizar el compose del servicio glory-rest en Coolify
$envFile = "c:\Users\Owner\OneDrive\Documentos\WP\app\public\wp-content\themes\glorytemplate\.agent\coolify-manager-rs\.env"
$token = ((Get-Content $envFile | Where-Object { $_ -match '^COOLIFY_VPS1_API_TOKEN=' }) -replace 'COOLIFY_VPS1_API_TOKEN=','').Trim()

$compose = @'
services:
  app:
    build:
      context: .
      dockerfile_inline: |
        FROM rust:1-bookworm AS backend-builder
        WORKDIR /app
        RUN apt-get update && apt-get install -y --no-install-recommends git && rm -rf /var/lib/apt/lists/*
        RUN git clone --branch glory-rs-rest --depth 1 --recurse-submodules https://github.com/1ndoryu/glory-rs.git .
        ENV SQLX_OFFLINE=true
        RUN cargo build --release --bin glory-backend
        FROM node:20-slim AS frontend-builder
        WORKDIR /app
        RUN apt-get update && apt-get install -y --no-install-recommends git && rm -rf /var/lib/apt/lists/*
        RUN git clone --branch glory-rs-rest --depth 1 --recurse-submodules https://github.com/1ndoryu/glory-rs.git .
        WORKDIR /app/glory-rs
        RUN npm install --ignore-scripts
        WORKDIR /app/frontend
        RUN npm ci --ignore-scripts
        RUN npm run build
        FROM debian:bookworm-slim
        RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl && rm -rf /var/lib/apt/lists/*
        WORKDIR /app
        COPY --from=backend-builder /app/target/release/glory-backend /app/glory-backend
        COPY --from=frontend-builder /app/frontend/dist /app/static
        COPY --from=backend-builder /app/migrations/ /app/migrations/
        ENV HOST=0.0.0.0
        ENV PORT=3000
        EXPOSE 3000
        CMD ["/app/glory-backend"]
    environment:
      DATABASE_URL: 'postgres://glory_app:${DB_PASSWORD}@postgres:5432/glory'
      JWT_SECRET: '${JWT_SECRET}'
      HOST: '0.0.0.0'
      PORT: '3000'
      STATIC_DIR: /app/static
      APP_URL: 'http://app-b8s0cks444o0sogo8kg8wcgw.66.94.100.241.sslip.io'
      SERVICE_FQDN_APP_3000: 'http://app-b8s0cks444o0sogo8kg8wcgw.66.94.100.241.sslip.io'
    depends_on:
      postgres:
        condition: service_healthy
    healthcheck:
      test: ['CMD', 'curl', '-f', 'http://localhost:3000/swagger-ui/']
      interval: 30s
      timeout: 10s
      retries: 5
      start_period: 180s
    restart: unless-stopped
  postgres:
    image: 'postgres:16-alpine'
    volumes:
      - 'pg_data:/var/lib/postgresql/data'
    environment:
      POSTGRES_DB: glory
      POSTGRES_USER: glory_app
      POSTGRES_PASSWORD: '${DB_PASSWORD}'
    healthcheck:
      test: ['CMD-SHELL', 'pg_isready -U glory_app -d glory']
      interval: 10s
      timeout: 5s
      retries: 5
volumes:
  pg_data:
'@

$composeB64 = [Convert]::ToBase64String([System.Text.Encoding]::UTF8.GetBytes($compose))
$body = "{`"docker_compose_raw`": `"$composeB64`"}"

$headers = @{
    Authorization = "Bearer $token"
    Accept = "application/json"
    "Content-Type" = "application/json"
}

try {
    $resp = Invoke-RestMethod -Uri "http://66.94.100.241:8000/api/v1/services/b8s0cks444o0sogo8kg8wcgw" -Headers $headers -Method Patch -Body $body
    Write-Host "PATCH exitoso"
    $resp | ConvertTo-Json -Depth 2 | Write-Host
} catch {
    Write-Host "Error: $($_.Exception.Message)"
    if ($_.ErrorDetails) { Write-Host "Detail: $($_.ErrorDetails.Message)" }
}

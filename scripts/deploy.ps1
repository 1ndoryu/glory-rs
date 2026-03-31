# [303A-8] Deploy zero-downtime para glory-rest.
# Construye la imagen nueva via SSH MIENTRAS el contenedor viejo sigue sirviendo.
# Luego hace swap instantaneo con docker compose up -d.
# Nunca usa Coolify restart (mata contenedores antes de construir = 404).
#
# Uso: .\scripts\deploy.ps1
#       .\scripts\deploy.ps1 -Seed     (ejecuta datos de prueba despues del deploy)
#       .\scripts\deploy.ps1 -SkipBuild (solo swap, asume imagen ya construida)

param(
    [int]$TimeoutMinutos = 15,
    [switch]$Seed,
    [switch]$SkipBuild
)

$ErrorActionPreference = 'Stop'

$serverIp = "66.94.100.241"
$serverUser = "root"
$sshTarget = "$serverUser@$serverIp"
$serviceDir = "/data/coolify/services/b8s0cks444o0sogo8kg8wcgw"
$serviceNetwork = "b8s0cks444o0sogo8kg8wcgw"
$healthUrl = "http://restaurante.wandori.us/api/health"

function Invoke-Ssh {
    param([string]$Command)
    $result = ssh -o StrictHostKeyChecking=no -o ConnectTimeout=10 $sshTarget $Command 2>&1
    if ($LASTEXITCODE -ne 0) {
        $msg = if ($result) { $result -join "`n" } else { "SSH command failed with exit code $LASTEXITCODE" }
        throw $msg
    }
    return $result
}

# --- 1. Sincronizar compose con Coolify API ---
Write-Host "[1/5] Sincronizando compose con Coolify..." -ForegroundColor Cyan
& "$PSScriptRoot\update-coolify-compose.ps1"
Write-Host "       Compose sincronizado." -ForegroundColor Green

# --- 2. Asegurar postgres esta corriendo ---
Write-Host "[2/5] Verificando postgres..." -ForegroundColor Cyan
$pgStatus = Invoke-Ssh "cd $serviceDir && docker compose ps postgres --format '{{.Status}}' 2>/dev/null"
if (-not $pgStatus -or $pgStatus -notmatch "Up|running|healthy") {
    Write-Host "       Postgres no esta corriendo, iniciando..." -ForegroundColor Yellow
    Invoke-Ssh "cd $serviceDir && docker compose up -d postgres 2>&1"
    # Esperar a que postgres este healthy
    $pgDeadline = (Get-Date).AddSeconds(60)
    while ((Get-Date) -lt $pgDeadline) {
        Start-Sleep -Seconds 5
        $pgHealth = Invoke-Ssh "cd $serviceDir && docker compose ps postgres --format '{{.Status}}' 2>/dev/null"
        if ($pgHealth -match "healthy") { break }
    }
}
Write-Host "       Postgres OK." -ForegroundColor Green

# --- 3. Build imagen nueva (viejo contenedor sigue sirviendo) ---
if (-not $SkipBuild) {
    Write-Host "[3/5] Construyendo imagen nueva (el servicio sigue activo)..." -ForegroundColor Cyan
    Write-Host "       Esto toma ~5-7 min. No hay downtime." -ForegroundColor DarkGray
    $buildStart = Get-Date

    # Build via SSH — --no-cache para invalidar git clone (cargo usa mount cache, no se pierde)
    $buildOutput = ssh -o StrictHostKeyChecking=no -o ConnectTimeout=10 $sshTarget "cd $serviceDir && docker compose build app --no-cache --progress=plain 2>&1 | tail -30"
    $buildEnd = Get-Date
    $buildDuration = [math]::Round(($buildEnd - $buildStart).TotalSeconds)

    if ($LASTEXITCODE -ne 0) {
        Write-Host "       BUILD FALLO ($buildDuration s):" -ForegroundColor Red
        Write-Host ($buildOutput -join "`n") -ForegroundColor Red
        exit 1
    }
    Write-Host "       Build completado en $buildDuration s." -ForegroundColor Green
} else {
    Write-Host "[3/5] Build omitido (-SkipBuild)." -ForegroundColor Yellow
}

# --- 4. Swap instantaneo: recrear solo app ---
Write-Host "[4/5] Swap: reemplazando contenedor app..." -ForegroundColor Cyan
Invoke-Ssh "cd $serviceDir && docker compose up -d --no-build app 2>&1"

# Asegurar que Traefik puede alcanzar el contenedor
$traefikConnected = Invoke-Ssh "docker network inspect $serviceNetwork --format '{{range .Containers}}{{.Name}} {{end}}' 2>/dev/null"
if ($traefikConnected -notmatch "coolify-proxy") {
    Write-Host "       Conectando Traefik a la red del servicio..." -ForegroundColor Yellow
    Invoke-Ssh "docker network connect $serviceNetwork coolify-proxy 2>/dev/null || true"
}
Write-Host "       Contenedor reemplazado." -ForegroundColor Green

# --- 5. Health check ---
Write-Host "[5/5] Verificando salud..." -ForegroundColor Cyan
$deadline = (Get-Date).AddSeconds(120)
$ok = $false

while ((Get-Date) -lt $deadline) {
    Start-Sleep -Seconds 5
    try {
        $resp = Invoke-WebRequest -Uri $healthUrl -UseBasicParsing -TimeoutSec 10
        if ($resp.StatusCode -eq 200) {
            $ok = $true
            break
        }
    } catch {
        $remaining = [math]::Round(($deadline - (Get-Date)).TotalSeconds)
        Write-Host "       Esperando... ($remaining s restantes)" -ForegroundColor DarkGray
    }
}

if ($ok) {
    Write-Host ""
    Write-Host "Deploy exitoso! $healthUrl respondiendo." -ForegroundColor Green

    if ($Seed) {
        Write-Host "Ejecutando seed de datos de prueba..." -ForegroundColor Cyan
        Invoke-Ssh "cd $serviceDir && docker compose exec app /app/seed 2>&1"
        Write-Host "Seed completado." -ForegroundColor Green
    }
} else {
    Write-Host ""
    Write-Host "TIMEOUT: El servicio no respondio en 120s." -ForegroundColor Red
    Write-Host "Revisando logs del contenedor:" -ForegroundColor Yellow
    $logs = Invoke-Ssh "cd $serviceDir && docker compose logs app --tail 20 2>&1"
    Write-Host ($logs -join "`n")
    exit 1
}

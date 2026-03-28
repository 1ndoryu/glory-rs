# [283A-35] Deploy simplificado para glory-rest.
# Uso: .\scripts\deploy.ps1
#       .\scripts\deploy.ps1 -Seed    (ejecuta datos de prueba despues del deploy)
# Actualiza el compose en Coolify, reinicia el servicio y verifica que responda.

param(
    [int]$TimeoutMinutos = 15,
    [switch]$Seed
)

$ErrorActionPreference = 'Stop'
$envFile = "c:\Users\Owner\OneDrive\Documentos\WP\app\public\wp-content\themes\glorytemplate\.agent\coolify-manager-rs\.env"
$serviceUuid = "b8s0cks444o0sogo8kg8wcgw"
$baseApi = "http://66.94.100.241:8000"
$healthUrl = "http://app-b8s0cks444o0sogo8kg8wcgw.66.94.100.241.sslip.io/swagger-ui/"

# --- 1. Cargar token ---
$token = ((Get-Content $envFile | Where-Object { $_ -match '^COOLIFY_VPS1_API_TOKEN=' }) -replace 'COOLIFY_VPS1_API_TOKEN=','').Trim()
if (-not $token) { Write-Error "No se encontro COOLIFY_VPS1_API_TOKEN en $envFile"; exit 1 }

$headers = @{
    Authorization = "Bearer $token"
    Accept = "application/json"
    "Content-Type" = "application/json"
}

# --- 2. Actualizar compose ---
Write-Host "[1/4] Actualizando compose en Coolify..." -ForegroundColor Cyan
& "$PSScriptRoot\update-coolify-compose.ps1"
Write-Host "       Compose actualizado." -ForegroundColor Green

# --- 3. Restart servicio ---
Write-Host "[2/4] Reiniciando servicio..." -ForegroundColor Cyan
try {
    Invoke-RestMethod -Uri "$baseApi/api/v1/services/$serviceUuid/restart" -Headers $headers -Method Post | Out-Null
} catch {
    # Si esta detenido, iniciar
    Write-Host "       Servicio detenido, iniciando..." -ForegroundColor Yellow
    Invoke-RestMethod -Uri "$baseApi/api/v1/services/$serviceUuid/start" -Headers $headers -Method Post | Out-Null
}
Write-Host "       Build iniciado en el servidor." -ForegroundColor Green

# --- 4. Esperar health check ---
Write-Host "[3/4] Esperando que el servicio responda (max $TimeoutMinutos min)..." -ForegroundColor Cyan
$deadline = (Get-Date).AddMinutes($TimeoutMinutos)
$ok = $false

while ((Get-Date) -lt $deadline) {
    Start-Sleep -Seconds 30
    try {
        $resp = Invoke-WebRequest -Uri $healthUrl -UseBasicParsing -TimeoutSec 10
        if ($resp.StatusCode -eq 200) {
            $ok = $true
            break
        }
    } catch {
        Write-Host "       Aun compilando... ($([math]::Round(($deadline - (Get-Date)).TotalMinutes, 1)) min restantes)" -ForegroundColor DarkGray
    }
}

if ($ok) {
    Write-Host "[4/4] Deploy exitoso! Servicio respondiendo en $healthUrl" -ForegroundColor Green
    if ($Seed) {
        Write-Host "" 
        Write-Host "Ejecutando seed de datos de prueba..." -ForegroundColor Cyan
        $cmExe = "c:\Users\Owner\OneDrive\Documentos\WP\app\public\wp-content\themes\glorytemplate\.agent\coolify-manager-rs\target\release\coolify-manager.exe"
        $configPath = "c:\Users\Owner\OneDrive\Documentos\WP\app\public\wp-content\themes\glorytemplate\.agent\coolify-manager-rs\config\settings.json"
        & $cmExe -c $configPath exec --name glory-rest --command "/app/seed"
        Write-Host "Seed completado." -ForegroundColor Green
    }
} else {
    Write-Host "[4/4] TIMEOUT: El servicio no respondio en $TimeoutMinutos minutos." -ForegroundColor Red
    Write-Host "       Verifica los logs en Coolify: $baseApi" -ForegroundColor Yellow
    exit 1
}

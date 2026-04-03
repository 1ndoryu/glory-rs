# [044A-1] Deploy zero-downtime via coolify-manager-rs.
# Wrapper agnostico: toda la logica de deploy vive en coolify-manager-rs deploy-service.
# Este script solo localiza el binario y reenvía los flags.
#
# Uso: .\scripts\deploy.ps1
#       .\scripts\deploy.ps1 -Seed     (ejecuta datos de prueba despues del deploy)
#       .\scripts\deploy.ps1 -SkipBuild (solo swap, asume imagen ya construida)
#       .\scripts\deploy.ps1 -SkipComposeSync (no sincronizar compose con Coolify API)

param(
    [switch]$Seed,
    [switch]$SkipBuild,
    [switch]$SkipComposeSync
)

$ErrorActionPreference = 'Stop'

# --- Localizar binario de coolify-manager ---
$cmBinary = $null
$candidates = @(
    "$PSScriptRoot\..\..\..\WP\app\public\wp-content\themes\glorytemplate\.agent\coolify-manager-rs\target\release\coolify-manager.exe",
    "c:\Users\Owner\OneDrive\Documentos\WP\app\public\wp-content\themes\glorytemplate\.agent\coolify-manager-rs\target\release\coolify-manager.exe"
)

foreach ($path in $candidates) {
    $resolved = [System.IO.Path]::GetFullPath($path)
    if (Test-Path $resolved) {
        $cmBinary = $resolved
        break
    }
}

if (-not $cmBinary) {
    Write-Host "ERROR: No se encontro coolify-manager.exe" -ForegroundColor Red
    Write-Host "Candidatos buscados:" -ForegroundColor Yellow
    foreach ($c in $candidates) { Write-Host "  $([System.IO.Path]::GetFullPath($c))" }
    exit 1
}

# --- Localizar config ---
$cmDir = Split-Path (Split-Path (Split-Path $cmBinary -Parent) -Parent) -Parent
$configPath = Join-Path $cmDir "config\settings.json"

if (-not (Test-Path $configPath)) {
    Write-Host "ERROR: No se encontro settings.json en $configPath" -ForegroundColor Red
    exit 1
}

# --- Nombre del sitio (configurable por proyecto, default glory-rest) ---
$siteName = if ($env:COOLIFY_SITE_NAME) { $env:COOLIFY_SITE_NAME } else { "glory-rest" }

# --- Construir argumentos ---
$args = @("deploy-service", "--name", $siteName, "--config", $configPath)

if ($SkipBuild) { $args += "--skip-build" }
if ($Seed) { $args += "--seed" }
if ($SkipComposeSync) { $args += "--skip-compose-sync" }

Write-Host "Ejecutando: coolify-manager $($args -join ' ')" -ForegroundColor Cyan
& $cmBinary @args

if ($LASTEXITCODE -ne 0) {
    Write-Host "Deploy fallo (exit code: $LASTEXITCODE)" -ForegroundColor Red
    exit $LASTEXITCODE
}


# dev.ps1 — inicia backend + frontend en modo dev con logs guardados en archivo
# Uso: .\dev.ps1

$null = New-Item -ItemType Directory -Force -Path "logs"
$timestamp = Get-Date -Format "yyyy-MM-dd_HHmm"
$logFile = "logs\dev_$timestamp.log"
Write-Host "Iniciando modo dev..." -ForegroundColor Cyan
Write-Host "Logs guardados en: $logFile" -ForegroundColor Yellow

npm run dev 2>&1 | Tee-Object -FilePath $logFile

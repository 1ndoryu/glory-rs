param(
    [Parameter(Mandatory = $false)]
    [string]$TareaId
)

$ErrorActionPreference = 'Stop'

if ([string]::IsNullOrWhiteSpace($TareaId)) {
    Write-Error 'Uso: npm run self-check -- -TareaId 174A-XX'
    exit 1
}

$fallos = New-Object System.Collections.Generic.List[string]

function Invoke-Step {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Label,
        [Parameter(Mandatory = $true)]
        [scriptblock]$Action
    )

    Write-Host "[self-check] $Label"
    try {
        & $Action
        Write-Host "[self-check] OK - $Label"
    }
    catch {
        $fallos.Add("$Label -> $($_.Exception.Message)")
        Write-Host "[self-check] FAIL - $Label"
    }
}

function Assert-LastExitCode {
    param([string]$Label)
    if ($LASTEXITCODE -ne 0) {
        throw "$Label devolvio exit code $LASTEXITCODE"
    }
}

Write-Host "[self-check] Tarea: $TareaId"
Write-Host '[self-check] Checklist de protocolo:'
Write-Host '[OBLIGATORIA] roadmap leido y actualizado'
Write-Host '[OBLIGATORIA] validaciones del stack en verde'
Write-Host '[OBLIGATORIA] tests ejecutados si aplican'
Write-Host '[OBLIGATORIA] tarea archivada en Agente/completados/'
Write-Host '[OBLIGATORIA] commit por tarea y push inmediato'
Write-Host '[CONDICIONAL] documentacion actualizada si se toco funcionalidad'
Write-Host '[CONDICIONAL] regla Sentinel si el bug era prevenible'

if (Test-Path 'Cargo.toml') {
    Invoke-Step -Label 'cargo check' -Action {
        cargo check | Out-Host
        Assert-LastExitCode 'cargo check'
    }
    Invoke-Step -Label 'cargo clippy --all-targets -- -D warnings' -Action {
        cargo clippy --all-targets -- -D warnings | Out-Host
        Assert-LastExitCode 'cargo clippy'
    }
    Invoke-Step -Label 'cargo test' -Action {
        cargo test | Out-Host
        Assert-LastExitCode 'cargo test'
    }
}

if (Test-Path 'frontend/package.json') {
    Invoke-Step -Label 'frontend type-check' -Action {
        npm --prefix frontend run type-check | Out-Host
        Assert-LastExitCode 'frontend type-check'
    }
}

if (Test-Path 'scripts/check-roadmap.mjs') {
    Write-Host '[self-check] Roadmap actual:'
    node scripts/check-roadmap.mjs | Out-Host
    if ($LASTEXITCODE -gt 1) {
        $fallos.Add("roadmap watcher devolvio exit code $LASTEXITCODE")
    }
}

if ($fallos.Count -gt 0) {
    Write-Host '[self-check] Fallos detectados:'
    foreach ($fallo in $fallos) {
        Write-Host " - $fallo"
    }
    exit 1
}

Write-Host "[self-check] OK para $TareaId"
exit 0
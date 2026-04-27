<#
[224A-1][264A-1] Limpieza inteligente del target dir de cargo (C:\tmp\glory-target).

Problema: el target dir crece sin lí­mite (incremental cache + deps + build scripts).
Llegó a 11+ GB en este proyecto.

264A-1: la limpieza "safe" original no tocaba incremental/, así que `npm run dev`
podía seguir acumulando gigas durante compilaciones sucesivas. Ahora el modo por
defecto aplica poda automática de incremental cuando el target supera un tope.

Modos:
    (default)   Safe: borra examples/, doctests/, y artifacts > N dí­as via cargo-sweep.
                            Si el target supera MaxTotalMB, también poda incremental/.
    -Hard       Borra incremental/ completo (cargo lo regenera; siguiente build paga).
                            No mata glory-backend; sirve para autolimpieza mientras dev sigue vivo.
    -Aggressive Borra incremental/ + deps/ + build/ + .fingerprint/ (recompila todo).
                            Este sí mata glory-backend para evitar locks.
    -DryRun     Solo reporta tamaños.

Uso:
    pwsh scripts/clean-cargo-target.ps1
    pwsh scripts/clean-cargo-target.ps1 -Hard
    pwsh scripts/clean-cargo-target.ps1 -Aggressive
    pwsh scripts/clean-cargo-target.ps1 -DryRun
    pwsh scripts/clean-cargo-target.ps1 -MaxTotalMB 4096

npm scripts (ver package.json):
    npm run clean:cargo        (safe con poda automática por tamaño)
    npm run clean:cargo:hard   (-Aggressive)

Hook automático:
    `predev` poda antes de arrancar si el target ya está pasado de tamaño.
    `postdev` corre la versión safe al terminar `npm run dev`.
#>

param(
    [switch]$Hard,
    [switch]$Aggressive,
    [switch]$DryRun,
    [int]$SweepDays = 7,
    [int]$MaxTotalMB = 4096
)

$ErrorActionPreference = 'Continue'

$targetDir = 'C:\tmp\glory-target'
if (-not (Test-Path $targetDir)) {
    Write-Host "[clean-cargo] $targetDir no existe. Nada que hacer."
    exit 0
}

function Get-DirSizeMB($path) {
    if (-not (Test-Path $path)) { return 0 }
    $sum = (Get-ChildItem $path -Recurse -ErrorAction SilentlyContinue |
            Measure-Object -Property Length -Sum).Sum
    return [math]::Round(($sum / 1MB), 0)
}

function Report($label) {
    $debugMB   = Get-DirSizeMB "$targetDir\debug"
    $releaseMB = Get-DirSizeMB "$targetDir\release"
    $totalMB   = Get-DirSizeMB $targetDir
    Write-Host ("[clean-cargo] {0,-10} debug={1,6} MB  release={2,6} MB  total={3,6} MB" -f $label, $debugMB, $releaseMB, $totalMB)
}

Report 'BEFORE'
$initialTotalMB = Get-DirSizeMB $targetDir

# Modo safe: solo paths que se regeneran sin coste o ya no son necesarios.
$pathsSafe = @(
    "$targetDir\debug\examples",
    "$targetDir\release\examples",
    "$targetDir\debug\doc",
    "$targetDir\release\doc"
)

$pathsHard = @(
    "$targetDir\debug\incremental",
    "$targetDir\release\incremental"
)

$pathsAggressive = @(
    "$targetDir\debug\deps",
    "$targetDir\debug\build",
    "$targetDir\debug\.fingerprint",
    "$targetDir\release\deps",
    "$targetDir\release\build",
    "$targetDir\release\.fingerprint"
)

$toRemove = $pathsSafe
if ((-not $Hard) -and (-not $Aggressive) -and $initialTotalMB -gt $MaxTotalMB) {
    Write-Host "[clean-cargo] Target excede tope ($initialTotalMB MB > $MaxTotalMB MB). Poda automatica de incremental/."
    $toRemove += $pathsHard
}
if ($Hard -or $Aggressive) { $toRemove += $pathsHard }
if ($Aggressive)           { $toRemove += $pathsAggressive }

# Si vamos a tocar deps/ o build outputs pesados, matar glory-backend para liberar locks.
# [264A-1] -Hard solo toca incremental/, así que debe ser usable con el backend vivo.
$needsKill = $Aggressive
if ($needsKill) {
    Get-Process -Name 'glory-backend' -ErrorAction SilentlyContinue | ForEach-Object {
        if ($DryRun) {
            Write-Host "[clean-cargo] DryRun: matarí­a glory-backend (PID $($_.Id))"
        } else {
            Write-Host "[clean-cargo] Matando glory-backend (PID $($_.Id))..."
            Stop-Process -Id $_.Id -Force -ErrorAction SilentlyContinue
        }
    }
    Start-Sleep -Milliseconds 300
}

foreach ($p in $toRemove) {
    if (-not (Test-Path $p)) { continue }
    $mb = Get-DirSizeMB $p
    if ($DryRun) {
        Write-Host "[clean-cargo] DryRun: eliminarí­a $p ($mb MB)"
    } else {
        Write-Host "[clean-cargo] Eliminando $p ($mb MB)..."
        Remove-Item $p -Recurse -Force -ErrorAction SilentlyContinue
    }
}

# cargo sweep: borra artifacts no usados por builds recientes (>N dí­as).
# Es la forma "fina" de limpiar deps/ sin perder incremental usable.
if (-not $DryRun) {
    $sweepCmd = Get-Command 'cargo-sweep' -ErrorAction SilentlyContinue
    if ($sweepCmd) {
        Write-Host "[clean-cargo] cargo sweep --time $SweepDays (artifacts no tocados en >$SweepDays dí­as)..."
        Push-Location (Resolve-Path "$PSScriptRoot\..")
        try {
            & cargo sweep --time $SweepDays 2>&1 | Out-Host
        } catch {
            Write-Warning "[clean-cargo] cargo sweep fallí³: $_"
        } finally {
            Pop-Location
        }
    } else {
        Write-Host "[clean-cargo] (info) Para limpieza fina por edad: cargo install cargo-sweep"
    }
}

Report 'AFTER'
Write-Host '[clean-cargo] OK.'

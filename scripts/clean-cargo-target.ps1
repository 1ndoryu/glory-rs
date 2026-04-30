<#
[224A-1][264A-1][294A-2][304A-2] Limpieza inteligente de los target dirs de cargo (C:\tmp\glory*-target).

Problema: el target dir crece sin lí­mite (incremental cache + deps + build scripts).
Llegó a 11+ GB en este proyecto.

264A-1: la limpieza "safe" original no tocaba incremental/, así que `npm run dev`
podía seguir acumulando gigas durante compilaciones sucesivas. Ahora el modo por
defecto aplica poda automática de incremental cuando el target supera un tope.

294A-2: el tope ahora se cumple de verdad. Si borrar incremental/ no baja el
target por debajo de MaxTotalMB, el script escala a caches regenerables pesados
como deps/, build/ y .fingerprint/ hasta quedar bajo el limite.

304A-2: `cargo run --target-dir C:\tmp\glory-openapi-target -- --emit-openapi ...`
evita locks al exportar OpenAPI, pero creó un segundo target dir que no entraba
en la limpieza automática. Ahora el script descubre y limpia TODOS los target dirs
`C:\tmp\glory*-target`, no solo el principal.

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
    [int]$MaxTotalMB = 4096,
    [string[]]$TargetDirs
)

$ErrorActionPreference = 'Continue'

function Get-ManagedTargetDirs {
    $defaults = @(
        'C:\tmp\glory-target',
        'C:\tmp\glory-openapi-target'
    )

    $discovered = @()
    if (Test-Path 'C:\tmp') {
        $discovered = Get-ChildItem 'C:\tmp' -Directory -ErrorAction SilentlyContinue |
            Where-Object { $_.Name -like 'glory*-target' } |
            Select-Object -ExpandProperty FullName
    }

    return @($TargetDirs + $defaults + $discovered) |
        Where-Object { -not [string]::IsNullOrWhiteSpace($_) } |
        ForEach-Object { $_.TrimEnd('\') } |
        Select-Object -Unique
}

function Get-DirSizeMB($path) {
    if (-not (Test-Path $path)) { return 0 }
    $sum = (Get-ChildItem $path -Recurse -Force -ErrorAction SilentlyContinue |
            Measure-Object -Property Length -Sum).Sum
    return [math]::Round(($sum / 1MB), 0)
}

function Test-CompileRunning {
    $processNames = @('cargo', 'rustc')
    foreach ($name in $processNames) {
        if (Get-Process -Name $name -ErrorAction SilentlyContinue) {
            return $true
        }
    }

    return $false
}

function Remove-TargetPath($path, $reason) {
    if (-not (Test-Path $path)) { return }
    $mb = Get-DirSizeMB $path
    if ($DryRun) {
        Write-Host "[clean-cargo] DryRun: eliminaria $path ($mb MB) [$reason]"
    } else {
        Write-Host "[clean-cargo] Eliminando $path ($mb MB) [$reason]..."
        Remove-Item $path -Recurse -Force -ErrorAction SilentlyContinue
    }
}

function Report($targetDir, $label) {
    $debugMB   = Get-DirSizeMB "$targetDir\debug"
    $releaseMB = Get-DirSizeMB "$targetDir\release"
    $totalMB   = Get-DirSizeMB $targetDir
    Write-Host ("[clean-cargo] {0} {1,-10} debug={2,6} MB  release={3,6} MB  total={4,6} MB" -f $targetDir, $label, $debugMB, $releaseMB, $totalMB)
}

function Invoke-CleanTargetDir($targetDir) {
    if (-not (Test-Path $targetDir)) {
        Write-Host "[clean-cargo] $targetDir no existe. Nada que hacer."
        return
    }

    Report $targetDir 'BEFORE'
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
        Write-Host "[clean-cargo] $targetDir excede tope ($initialTotalMB MB > $MaxTotalMB MB). Poda automatica de incremental/."
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
        Remove-TargetPath $p 'rutina inicial'
    }

    # cargo sweep: borra artifacts no usados por builds recientes (>N dí­as).
    # Es la forma "fina" de limpiar deps/ sin perder incremental usable.
    if ((-not $DryRun) -and ((Get-DirSizeMB $targetDir) -gt $MaxTotalMB)) {
        $sweepCmd = Get-Command 'cargo-sweep' -ErrorAction SilentlyContinue
        if ($sweepCmd) {
            Write-Host "[clean-cargo] cargo sweep --time $SweepDays sobre $targetDir (artifacts no tocados en >$SweepDays dí­as)..."
            Push-Location (Resolve-Path "$PSScriptRoot\..")
            $previousTargetDir = $env:CARGO_TARGET_DIR
            $env:CARGO_TARGET_DIR = $targetDir
            try {
                & cargo sweep --time $SweepDays 2>&1 | Out-Host
                if ($LASTEXITCODE -ne 0) {
                    Write-Warning "[clean-cargo] cargo sweep devolvio exit code $LASTEXITCODE; se continuara con limpieza por tamano."
                }
            } catch {
                Write-Warning "[clean-cargo] cargo sweep fallí³: $_"
            } finally {
                if ([string]::IsNullOrEmpty($previousTargetDir)) {
                    Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue
                } else {
                    $env:CARGO_TARGET_DIR = $previousTargetDir
                }
                Pop-Location
            }
        } else {
            Write-Host "[clean-cargo] (info) Para limpieza fina por edad: cargo install cargo-sweep"
        }
    } elseif (-not $DryRun) {
        Write-Host "[clean-cargo] $targetDir bajo el tope; salto cargo sweep."
    }

    $afterRoutineTotalMB = Get-DirSizeMB $targetDir
    if ($afterRoutineTotalMB -gt $MaxTotalMB) {
        if (Test-CompileRunning) {
            Write-Warning "[clean-cargo] $targetDir sigue sobre el tope ($afterRoutineTotalMB MB > $MaxTotalMB MB), pero cargo/rustc esta activo. Se evita tocar deps/build durante compilacion."
        } else {
            Write-Host "[clean-cargo] $targetDir sigue sobre el tope ($afterRoutineTotalMB MB > $MaxTotalMB MB). Escalando a caches regenerables."
            $capEscalationPaths = @(
                "$targetDir\debug\deps",
                "$targetDir\debug\build",
                "$targetDir\debug\.fingerprint",
                "$targetDir\release\deps",
                "$targetDir\release\build",
                "$targetDir\release\.fingerprint"
            )

            foreach ($p in $capEscalationPaths) {
                if ((Get-DirSizeMB $targetDir) -le $MaxTotalMB) { break }
                Remove-TargetPath $p 'cumplir MaxTotalMB'
            }
        }
    }

    Report $targetDir 'AFTER'
    $finalTotalMB = Get-DirSizeMB $targetDir
    if ($finalTotalMB -gt $MaxTotalMB) {
        Write-Warning "[clean-cargo] $targetDir aun supera el tope ($finalTotalMB MB > $MaxTotalMB MB). Puede haber archivos bloqueados por un proceso activo."
    } else {
        Write-Host "[clean-cargo] $targetDir OK."
    }
}

$managedTargetDirs = Get-ManagedTargetDirs
$existingTargetDirs = $managedTargetDirs | Where-Object { Test-Path $_ }

if (-not $existingTargetDirs) {
    Write-Host '[clean-cargo] No hay target dirs gestionados. Nada que hacer.'
    exit 0
}

foreach ($dir in $existingTargetDirs) {
    Invoke-CleanTargetDir $dir
}

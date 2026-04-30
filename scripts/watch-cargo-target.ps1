<#
[264A-1][304A-2] Vigilante liviano de los target dirs de Cargo para sesiones largas de `npm run dev`.

Problema: `postdev` solo limpia al cerrar la sesión. Si el backend recompila muchas veces,
el target puede seguir creciendo durante horas.

Estrategia:
- Esperar un margen inicial para no tocar el target durante el arranque de `cargo run`.
- Revisar el tamaño total del target cada cierto intervalo.
- Si supera MaxTotalMB y no hay procesos cargo/rustc activos, ejecutar limpieza -Hard.
- -Hard solo elimina incremental/, así que no mata glory-backend ni tumba el dev server.

[264A-2] Fix de carrera: la primera versión corría la poda inmediatamente al arrancar
`npm run dev`. Si `cargo` ya había tomado el target pero todavía no aparecía `rustc`,
la limpieza podía pelearse con `.fingerprint/` y romper la compilación.

[304A-2] Ya no observa solo `C:\tmp\glory-target`: ahora descubre también target dirs
auxiliares como `C:\tmp\glory-openapi-target`, que antes crecían sin ningún límite.
#>

param(
    [int]$IntervalSeconds = 300,
    [int]$StartupDelaySeconds = 120,
    [int]$MaxTotalMB = 4096,
    [switch]$RunOnce,
    [switch]$DryRun,
    [string[]]$TargetDirs
)

$ErrorActionPreference = 'Continue'
$cleanScript = Join-Path $PSScriptRoot 'clean-cargo-target.ps1'

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

function Invoke-PruneIfNeeded {
    $managedTargetDirs = Get-ManagedTargetDirs | Where-Object { Test-Path $_ }
    if (-not $managedTargetDirs) {
        Write-Host '[watch-cargo] No hay target dirs gestionados activos.'
        return
    }

    foreach ($targetDir in $managedTargetDirs) {
        $totalMB = Get-DirSizeMB $targetDir
        Write-Host "[watch-cargo] $targetDir = $totalMB MB (cap=$MaxTotalMB MB)"

        if ($totalMB -le $MaxTotalMB) {
            continue
        }

        if (Test-CompileRunning) {
            Write-Host '[watch-cargo] cargo/rustc activo; salto esta pasada para no pelear con la compilacion.'
            return
        }

        $args = @('-ExecutionPolicy', 'Bypass', '-File', $cleanScript, '-Hard', '-MaxTotalMB', $MaxTotalMB, '-TargetDirs', $targetDir)
        if ($DryRun) {
            $args += '-DryRun'
        }

        Write-Host "[watch-cargo] cap superado en $targetDir; ejecutando limpieza -Hard de incremental/."
        & powershell @args
    }
}

if (-not $RunOnce) {
    Write-Host "[watch-cargo] Esperando $StartupDelaySeconds s antes de la primera pasada."
    Start-Sleep -Seconds $StartupDelaySeconds
}

do {
    Invoke-PruneIfNeeded
    if (-not $RunOnce) {
        Start-Sleep -Seconds $IntervalSeconds
    }
} while (-not $RunOnce)

# [253A-12] Script de auto-verificacion de cumplimiento del protocolo v5.0.
# [253A-15] Anadida regla 18 (proactividad).
# Se ejecuta como ultimo paso antes de cerrar una tarea.
# Imprime un checklist de todas las reglas y el agente debe confirmar cada una.
# Las reglas marcadas [CONDICIONAL] solo aplican si la tarea toco ese dominio.

param(
    [string]$TareaId = ""
)

$reglas = @(
    @{ id = "-1"; nombre = "LO MAS DIFICIL PRIMERO"; pregunta = "Se abordo primero la tarea mas compleja?"; cond = $false }
    @{ id = "0"; nombre = "Flujo obligatorio"; pregunta = "Se anuncio el flujo al inicio con formato exacto?"; cond = $false }
    @{ id = "1"; nombre = "Autonomia total"; pregunta = "Sin interrupciones ni confirmaciones triviales?"; cond = $false }
    @{ id = "2"; nombre = "Cero parches"; pregunta = "La solucion escala 10x sin reescritura?"; cond = $false }
    @{ id = "3"; nombre = "Ediciones controladas"; pregunta = "Archivo por archivo, validando entre cada uno?"; cond = $false }
    @{ id = "4"; nombre = "Guardian del orden"; pregunta = "Violaciones visibles corregidas (imports, hardcodeo)?"; cond = $false }
    @{ id = "5"; nombre = "Seguridad"; pregunta = "SQL parametrizado, secrets en env, input validado?"; cond = $true }
    @{ id = "6"; nombre = "Sin fallos silenciosos"; pregunta = "I/O con manejo errores? Sin unwrap en prod?"; cond = $true }
    @{ id = "7"; nombre = "Rendimiento"; pregunta = "Sin N+1? Zustand selectores especificos?"; cond = $true }
    @{ id = "8"; nombre = "Arquitectura SOLID"; pregunta = "Max 3 useState? Limites lineas? SRP?"; cond = $true }
    @{ id = "9"; nombre = "Estandares codigo"; pregunta = "camelCase, PascalCase, CSS espanol, sin inline?"; cond = $true }
    @{ id = "10"; nombre = "Comentarios"; pregunta = "Comentario en codigo + lecciones-aprendidas.md?"; cond = $false }
    @{ id = "11"; nombre = "Validacion obligatoria"; pregunta = "get_errors + type-check/cargo ejecutados?"; cond = $false }
    @{ id = "12"; nombre = "Commits"; pregunta = "git add explicito? diff verificado? Mensaje correcto?"; cond = $false }
    @{ id = "14"; nombre = "Glory Sentinel"; pregunta = "sentinel-disable con justificacion valida?"; cond = $true }
    @{ id = "15"; nombre = "Responsive"; pregunta = "Mobile 320px, tablet 768px, desktop 1024px?"; cond = $true }
    @{ id = "16"; nombre = "Revision roadmap"; pregunta = "Roadmap releido DESPUES de commit y resumen?"; cond = $false }
    @{ id = "17"; nombre = "Glory Framework"; pregunta = "Funcionalidad agnostica evaluada para submodulo?"; cond = $false }
    @{ id = "18"; nombre = "Proactividad"; pregunta = "Mejora del proceso detectada e implementada? self-check.ps1 existe?"; cond = $false }
)

$pasos = @(
    @{ n = "5"; nombre = "Archivar completados/"; pregunta = "En Agente/completados/tareas-YYYY-MM-DD.md?" }
    @{ n = "6"; nombre = "Documentar"; pregunta = "Agente/documentacion/ creada/actualizada?" }
    @{ n = "7"; nombre = "Prevencion"; pregunta = "Prevencion con Code Sentinel evaluada?" }
    @{ n = "9"; nombre = "Commit y push"; pregunta = "Commit con mensaje correcto? Push hecho?" }
    @{ n = "10"; nombre = "Releer roadmap"; pregunta = "Roadmap releido como ultima accion?" }
)

$sep = "============================================"

Write-Host ""
Write-Host $sep -ForegroundColor Cyan
Write-Host "  SELF-CHECK - Protocolo v5.0" -ForegroundColor Cyan
if ($TareaId) {
    Write-Host "  Tarea: $TareaId" -ForegroundColor Yellow
}
Write-Host $sep -ForegroundColor Cyan
Write-Host ""

Write-Host "--- REGLAS ABSOLUTAS ---" -ForegroundColor Green
foreach ($r in $reglas) {
    if ($r.cond) {
        $tag = "[CONDICIONAL]"
        $color = "DarkYellow"
    } else {
        $tag = "[OBLIGATORIA]"
        $color = "White"
    }
    Write-Host "  [$($r.id)] $($r.nombre) - $tag" -ForegroundColor $color
    Write-Host "        $($r.pregunta)" -ForegroundColor Gray
}

Write-Host ""
Write-Host "--- PASOS DEL FLUJO ---" -ForegroundColor Green
foreach ($p in $pasos) {
    Write-Host "  [Paso $($p.n)] $($p.nombre)" -ForegroundColor White
    Write-Host "        $($p.pregunta)" -ForegroundColor Gray
}

Write-Host ""
Write-Host $sep -ForegroundColor Cyan
Write-Host "  El agente debe confirmar CADA punto." -ForegroundColor Yellow
Write-Host "  [CONDICIONAL] se omite si la tarea no" -ForegroundColor Yellow
Write-Host "  toco ese dominio (backend/frontend/css)." -ForegroundColor Yellow
Write-Host $sep -ForegroundColor Cyan
Write-Host ""

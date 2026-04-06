# Falso positivo: límite de líneas en archivos de datos

## Problema
Code Sentinel reporta "Archivo excede limite de 300 lineas para componente" en `frontend/src/data/planes/planes.ts` (410 líneas efectivas). Este archivo es un archivo de datos estáticos puros (arrays de objetos, sin JSX, sin lógica, sin hooks) — la regla de 300 líneas está pensada para componentes y hooks, no para data files.

## Regla afectada
`limite-lineas` en Code Sentinel (staticAnalyzer)

## Corrección necesaria
Excluir archivos bajo `**/data/**` o detectar si el archivo exporta solo constantes/arrays (sin JSX, sin funciones con lógica) para no aplicar el límite de 300 líneas. Alternativa: subir el límite para archivos .ts que no contengan JSX.

## Fecha
2026-04-06

# Prevención: Detectar generated.ts monolítico

## Problema
Orval puede generar un archivo `generated.ts` monolítico si no se configura `mode: 'tags-split'`.
Esto produce un archivo enorme e imposible de mantener.

## Regla propuesta para Code Sentinel
- **Archivo objetivo:** `frontend/src/api/generated.ts`
- **Condición:** Si el archivo tiene más de 100 líneas y contiene funciones generadas (no solo re-exports), marcar como error.
- **Mensaje:** "El archivo generated.ts parece monolítico. Configurar Orval con `mode: 'tags-split'` en `orval.config.ts`."
- **Severidad:** Error

## Estado
Regla documentada en instrucciones (Regla 9: Estándares de código). Implementación en Code Sentinel pendiente.

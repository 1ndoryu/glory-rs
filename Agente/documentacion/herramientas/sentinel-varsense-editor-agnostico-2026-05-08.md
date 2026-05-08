# Sentinel + VarSense editor-agnosticos

## Estado 2026-05-08

El bloque `085A-1` avanzo la separacion core/adaptador en las herramientas `.agent/code-sentinel` y `.agent/varsense`.

## Sentinel

- `src/core/analyzeDocument.ts` es la entrada core para analizar un `CoreTextDocument` sin abrir VS Code.
- `src/core/violacionAdapter.ts` convierte el formato legacy `Violacion` a `CoreFinding` serializable.
- El provider VS Code crea `CoreTextDocument` con `documentFromVsCode` y convierte hallazgos con `findingToDiagnostic`.
- Las reglas Glory/API siguen inyectadas desde VS Code como `extraAnalyzers` porque aun dependen de workspace/watchers.
- `ruleRegistry` ya no importa `vscode`; recibe overrides por proveedor inyectado.

## VarSense

- Los tipos compartidos usan `CoreRange` y severidades numericas compatibles con VS Code.
- `cssParser` y `valueParser` operan sobre `CoreTextDocument` y reciben opciones inyectadas en vez de importar `configService`.
- Providers y scanner convierten documentos/rangos en el borde VS Code.

## Validacion

- Sentinel: `npm run compile`, `npm run test:unit`.
- VarSense: `npm test`.

## Pendientes

- Sentinel: extraer `core/report.ts` y desacoplar Glory/API/watchers internos.
- VarSense: extraer `VariableIndexBuilder` e interfaces `DocumentProvider`, `WorkspaceFileProvider` y `FileWatcher`.
- Tooling: `npm run lint` en Sentinel falla porque ESLint no esta declarado/configurado.

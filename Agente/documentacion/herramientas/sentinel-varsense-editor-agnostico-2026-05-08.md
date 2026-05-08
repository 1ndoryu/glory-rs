# Sentinel + VarSense editor-agnosticos

## Estado 2026-05-08

El bloque `085A-1` avanzo la separacion core/adaptador en las herramientas `.agent/code-sentinel` y `.agent/varsense`.

El bloque `085A-2` continuo esa separacion con reportes core en Sentinel y builders core de indices en VarSense.

## Sentinel

- `src/core/analyzeDocument.ts` es la entrada core para analizar un `CoreTextDocument` sin abrir VS Code.
- `src/core/violacionAdapter.ts` convierte el formato legacy `Violacion` a `CoreFinding` serializable.
- `src/core/report.ts` genera Markdown desde `CoreFinding` sin filesystem ni objetos VS Code.
- El provider VS Code crea `CoreTextDocument` con `documentFromVsCode` y convierte hallazgos con `findingToDiagnostic`.
- `providers/reportGenerator.ts` convierte `vscode.Diagnostic` a core con `diagnosticToFinding` y se limita a escribir/abrir el reporte.
- Las reglas Glory/API siguen inyectadas desde VS Code como `extraAnalyzers` porque aun dependen de workspace/watchers.
- `ruleRegistry` ya no importa `vscode`; recibe overrides por proveedor inyectado.

## VarSense

- Los tipos compartidos usan `CoreRange` y severidades numericas compatibles con VS Code.
- `cssParser` y `valueParser` operan sobre `CoreTextDocument` y reciben opciones inyectadas en vez de importar `configService`.
- `core/workspaceProviders.ts` define `DocumentProvider`, `WorkspaceFileProvider` y `FileWatcherProvider`.
- `core/variableIndexBuilder.ts` construye indices de variables desde providers core.
- `core/classIndexBuilder.ts` cruza clases definidas y tokens de consumo desde providers core.
- Providers y scanner convierten documentos/rangos en el borde VS Code.
- `services/classScanner.ts` quedo como adaptador fino sobre `ClassIndexBuilder`.

## Validacion

- Sentinel: `npm run compile`, `npm run test:unit` (`284 passing`, `1 pending` en `085A-2`).
- VarSense: `npm test` (`29 passing` en `085A-2`).

## Pendientes

- Sentinel: desacoplar Glory/API/watchers internos y mover `vscodeAdapter.ts` a `platform/vscode` cuando la estructura se consolide.
- VarSense: extraer watchers reales a `FileWatcherProvider`, mantener singleton solo en adaptador VS Code y preparar providers Node para CLI.
- Tooling: `npm run lint` en Sentinel falla porque ESLint no esta declarado/configurado.

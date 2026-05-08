# Sentinel + VarSense editor-agnosticos

## Estado 2026-05-08

El bloque `085A-1` avanzo la separacion core/adaptador en las herramientas `.agent/code-sentinel` y `.agent/varsense`.

El bloque `085A-2` continuo esa separacion con reportes core en Sentinel y builders core de indices en VarSense.

El bloque `085A-3` agrego la primera CLI real de Sentinel sobre el core.

## Sentinel

- `src/core/analyzeDocument.ts` es la entrada core para analizar un `CoreTextDocument` sin abrir VS Code.
- `src/core/violacionAdapter.ts` convierte el formato legacy `Violacion` a `CoreFinding` serializable.
- `src/core/report.ts` genera Markdown desde `CoreFinding` sin filesystem ni objetos VS Code.
- El provider VS Code crea `CoreTextDocument` con `documentFromVsCode` y convierte hallazgos con `findingToDiagnostic`.
- `providers/reportGenerator.ts` convierte `vscode.Diagnostic` a core con `diagnosticToFinding` y se limita a escribir/abrir el reporte.
- `src/cli/index.ts` expone `sentinel analyze` para reportes Markdown/JSON sin abrir VS Code.
- Las reglas Glory/API siguen inyectadas desde VS Code como `extraAnalyzers` porque aun dependen de workspace/watchers.
- `ruleRegistry` ya no importa `vscode`; recibe overrides por proveedor inyectado.
- `analyzers/glory/apiFallbackRules.ts` contiene la regla pura `acceso-api-sin-fallback`, separada de `apiContractRules.ts` para que la CLI no cargue el indexador `vscode` indirectamente.

### CLI Sentinel

Comandos soportados:

```powershell
sentinel analyze --workspace . --format markdown --output .sentinel-report.md
sentinel analyze --file src/app.ts --format json
```

La CLI busca `sentinel.config.json` en el workspace si no se pasa `--config`. El archivo puede definir `includePatterns`, `excludePatterns`, `directoryExceptions` y `rules` con el formato de overrides usado por Sentinel.

Codigos de salida:

- `0`: analisis limpio o solo hallazgos no error.
- `1`: existe al menos un hallazgo con severidad `error`.
- `2`: fallo de ejecucion o configuracion.

## VarSense

- Los tipos compartidos usan `CoreRange` y severidades numericas compatibles con VS Code.
- `cssParser` y `valueParser` operan sobre `CoreTextDocument` y reciben opciones inyectadas en vez de importar `configService`.
- `core/workspaceProviders.ts` define `DocumentProvider`, `WorkspaceFileProvider` y `FileWatcherProvider`.
- `core/variableIndexBuilder.ts` construye indices de variables desde providers core.
- `core/classIndexBuilder.ts` cruza clases definidas y tokens de consumo desde providers core.
- Providers y scanner convierten documentos/rangos en el borde VS Code.
- `services/classScanner.ts` quedo como adaptador fino sobre `ClassIndexBuilder`.

## Validacion

- Sentinel: `npm run compile`, `npm run test:unit` (`287 passing`, `1 pending` en `085A-3`). Smoke CLI JSON y Markdown pasaron con fixtures temporales y exit `1` esperado por `hardcoded-secret`.
- VarSense: `npm test` (`29 passing` en `085A-2`).

## Pendientes

- Sentinel: desacoplar Glory/API/watchers internos y mover `vscodeAdapter.ts` a `platform/vscode` cuando la estructura se consolide.
- VarSense: extraer watchers reales a `FileWatcherProvider`, mantener singleton solo en adaptador VS Code y preparar providers Node para CLI.
- Tooling: `npm run lint` en Sentinel falla porque ESLint no esta declarado/configurado.
- Equivalencia: agregar fixtures que comparen CLI y core antes de iniciar LSP.

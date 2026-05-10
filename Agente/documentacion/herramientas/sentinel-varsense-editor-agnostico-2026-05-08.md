# Sentinel + VarSense editor-agnosticos

## Estado 2026-05-10

El bloque `085A-1` avanzo la separacion core/adaptador en las herramientas `.agent/code-sentinel` y `.agent/varsense`.

El bloque `085A-2` continuo esa separacion con reportes core en Sentinel y builders core de indices en VarSense.

El bloque `085A-3` agrego la primera CLI real de Sentinel sobre el core.

El bloque `105A-1` agrego LSP y Zed a Sentinel, dejo lint operativo y valido el estado completo de VarSense.

## Sentinel

- `src/core/analyzeDocument.ts` es la entrada core para analizar un `CoreTextDocument` sin abrir VS Code.
- `src/core/violacionAdapter.ts` convierte el formato legacy `Violacion` a `CoreFinding` serializable.
- `src/core/report.ts` genera Markdown desde `CoreFinding` sin filesystem ni objetos VS Code.
- El provider VS Code crea `CoreTextDocument` con `documentFromVsCode` y convierte hallazgos con `findingToDiagnostic`.
- `providers/reportGenerator.ts` convierte `vscode.Diagnostic` a core con `diagnosticToFinding` y se limita a escribir/abrir el reporte.
- `src/cli/index.ts` expone `sentinel analyze` para reportes Markdown/JSON sin abrir VS Code.
- `src/lsp/server.ts` expone `sentinel-lsp` para diagnostics nativos en clientes LSP.
- `src/core/config.ts` centraliza defaults y overrides compartidos por CLI y LSP.
- `integrations/zed/` registra una extension Zed minima que solo localiza y lanza `sentinel-lsp`.
- `.zed/tasks.json` contiene tareas de ejemplo para reportes CLI desde Zed.
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

### LSP y Zed Sentinel

Comandos soportados:

```powershell
sentinel-lsp --stdio
npm run lsp
npm run smoke:lsp
npm run check:zed
```

Resolucion en Zed:

- `SENTINEL_LSP_PATH` si se quiere apuntar a un script/binario concreto.
- `sentinel-lsp` disponible en PATH.
- `../../out/lsp/server.js` para desarrollo dentro del repo.

## VarSense

- Los tipos compartidos usan `CoreRange` y severidades numericas compatibles con VS Code.
- `cssParser` y `valueParser` operan sobre `CoreTextDocument` y reciben opciones inyectadas en vez de importar `configService`.
- `core/workspaceProviders.ts` define `DocumentProvider`, `WorkspaceFileProvider` y `FileWatcherProvider`.
- `core/variableIndexBuilder.ts` construye indices de variables desde providers core.
- `core/classIndexBuilder.ts` cruza clases definidas y tokens de consumo desde providers core.
- Providers y scanner convierten documentos/rangos en el borde VS Code.
- `services/classScanner.ts` quedo como adaptador fino sobre `ClassIndexBuilder`.
- `src/cli/index.ts`, `src/lsp/server.ts`, `scripts/smoke-lsp-stdio.mjs`, `.zed/tasks.json` e `integrations/zed/` ya existen y estan validados.

## Validacion

- Sentinel: `npm run lint` (0 errores, 14 warnings preexistentes), `npm run test:unit` (`294 passing`, `1 pending`, incluye `check:core` y `smoke:lsp`) y `npm run check:zed`.
- VarSense: `npm test` (`36 passing`, incluye compile, compile:tests, lint, `check:core`, `smoke:lsp`, equivalencia y pruebas VS Code).

## Pendientes

- Sentinel: desacoplar Glory/API/watchers internos (`gloryAnalyzer`, `apiEndpointAnalyzer`, schema/islas/contratos/tipos/constantes) y mover `vscodeAdapter.ts` a `platform/vscode` cuando la estructura se consolide.
- VarSense: extraer watchers reales a `FileWatcherProvider` y mantener singleton solo en adaptador VS Code.
- Equivalencia: ampliar fixtures Sentinel a familias PHP/Rust/React ademas de la fixture CSS actual.

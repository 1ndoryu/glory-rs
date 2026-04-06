# Falso positivo: sqlx::query() sin macro en código de test

## Problema detectado
Code Sentinel reporta `sqlx::query() detectado sin macro. Usar sqlx::query! para verificar SQL en compilación` en archivos de test (`tests/haddock_db.rs` y `src/services/haddock.rs` módulo test).

## Por qué es falso positivo
Los test helpers usan `sqlx::query()` deliberadamente porque:
1. Las queries de setup (INSERT INTO users para tests) no están en el cache offline (`.sqlx/`)
2. Con `SQLX_OFFLINE=true`, `sqlx::query!()` solo funciona para queries cacheadas
3. Agregar queries de test al cache offline es frágil y innecesario
4. El patrón `sqlx::query()` runtime en tests es práctica estándar en Rust

## Corrección necesaria en Code Sentinel
La regla `sqlx::query()` sin macro debería tener una excepción para:
- Archivos en directorio `tests/`
- Bloques `#[cfg(test)] mod tests { ... }`
- Funciones marcadas con `#[test]`, `#[tokio::test]`, `#[sqlx::test]`

## Implementación sugerida
En el analyzer que detecta `sqlx::query()`, agregar filtro:
```typescript
// No reportar si estamos dentro de un módulo de test o archivo de test
if (isTestContext(document, position)) {
    return; // Skip — uso legítimo en tests
}
```

# Normalizacion de publicaciones Rust -> legacy — 2026-04-27

## Contexto

El feed de comunidad empezo a renderizar publicaciones reales del backend Rust, pero la UI legacy seguia esperando el shape antiguo en camelCase.

## Sintoma visible

`TarjetaPublicacion` explotaba con `Cannot read properties of undefined (reading 'length')` al intentar acceder a `publicacion.samplesAdjuntos.length`.

## Causa raiz

La API Rust de publicaciones responde `PostDetail` en snake_case:

- `autor_id`
- `samples_adjuntos`
- `total_likes`
- `created_at`
- `moderacion_estado`
- `mi_reaccion`
- `yo_ya_repostee`
- `repost_original`

El frontend legacy renderiza `Publicacion` en camelCase:

- `autorId`
- `samplesAdjuntos`
- `totalLikes`
- `creadoAt`
- `moderacionEstado`
- `reaccion`
- `reposteado`
- `repostOriginal`

Al pasar el objeto raw directo desde red al componente, `samplesAdjuntos` quedaba `undefined` y el componente caía en runtime.

## Cambio aplicado

- Se agregó `frontend/src/legacy/services/normalizers/postNormalizer.ts`
- `frontend/src/legacy/services/apiSocial.ts` ahora normaliza:
  - creación de publicación
  - listado de publicaciones
  - publicaciones de perfil
  - detalle individual
- `frontend/src/legacy/hooks/useComunidadIsland.tsx` dejó de consumir `/publicaciones` crudo y ahora usa el servicio normalizado

## Resultado

La comunidad ya no depende del shape crudo del backend Rust. Los arrays críticos como `samplesAdjuntos` e `imagenes` siempre llegan definidos al componente.

## Gotchas

- Este error no lo detecta bien el IDE porque no era un error de tipos en el source, sino una discrepancia entre el contrato real del fetch y la interfaz asumida por el renderer.
- La primera corrección de `items` vs `data` destapó este segundo problema porque por fin el feed empezó a renderizar registros reales.

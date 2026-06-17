# Análisis: ModalInspectorSample — Sección "Origen y Sampleo"

> **Fecha:** 2026-06-16 (revisado)
> **Propósito:** Evaluar exclusivamente qué datos muestra actualmente la sección "Origen y Sampleo" del `ModalInspectorSample` (modal de inspección técnica de un sample), qué datos debería mostrar, y qué cambios en el pipeline (API Rust → frontend) se necesitan para que ocurra.
> **Alcance:** SOLO `ModalInspectorSample.tsx`. No cubre `TablaRelaciones`, `LadoCancionRelacion`, `CancionDetalleIsland`, ni otras vistas públicas.
> **Estado:** Pendiente de aprobación para implementar

---

## 1. Estado Actual del Modal

### 1.1 Qué muestra hoy "Origen y Sampleo"

```
Es Recorte: Si
Cancion Origen ID: 74
Relación Sampleo ID: 68
```

3 campos. Solo IDs numéricos crudos.

Además, hay dos secciones **que nunca se ven** porque los datos no llegan del API:

- **Cancion Origen** (título condicional) — se renderiza sólo si `cancionOrigen?.titulo` existe, pero el API nunca lo popular
- **SeccionExtraccionInspector** — se renderiza sólo si `extraccion` existe, pero el API nunca lo incluye

### 1.2 Flujo de datos actual

```
Rust handler (GET /api/samples/{slug})
  │  SQL query: SELECT s.cancion_origen_id, s.relacion_sampleo_id ← solo FKs
  │  ❌ No hace LEFT JOIN con canciones
  │  ❌ No hace LEFT JOIN con relaciones_sample
  │  ❌ No hace LEFT JOIN con cola_extraccion_samples
  │
  ▼
SampleDetailResponse (Rust struct)
  │  - cancion_origen_id: Option<i32>    ← número crudo
  │  - relacion_sampleo_id: Option<i32>  ← número crudo
  │  ❌ NO tiene cancion_origen (enriquecido)
  │  ❌ NO tiene extraccion
  │
  ▼
normalizarSampleDetalle() (frontend)
  │  - cancionOrigenId: number | null    ← mismo número
  │  - relacionSampleoId: number | null  ← mismo número
  │  - extraccion: siempre null          ← nunca llega
  │  - cancionOrigen: siempre null       ← nunca llega
  │
  ▼
ModalInspectorSample
  │  "Origen y Sampleo" muestra:
  │    Es Recorte: Si
  │    Cancion Origen ID: 74
  │    Relación Sampleo ID: 68
  │  SeccionExtraccionInspector: NO se renderiza (extraccion es null)
```

### 1.3 El frontend YA tiene los componentes para mostrarlo

**`SeccionExtraccionInspector.tsx`** ya renderiza TODO lo que se pide:

| Lo que renderiza                     | Campo `ExtraccionSample` que usa         |
| ------------------------------------ | ---------------------------------------- |
| Canción Fuente (Artista — Título)    | `sampleoFuenteArtista + sampleoFuenteTitulo` |
| Canción Destino (Artista — Título)   | `sampleoDestinoArtista + sampleoDestinoTitulo` |
| Link a canción fuente (slug)         | `fuenteSlug`                             |
| Link a canción destino (slug)        | `destinoSlug`                            |
| Álbum fuente/destino                 | `fuenteAlbum`, `destinoAlbum`            |
| Tipo Elemento                        | `tipoElemento`                           |
| YouTube link                         | `youtubeId`                              |
| Spotify link                         | `spotifyId`                              |
| URL de descarga                      | `fuenteUrl`                              |
| Título en fuente                     | `fuenteTitulo`, `fuenteArtista`          |
| BPM Detectado                        | `bpmDetectado`                           |
| Rango extraído (compás inicio/fin)   | `compasInicioSeg`, `compasFinSeg`        |
| Timing inicio                        | `timingInicioSeg`                        |
| Método descarga                      | `descargaMetodo`                         |
| Origen                               | `origen`                                 |

El componente **no necesita modificaciones**. El único problema es que `extraccion` nunca llega del API.

---

## 2. GAP Identificado: API no sirve datos enriquecidos

### 2.1 GAP ÚNICO (el resto ya existe)

| Capa           | Problema                                                                        |
| -------------- | ------------------------------------------------------------------------------- |
| **Rust (SQL)** | La query `find_sample_by_slug_or_short_id` no hace JOIN con `canciones`, `relaciones_sample` ni `cola_extraccion_samples` |
| **Rust (model)** | `SampleDetailResponse` no tiene campo `cancion_origen` ni `extraccion`         |
| **Rust (builder)** | `build_sample_detail()` no popula `cancion_origen` ni `extraccion`            |
| **Frontend (normalizer)** | `normalizarSampleDetalle()` no mapea `cancionOrigen` ni `extraccion` porque el API no los envía |
| **Frontend (modal)** | La sección "Origen y Sampleo" muestra solo IDs crudos; `SeccionExtraccionInspector` no se renderiza |

**No hay otros gaps.** El scraper, la BD, el extractor de audio y el frontend ya tienen todo lo necesario. El único cuello de botella es la API REST.

### 2.2 Datos existentes en BD que no llegan al modal

| Tabla                    | Columnas útiles                              | ¿Se usa hoy? |
| ------------------------ | -------------------------------------------- | ------------ |
| `canciones` (via `cancion_origen_id`) | `titulo`, `slug`, `artista_id`, `whosampled_url`, `bpm`, `youtube_id`, `spotify_id` | ❌ |
| `relaciones_sample` (via `relacion_sampleo_id`) | `whosampled_id`, `tipo_elemento`, `timings_fuente`, `timings_destino`, `votos_total` | ❌ |
| `cola_extraccion_samples` (via FK a sample) | `youtube_id`, `timing_inicio_seg`, `bpm_detectado`, `compas_inicio_seg`, `compas_fin_seg`, `lado`, `fuente_url`, `metadata_extraccion` (QQ23) | ❌ |
| `metadata_extraccion` (JSONB) | `sampleo_fuente_titulo`, `sampleo_fuente_artista`, `sampleo_destino_titulo`, `sampleo_destino_artista`, `fuente_slug`, `destino_slug`, `fuente_album`, `destino_album` | ❌ |

---

## 3. Solución Propuesta

### 3.1 Cambios en Rust

#### 3.1.1 SQL query — agregar LEFT JOINs

En `src/repositories/sample_catalog.rs`, función `find_sample_by_slug_or_short_id`:

```sql
SELECT
    -- ... campos existentes ...
    s.cancion_origen_id,
    s.relacion_sampleo_id,

    -- QQ51: Datos enriquecidos de cancion origen
    co.id AS "cancion_origen_id!",
    co.titulo AS "cancion_origen_titulo?",
    co.slug AS "cancion_origen_slug?",
    co.whosampled_url AS "cancion_origen_whosampled_url?",
    co.bpm AS "cancion_origen_bpm?",
    a.nombre AS "cancion_origen_artista?",

    -- QQ51: Datos de la relacion de sampleo
    rs.whosampled_id AS "relacion_sampleo_whosampled_id?",
    rs.tipo_elemento AS "relacion_sampleo_tipo?",

    -- QQ117: Extraccion (una fila, la mas reciente por sample_id)
    ces.youtube_id AS "extraccion_youtube_id?",
    ces.spotify_id AS "extraccion_spotify_id?",
    ces.timing_inicio_seg AS "extraccion_timing_inicio_seg?",
    ces.bpm_detectado AS "extraccion_bpm_detectado?",
    ces.compas_inicio_seg AS "extraccion_compas_inicio_seg?",
    ces.compas_fin_seg AS "extraccion_compas_fin_seg?",
    ces.lado AS "extraccion_lado?",
    ces.estado AS "extraccion_estado?",
    ces.ruta_audio_extraido AS "extraccion_ruta_audio?",
    ces.fuente_url AS "extraccion_fuente_url?",
    ces.fuente_titulo AS "extraccion_fuente_titulo?",
    ces.fuente_artista AS "extraccion_fuente_artista?",
    ces.descarga_metodo AS "extraccion_descarga_metodo?",
    ces.origen AS "extraccion_origen?",
    ces.lado_extraccion AS "extraccion_lado_extraccion?",
    ces.metadata_extraccion AS "extraccion_metadata?: serde_json::Value"

FROM samples s
INNER JOIN usuarios_ext u ON u.id = s.creador_id

-- QQ51: Join opcional para datos de cancion origen
LEFT JOIN canciones co ON co.id = s.cancion_origen_id
LEFT JOIN artistas_musicales a ON a.id = co.artista_id

-- QQ51: Join opcional para relacion de sampleo
LEFT JOIN relaciones_sample rs ON rs.id = s.relacion_sampleo_id

-- QQ117: Extraccion — subquery que trae la fila mas reciente de cola_extraccion_samples para este sample
LEFT JOIN LATERAL (
    SELECT * FROM cola_extraccion_samples
    WHERE sample_id = s.id
    ORDER BY creado_en DESC
    LIMIT 1
) ces ON TRUE
```

#### 3.1.2 Nuevos structs en `src/models/sample.rs`

```rust
/// QQ51: Datos enriquecidos de la canción origen de un sample
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CancionOrigenResumen {
    pub id: i32,
    pub titulo: String,
    pub slug: String,
    pub artista: Option<String>,
    pub whosampled_url: Option<String>,
    pub bpm: Option<i16>,
}

/// QQ117: Metadata de extracción para un sample (vía cola_extraccion_samples)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExtraccionSampleResponse {
    pub youtube_id: Option<String>,
    pub spotify_id: Option<String>,
    pub timing_inicio_seg: Option<i32>,
    pub bpm_detectado: Option<i32>,
    pub compas_inicio_seg: Option<i32>,
    pub compas_fin_seg: Option<i32>,
    pub lado: Option<String>,
    pub estado: Option<String>,
    pub ruta_audio_extraido: Option<String>,
    pub fuente_url: Option<String>,
    pub fuente_titulo: Option<String>,
    pub fuente_artista: Option<String>,
    pub descarga_metodo: Option<String>,
    pub origen: Option<String>,
    pub lado_extraccion: Option<String>,
    /// QQ23: Campos del JSONB metadata_extraccion (deserializados)
    pub sampleo_fuente_titulo: Option<String>,
    pub sampleo_fuente_artista: Option<String>,
    pub sampleo_destino_titulo: Option<String>,
    pub sampleo_destino_artista: Option<String>,
    pub fuente_slug: Option<String>,
    pub fuente_album: Option<String>,
    pub destino_slug: Option<String>,
    pub destino_album: Option<String>,
    pub votos_total: Option<i32>,
    pub tipo_elemento: Option<String>,
    pub duracion_extraida: Option<f64>,
    pub formato_extraido: Option<String>,
    pub tamano_bytes: Option<i64>,
}
```

#### 3.1.3 Campos nuevos en `SampleDetailResponse`

Agregar a `SampleDetailResponse`:

```rust
    /// QQ51: Datos enriquecidos de la canción origen (si es recorte)
    pub cancion_origen: Option<CancionOrigenResumen>,

    /// QQ117: Metadata de extracción (si existe)
    pub extraccion: Option<ExtraccionSampleResponse>,

    /// QQ51: URL de WhoSampled construida desde relacion_sampleo.whosampled_id
    pub whosampled_url: Option<String>,
```

#### 3.1.4 `SampleCatalogDetailRecord` — nuevos campos

Agregar campos para recibir las columnas de los LEFT JOINs. Se mapean como `Option<T>` porque los JOINs son opcionales.

#### 3.1.5 `build_sample_detail()` — popular los nuevos campos

```rust
fn build_sample_detail(...) -> SampleDetailResponse {
    // ... campos existentes ...

    // QQ51: cancion_origen enriquecido
    cancion_origen: record.cancion_origen_id.map(|id| {
        CancionOrigenResumen {
            id,
            titulo: record.cancion_origen_titulo.clone().unwrap_or_default(),
            slug: record.cancion_origen_slug.clone().unwrap_or_default(),
            artista: record.cancion_origen_artista.clone(),
            whosampled_url: record.cancion_origen_whosampled_url.clone(),
            bpm: record.cancion_origen_bpm,
        }
    }),

    // QQ117: extraccion (si existe al menos un campo clave)
    extraccion: record.extraccion_youtube_id.as_ref().or(record.extraccion_bpm_detectado.as_ref()).map(|_| {
        let metadata = record.extraccion_metadata.as_ref()
            .and_then(|v| v.as_object())
            .map(|obj| {
                // Extraer QQ23 del JSONB
                ExtraccionSampleResponse { ... }
            })
            .unwrap_or_default();
        // ... populate from flat fields + metadata
    }),

    whosampled_url: record.relacion_sampleo_whosampled_id
        .map(|id| format!("https://www.whosampled.com/sample/{}/", id)),
}
```

### 3.2 Cambios en Frontend

#### 3.2.1 Actualizar `Sample` type en `frontend/src/legacy/types/sample.ts`

```typescript
export interface Sample {
    // ... campos existentes ...

    /* QQ51: Datos enriquecidos de la cancion de origen */
    cancionOrigenId?: number | null;
    cancionOrigen?: CancionOrigenResumen | null;

    /* QQ117: Metadata de extraccion */
    extraccion?: ExtraccionSample | null;

    /* URL de WhoSampled construida */
    whosampledUrl?: string | null;
}

/* QQ51: Version enriquecida de la cancion origen */
export interface CancionOrigenResumen {
    id: number;
    titulo: string;
    slug: string;
    artista?: string | null;
    whosampledUrl?: string | null;
    bpm?: number | null;
}
```

#### 3.2.2 Actualizar normalizador

En `normalizarSampleDetalle()`, mapear los nuevos campos del API response.

#### 3.2.3 Rediseñar "Origen y Sampleo" en `ModalInspectorSample.tsx`

El nuevo diseño debe mostrar (reemplazar los IDs crudos):

```
Origen y Sampleo
─────────────────
🎵 Canción de Origen:  Artista — Título
🔗 WhoSampled:         https://www.whosampled.com/track/xxx/
🎚️ BPM (WhoSampled):   94

📋 Relación Sampleo ID: 68
🔗 WhoSampled Relación: https://www.whosampled.com/sample/68/

📦 Extracción (SeccionExtraccionInspector)
   (se renderiza automáticamente si extraccion existe)
```

La sección actual de "Origen y Sampleo" se simplifica para mostrar datos enriquecidos en lugar de IDs crudos. La data de extracción (YouTube, Spotify, timestamps, BPM detectado, canciones fuente/destino) ya la maneja `SeccionExtraccionInspector` — no duplicar.

---

## 4. Archivos Afectados (SOLO los necesarios)

### Backend (Rust)

| Archivo                              | Acción                                                       |
| ------------------------------------ | ------------------------------------------------------------ |
| `src/models/sample.rs`               | Agregar `CancionOrigenResumen`, `ExtraccionSampleResponse`, campos nuevos en `SampleDetailResponse` |
| `src/repositories/sample_catalog.rs` | Agregar LEFT JOINs a `find_sample_by_slug_or_short_id`, agregar campos a `SampleCatalogDetailRecord` |
| `src/services/sample_catalog/mod.rs` | Actualizar `build_sample_detail()` para popular los nuevos campos |

### Frontend (TypeScript/React)

| Archivo                                                       | Acción                                              |
| ------------------------------------------------------------- | --------------------------------------------------- |
| `frontend/src/legacy/types/sample.ts`                         | Agregar `CancionOrigenResumen`, `whosampledUrl`     |
| `frontend/src/legacy/services/normalizers/sampleNormalizer.ts` | Mapear `cancionOrigen`, `extraccion`, `whosampledUrl` |
| `frontend/src/legacy/components/ui/ModalInspectorSample.tsx`  | Rediseñar sección "Origen y Sampleo" con datos rich |

**NO se tocan:**
- Scraper Python (ya extrae todo lo necesario, excepto BPM que es nice-to-have)
- `SeccionExtraccionInspector.tsx` (ya funciona, solo necesita datos)
- `TablaRelaciones.tsx` (fuera del alcance)
- `LadoCancionRelacion.tsx` (fuera del alcance)
- `CancionDetalleIsland.tsx` (fuera del alcance)

---

## 5. Dependencias y Orden de Implementación

| Orden | Acción                                          | Depende de |
| ----- | ----------------------------------------------- | ---------- |
| 1     | Rust: agregar structs `CancionOrigenResumen`, `ExtraccionSampleResponse` | —          |
| 2     | Rust: agregar LEFT JOINs a SQL query + `SampleCatalogDetailRecord` | —          |
| 3     | Rust: actualizar `build_sample_detail()`        | 1, 2       |
| 4     | Rust: `cargo check`, `cargo clippy`, `cargo test` | 3          |
| 5     | Frontend: agregar tipos `CancionOrigenResumen`, campos a `Sample` | —          |
| 6     | Frontend: actualizar normalizador               | 5          |
| 7     | Frontend: rediseñar "Origen y Sampleo" en modal | 5, 6       |
| 8     | `npm run codegen` (regenerar cliente OpenAPI)   | 4          |
| 9     | `npm run type-check` + test visual              | 7, 8       |

**Esfuerzo estimado total:** 4-6h (2-3h Rust + 1-2h frontend + 1h validación/correcciones)

---

## 6. Notas Técnicas

### 6.1 `cola_extraccion_samples` puede tener múltiples filas

Un sample puede tener múltiples entradas en `cola_extraccion_samples` (por reintentos o diferentes métodos de extracción). La solución propuesta usa `LEFT JOIN LATERAL ... ORDER BY creado_en DESC LIMIT 1` para traer solo la más reciente. Si se necesita historial completo, sería otro endpoint.

### 6.2 `metadata_extraccion` (JSONB) contiene los campos QQ23

El extractor de audio guarda metadata enriquecida en `metadata_extraccion` como JSONB. Los campos `sampleo_fuente_titulo`, `sampleo_fuente_artista`, `fuente_slug`, etc. viven dentro de ese JSON. La API debe deserializarlos al struct `ExtraccionSampleResponse`.

### 6.3 BPM: dos fuentes

- **`canciones.bpm`** (WhoSampled) — a menudo NULL porque el scraper no lo extrae (GAP secundario del scraper, no blocker)
- **`cola_extraccion_samples.bpm_detectado`** — BPM detectado por análisis de audio (más preciso, disponible post-extracción)

El modal puede mostrar ambos si existen, indicando la fuente.

### 6.4 WhoSampled URL

- Para la **canción origen**: viene de `canciones.whosampled_url` (URL completa)
- Para la **relación de sampleo**: se construye desde `relaciones_sample.whosampled_id` como `https://www.whosampled.com/sample/{id}/`

### 6.5 Estado actual de `ModalInspectorSample.tsx`

```typescript
// Líneas 124-150 actuales — MUESTRA SOLO IDs CRUDOS
{completo && ((datos as Sample).cancionOrigenId || (datos as Sample).relacionSampleoId) && (
    <div className="inspectorSeccion">
        <div className="inspectorSeccionTitulo">
            <Layers size={14} /> Origen y Sampleo
        </div>
        <div className="inspectorGrid">
            <Campo etiqueta="Es Recorte" valor={...} />
            <Campo etiqueta="Cancion Origen ID" valor={(datos as Sample).cancionOrigenId} numerico />
            {(datos as Sample).cancionOrigen?.titulo && (
                <Campo etiqueta="Cancion Origen" valor={(datos as Sample).cancionOrigen!.titulo} ancho />
            )}
            {(datos as Sample).cancionOrigen?.slug && (
                <Campo etiqueta="Enlace Fuente" valor={`/cancion/${(datos as Sample).cancionOrigen!.slug}/`} ancho />
            )}
            <Campo etiqueta="Relación Sampleo ID" valor={(datos as Sample).relacionSampleoId} numerico />
        </div>
    </div>
)}
```

### 6.6 Diseño propuesto para "Origen y Sampleo" (después del fix)

```typescript
{completo && (datos.cancionOrigen || datos.relacionSampleoId) && (
    <div className="inspectorSeccion">
        <div className="inspectorSeccionTitulo">
            <Layers size={14} /> Origen y Sampleo
        </div>
        <div className="inspectorGrid">
            <Campo etiqueta="Es Recorte" valor={datos.cancionOrigen != null || datos.relacionSampleoId != null} />

            {/* QQ51: Cancion origen enriquecida */}
            {datos.cancionOrigen && (
                <>
                    <Campo etiqueta="Canción Origen" valor={
                        [datos.cancionOrigen.artista, datos.cancionOrigen.titulo].filter(Boolean).join(' — ')
                    } ancho />
                    <CampoLink etiqueta="Enlace" url={`/cancion/${datos.cancionOrigen.slug}/`} texto="Ver canción" />
                    {datos.cancionOrigen.whosampledUrl && (
                        <CampoLink etiqueta="WhoSampled" url={datos.cancionOrigen.whosampledUrl} texto="WhoSampled" />
                    )}
                    {datos.cancionOrigen.bpm != null && (
                        <Campo etiqueta="BPM (WhoSampled)" valor={datos.cancionOrigen.bpm} numerico />
                    )}
                </>
            )}

            {/* QQ51: Relacion sampleo — mostrar ID y WhoSampled si existe */}
            {datos.relacionSampleoId != null && (
                <>
                    <Campo etiqueta="Relación ID" valor={datos.relacionSampleoId} numerico />
                    {datos.whosampledUrl && (
                        <CampoLink etiqueta="WhoSampled Relación" url={datos.whosampledUrl} texto="WhoSampled" />
                    )}
                </>
            )}
        </div>
    </div>
)}
```

Esto requiere que `CampoLink` esté disponible en el scope (extraerlo a un lugar compartido o definirlo localmente).

---

## 7. Resumen

| Concepto                                                      | Estado |
| ------------------------------------------------------------- | ------ |
| ¿El scraper extrae los datos necesarios?                      | ✅ Sí (excepto BPM de WhoSampled, nice-to-have) |
| ¿La BD tiene los datos?                                       | ✅ Sí |
| ¿El frontend `SeccionExtraccionInspector` ya renderiza todo?  | ✅ Sí |
| ¿El frontend `ModalInspectorSample` tiene la estructura?      | ✅ Sí (pero muestra solo IDs) |
| **¿El API Rust sirve datos enriquecidos?**                    | **❌ No — ÚNICO GAP real** |
| **¿El normalizador frontend mapea datos enriquecidos?**       | **❌ No — porque el API no los envía** |

**El bloqueador es exclusivamente la API REST.** Una vez que `SampleDetailResponse` incluya `cancion_origen` (enriquecido) y `extraccion`, el modal funciona al 100% sin cambios en `SeccionExtraccionInspector` y con cambios mínimos en la sección "Origen y Sampleo".

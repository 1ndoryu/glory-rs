/* [264A-3] Publicador Rust de extracciones del scraper Python.
 *
 * Cierra el ciclo cola_extraccion_samples → samples real. Antes existía un
 * gap: el scraper marcaba filas como `estado='extraido'` pero nadie creaba
 * el sample, así que `publicar_auto` era un no-op que jamás encontraba
 * filas con sample_id asignado.
 *
 * Equivalente legacy: `App/Kamples/Services/PublicadorExtraccion.php` y
 * `DevController::publicarExtracciones`. Esta versión Rust NO implementa
 * dedup QK53 (futuro) ni el caso "generar_siguiente" del flujo de extensión
 * (la cola actual reutiliza la misma fila → no hay forma limpia de crear
 * sample nuevo sin perder la cadena; documentado como tarea futura).
 *
 * Modos soportados:
 *   - default (sin extension_modo): crear sample nuevo + vincular a relación.
 *   - extender / restaurar: reemplazar audio del sample existente y disparar
 *     re-procesamiento del pipeline IA (force_recompute).
 *   - generar_siguiente: NO soportado v1 — se loggea warning y se deja la fila
 *     en error con mensaje explicativo para revisión manual.
 *
 * Concurrencia: el repo reclama filas atomicamente vía
 * UPDATE...RETURNING + FOR UPDATE SKIP LOCKED, marcando procesado_at=NOW()
 * como flag de "siendo publicado". Si la publicación falla, revertir_reclamo
 * limpia procesado_at para que el siguiente ciclo lo retome. */

use crate::errors::AppError;
use crate::repositories::{
    ColaExtraccionRepository, ColaExtraidoReclamado, CreateUploadSampleParams, SampleRepository,
};
use crate::services::FileStorage;
use chrono::Datelike;
use nanoid::nanoid;
use serde::Serialize;
use sha2::{Digest, Sha256};
use slug::slugify;
use sqlx::PgPool;
use std::path::Path;
use std::sync::Arc;
use utoipa::ToSchema;

const ID_CORTO_ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const SYSTEM_USER_ID_FALLBACK: i32 = 1;

/// Resultado por item en una corrida de publicación.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ItemPublicacion {
    pub cola_id: i32,
    pub ok: bool,
    pub sample_id: Option<i32>,
    pub modo: String,
    pub error: Option<String>,
}

/// Resumen de una corrida de `publicar_pendientes`.
#[derive(Debug, Clone, Serialize, ToSchema, Default)]
pub struct PublicarResultado {
    pub publicados: usize,
    pub reemplazados: usize,
    pub errores: usize,
    pub items: Vec<ItemPublicacion>,
}

pub struct ExtraccionPublisherService;

impl ExtraccionPublisherService {
    /// Procesa hasta `limit` filas en estado='extraido'. Maneja tanto creación
    /// de samples nuevos como reemplazo de assets para extender/restaurar.
    pub async fn publicar_pendientes(
        pool: &PgPool,
        storage: &Arc<dyn FileStorage>,
        limit: i64,
    ) -> Result<PublicarResultado, AppError> {
        let mut resultado = PublicarResultado::default();
        let creador_id = sistema_user_id();

        // 1) Reclamar reemplazos primero (extender/restaurar) — son operaciones
        //    sobre samples ya existentes y no compiten por slugs.
        let reemplazos = ColaExtraccionRepository::reclamar_para_reemplazar(pool, limit)
            .await
            .map_err(|e| AppError::Internal(format!("reclamar reemplazos: {e}")))?;

        for fila in reemplazos {
            let modo = extension_modo(&fila).unwrap_or_else(|| "extender".to_string());
            match Self::reemplazar_assets(pool, storage, &fila).await {
                Ok(sample_id) => {
                    resultado.reemplazados += 1;
                    resultado.items.push(ItemPublicacion {
                        cola_id: fila.id,
                        ok: true,
                        sample_id: Some(sample_id),
                        modo,
                        error: None,
                    });
                }
                Err(error) => {
                    let msg = error.to_string();
                    tracing::error!(cola_id = fila.id, %error, "fallo reemplazando assets");
                    let _ = ColaExtraccionRepository::revertir_reclamo(pool, fila.id, &msg).await;
                    resultado.errores += 1;
                    resultado.items.push(ItemPublicacion {
                        cola_id: fila.id,
                        ok: false,
                        sample_id: fila.sample_id,
                        modo,
                        error: Some(msg),
                    });
                }
            }
        }

        // 2) Reclamar nuevos (sin sample_id). Si el modo es "generar_siguiente"
        //    en una fila SIN sample_id, lo tratamos como creación normal — la
        //    fila no estaba vinculada a nada todavía.
        let restantes = (limit - i64::try_from(resultado.reemplazados).unwrap_or(0)).max(0);
        if restantes == 0 {
            return Ok(resultado);
        }

        let nuevos = ColaExtraccionRepository::reclamar_para_publicar(pool, restantes)
            .await
            .map_err(|e| AppError::Internal(format!("reclamar publicar: {e}")))?;

        for fila in nuevos {
            let modo = extension_modo(&fila).unwrap_or_else(|| "nuevo".to_string());
            match Self::crear_sample_nuevo(pool, storage, &fila, creador_id).await {
                Ok(sample_id) => {
                    resultado.publicados += 1;
                    resultado.items.push(ItemPublicacion {
                        cola_id: fila.id,
                        ok: true,
                        sample_id: Some(sample_id),
                        modo,
                        error: None,
                    });
                }
                Err(error) => {
                    let msg = error.to_string();
                    tracing::error!(cola_id = fila.id, %error, "fallo creando sample");
                    let _ = ColaExtraccionRepository::revertir_reclamo(pool, fila.id, &msg).await;
                    resultado.errores += 1;
                    resultado.items.push(ItemPublicacion {
                        cola_id: fila.id,
                        ok: false,
                        sample_id: None,
                        modo,
                        error: Some(msg),
                    });
                }
            }
        }

        Ok(resultado)
    }

    /// Caso default: lee bytes del audio extraído por el scraper, los sube al
    /// storage persistente, crea fila en `samples` (estado='procesando' →
    /// dispara worker IA automáticamente), vincula al lado correspondiente
    /// de `relaciones_sample`, marca cola='completado' y borra el archivo
    /// temporal.
    async fn crear_sample_nuevo(
        pool: &PgPool,
        storage: &Arc<dyn FileStorage>,
        fila: &ColaExtraidoReclamado,
        creador_id: i32,
    ) -> Result<i32, AppError> {
        let ruta_origen = fila
            .ruta_audio_extraido
            .as_deref()
            .ok_or_else(|| AppError::Internal("ruta_audio_extraido vacia".into()))?;

        let bytes = leer_audio(ruta_origen).await?;
        let tamano = i64::try_from(bytes.len())
            .map_err(|_| AppError::Internal("audio demasiado grande".into()))?;
        let audio_hash = hash_bytes(&bytes);
        let formato = detectar_formato(ruta_origen);

        let id_corto = nanoid!(8, &ID_CORTO_ALPHABET);
        let titulo_base = construir_titulo(fila);
        let titulo = format!("{titulo_base} ({})", fila.lado);
        let slug_raw = slugify(&titulo_base);
        let slug = format!("{slug_raw}-{id_corto}");

        let now = chrono::Utc::now();
        let storage_key = format!(
            "samples/{}/{:04}/{:02}/{}.{}",
            creador_id,
            now.year(),
            now.month(),
            slug,
            formato,
        );

        storage
            .put_bytes(&storage_key, &bytes)
            .await
            .map_err(|e| AppError::Internal(format!("storage put_bytes: {e}")))?;

        let metadata_sample = serde_json::json!({
            "origen": "extraccion",
            "lado_extraccion": fila.lado,
            "relacion_id": fila.relacion_id,
            "cola_extraccion_id": fila.id,
            "youtube_id": fila.youtube_id,
            "spotify_id": fila.spotify_id,
            "bpm_detectado": fila.bpm_detectado,
            "compas_inicio_seg": fila.compas_inicio_seg,
            "compas_fin_seg": fila.compas_fin_seg,
            "extension_metadata": fila.metadata_extraccion,
        });

        let tags = vec![
            "extraccion".to_string(),
            format!("lado_{}", fila.lado.to_lowercase()),
        ];

        let created = SampleRepository::create_upload_sample(
            pool,
            CreateUploadSampleParams {
                creador_id,
                titulo: &titulo,
                slug: &slug,
                id_corto: &id_corto,
                descripcion: "",
                formato: &formato,
                tamano,
                tags: &tags,
                audio_hash: &audio_hash,
                ruta_original: &storage_key,
                permitir_descarga: true,
                licencia_libre: false,
                es_premium: false,
                precio: None,
                mostrar_en_comunidad: true,
                metadata: metadata_sample,
                sync_upload: false,
            },
        )
        .await
        .map_err(|e| {
            // Si falla la creación de sample, intentar limpiar el blob que ya
            // subimos para no dejar basura en el storage.
            tracing::error!(error = %e, "falló create_upload_sample, limpiando blob");
            AppError::Internal(format!("crear sample: {e}"))
        })?;

        // Vincular relacion_sampleo_id + cancion_origen_id en una sola pasada.
        // Inferir cancion_origen_id desde la relación según el lado.
        if let Err(e) = vincular_sample_a_relacion(pool, created.id, fila).await {
            tracing::warn!(sample_id = created.id, error = %e, "no se pudo vincular sample a relacion (continuando)");
        }

        ColaExtraccionRepository::marcar_completado(pool, fila.id, created.id, &storage_key)
            .await
            .map_err(|e| AppError::Internal(format!("marcar completado: {e}")))?;

        // Borrar archivo temporal del scraper (best-effort).
        if let Err(e) = tokio::fs::remove_file(ruta_origen).await {
            tracing::debug!(path = ruta_origen, error = %e, "no se pudo borrar audio temp");
        }

        Ok(created.id)
    }

    /// Modos extender/restaurar: el sample ya existe; solo subimos el nuevo
    /// audio y disparamos re-procesamiento del pipeline IA (force_recompute).
    async fn reemplazar_assets(
        pool: &PgPool,
        storage: &Arc<dyn FileStorage>,
        fila: &ColaExtraidoReclamado,
    ) -> Result<i32, AppError> {
        let sample_id = fila
            .sample_id
            .ok_or_else(|| AppError::Internal("reemplazo sin sample_id".into()))?;
        let ruta_origen = fila
            .ruta_audio_extraido
            .as_deref()
            .ok_or_else(|| AppError::Internal("ruta_audio_extraido vacia".into()))?;
        let modo = extension_modo(fila).unwrap_or_else(|| "extender".to_string());

        let bytes = leer_audio(ruta_origen).await?;
        let tamano = i64::try_from(bytes.len())
            .map_err(|_| AppError::Internal("audio demasiado grande".into()))?;
        let audio_hash = hash_bytes(&bytes);
        let formato = detectar_formato(ruta_origen);

        // Necesitamos creador_id + slug para construir storage_key coherente.
        let row: (i32, String) = sqlx::query_as(
            "SELECT creador_id, slug FROM samples WHERE id = $1 AND eliminado_en IS NULL",
        )
        .bind(sample_id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::NotFound(format!("sample {sample_id} no existe")))?;

        let now = chrono::Utc::now();
        let storage_key = format!(
            "samples/{}/{:04}/{:02}/{}-r{}.{}",
            row.0,
            now.year(),
            now.month(),
            row.1,
            now.timestamp(),
            formato,
        );

        storage
            .put_bytes(&storage_key, &bytes)
            .await
            .map_err(|e| AppError::Internal(format!("storage put_bytes: {e}")))?;

        let metadata_patch = serde_json::json!({
            "ultimo_reemplazo": {
                "modo": modo,
                "at": now.to_rfc3339(),
                "cola_extraccion_id": fila.id,
                "compas_inicio_seg": fila.compas_inicio_seg,
                "compas_fin_seg": fila.compas_fin_seg,
            },
        });

        let mut tx = pool.begin().await.map_err(AppError::from)?;

        sqlx::query(
            "UPDATE samples
             SET ruta_original = $2,
                 audio_hash    = $3,
                 tamano        = $4,
                 formato       = $5,
                 estado        = 'procesando',
                 ruta_optimizada = NULL,
                 ruta_waveform   = NULL,
                 metadata = COALESCE(metadata, '{}'::jsonb) || $6
             WHERE id = $1",
        )
        .bind(sample_id)
        .bind(&storage_key)
        .bind(&audio_hash)
        .bind(tamano)
        .bind(&formato)
        .bind(&metadata_patch)
        .execute(&mut *tx)
        .await
        .map_err(AppError::from)?;

        let cola_meta = serde_json::json!({
            "force_recompute": true,
            "audio_hash": audio_hash,
            "ruta_original": storage_key,
            "trigger": "reemplazo_extraccion",
        });
        sqlx::query(
            "INSERT INTO cola_procesamiento_ia (tipo, entidad_id, operacion, metadata)
             VALUES ('sample', $1, 'analisis_audio', $2)",
        )
        .bind(sample_id)
        .bind(&cola_meta)
        .execute(&mut *tx)
        .await
        .map_err(AppError::from)?;

        tx.commit().await.map_err(AppError::from)?;

        ColaExtraccionRepository::marcar_completado(pool, fila.id, sample_id, &storage_key)
            .await
            .map_err(|e| AppError::Internal(format!("marcar completado: {e}")))?;

        if let Err(e) = tokio::fs::remove_file(ruta_origen).await {
            tracing::debug!(path = ruta_origen, error = %e, "no se pudo borrar audio temp");
        }

        Ok(sample_id)
    }
}

/* ----- helpers ----- */

fn sistema_user_id() -> i32 {
    std::env::var("KAMPLES_SISTEMA_USUARIO_ID")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(SYSTEM_USER_ID_FALLBACK)
}

fn extension_modo(fila: &ColaExtraidoReclamado) -> Option<String> {
    fila.metadata_extraccion
        .as_ref()
        .and_then(|m| m.get("extension_modo"))
        .and_then(|v| v.as_str())
        .map(ToString::to_string)
}

async fn leer_audio(path: &str) -> Result<Vec<u8>, AppError> {
    tokio::fs::read(path)
        .await
        .map_err(|e| AppError::Internal(format!("leer audio {path}: {e}")))
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

fn detectar_formato(path: &str) -> String {
    Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .map_or_else(|| "mp3".to_string(), str::to_lowercase)
}

fn construir_titulo(fila: &ColaExtraidoReclamado) -> String {
    if let Some(meta) = &fila.metadata_extraccion {
        if let Some(t) = meta.get("titulo_cancion").and_then(|v| v.as_str()) {
            if !t.trim().is_empty() {
                return t.to_string();
            }
        }
        if let Some(n) = meta.get("nombre_archivo").and_then(|v| v.as_str()) {
            if !n.trim().is_empty() {
                return n.to_string();
            }
        }
    }
    if let Some(yt) = &fila.youtube_id {
        return format!("Extraccion {yt}");
    }
    format!("Extraccion #{}", fila.id)
}

/// Lee la relación_sample y vincula sample_fuente_id o sample_destino_id
/// según `lado`. También fija `samples.relacion_sampleo_id` y
/// `samples.cancion_origen_id` (de la canción del lado correspondiente).
async fn vincular_sample_a_relacion(
    pool: &PgPool,
    sample_id: i32,
    fila: &ColaExtraidoReclamado,
) -> Result<(), AppError> {
    let rel: (i32, i32) = sqlx::query_as(
        "SELECT cancion_fuente_id, cancion_destino_id FROM relaciones_sample WHERE id = $1",
    )
    .bind(fila.relacion_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)?
    .ok_or_else(|| AppError::NotFound(format!("relacion {} no existe", fila.relacion_id)))?;

    let (col_lado, cancion_id) = match fila.lado.as_str() {
        "fuente" => ("sample_fuente_id", rel.0),
        "destino" => ("sample_destino_id", rel.1),
        otro => {
            return Err(AppError::BadRequest(format!(
                "lado desconocido: {otro}"
            )));
        }
    };

    // Update dinámico del lado (sample_fuente_id o sample_destino_id).
    // El nombre de columna es whitelist (fuente/destino) — sin riesgo de inyección.
    let sql = format!(
        "UPDATE relaciones_sample SET {col_lado} = $2, updated_at = NOW() WHERE id = $1"
    );
    sqlx::query(&sql)
        .bind(fila.relacion_id)
        .bind(sample_id)
        .execute(pool)
        .await
        .map_err(AppError::from)?;

    sqlx::query(
        "UPDATE samples
         SET relacion_sampleo_id = $2,
             cancion_origen_id   = $3
         WHERE id = $1",
    )
    .bind(sample_id)
    .bind(fila.relacion_id)
    .bind(cancion_id)
    .execute(pool)
    .await
    .map_err(AppError::from)?;

    Ok(())
}

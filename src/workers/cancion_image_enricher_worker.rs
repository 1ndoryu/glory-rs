/* [304A-1] Worker lento de enriquecimiento de portadas de canciones.
 *
 * Problema: ~2.7k canciones tienen `imagen_url` NULL porque el scraper de
 * WhoSampled no siempre extrae la portada. Sin imagen en la cancion origen,
 * el sample extraido tampoco hereda imagen → feed/admin renderiza placeholder.
 *
 * Estrategia (lento e idempotente, sin lotes grandes ni rate-limit storms):
 *   - Cada `INTERVAL` segundos, toma UNA cancion sin imagen.
 *   - Consulta iTunes Search API (gratis, sin API key, ~20 req/min) por
 *     "<artista> <titulo>" y extrae `artworkUrl100` → upgrade a 600x600.
 *   - Si encuentra coincidencia, UPDATE imagen_url + marca metadata.imagen_fuente.
 *   - Si no, marca metadata.imagen_lookup_at = now() para no reintentar pronto
 *     (cooldown de 30 dias antes del proximo intento).
 *
 * Por que iTunes:
 *   - Devuelve artwork de alta calidad (cambiar `100x100bb.jpg` → `600x600bb.jpg`).
 *   - Sin API key, sin OAuth, sin rate limit estricto documentado.
 *   - Cobertura amplia para musica comercial (mainstream + catalogo).
 *
 * NO bloquea: errores se loguean, la cancion se reintenta tras cooldown.
 * NO satura: 1 cancion cada 30s = ~2880/dia, suficiente para drenar 2.7k en ~24h.
 */

use serde::Deserialize;
use sqlx::PgPool;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

const TICK_INTERVAL: Duration = Duration::from_secs(30);
const ERROR_INTERVAL: Duration = Duration::from_secs(120);
const HTTP_TIMEOUT: Duration = Duration::from_secs(15);
const COOLDOWN_DAYS: i32 = 30;

pub fn spawn_cancion_image_enricher_worker(pool: &PgPool) -> JoinHandle<()> {
    let pool = pool.clone();
    tokio::spawn(async move {
        run_forever(pool).await;
    })
}

async fn run_forever(pool: PgPool) {
    tracing::info!(
        "cancion image enricher worker iniciado (intervalo: {:?})",
        TICK_INTERVAL
    );

    let client = match reqwest::Client::builder()
        .timeout(HTTP_TIMEOUT)
        .user_agent("glory-kamples-image-enricher/1.0")
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            tracing::error!(%e, "no se pudo construir reqwest client; worker abortado");
            return;
        }
    };

    loop {
        match procesar_una(&pool, &client).await {
            Ok(true) => sleep(TICK_INTERVAL).await,
            Ok(false) => {
                /* Sin candidatos: dormir mas para no martillar la BD. */
                sleep(Duration::from_secs(600)).await;
            }
            Err(e) => {
                tracing::error!(%e, "error en cancion image enricher tick");
                sleep(ERROR_INTERVAL).await;
            }
        }
    }
}

async fn procesar_una(pool: &PgPool, client: &reqwest::Client) -> Result<bool, sqlx::Error> {
    /* Selecciona una cancion sin imagen y sin lookup reciente.
     * Prioriza canciones con `total_sampleada > 0` (las que aparecen en relaciones)
     * para que el feed/extracciones se beneficien primero. */
    let row: Option<(i32, String, String)> = sqlx::query_as(
        r"
        SELECT c.id, c.titulo, a.nombre
          FROM canciones c
          INNER JOIN artistas_musicales a ON a.id = c.artista_id
         WHERE (c.imagen_url IS NULL OR TRIM(c.imagen_url) = '')
           AND COALESCE(
                 (c.metadata->>'imagen_lookup_at')::timestamptz,
                 'epoch'::timestamptz
               ) < NOW() - make_interval(days => $1)
         ORDER BY c.total_sampleada DESC NULLS LAST, c.id ASC
         LIMIT 1
        ",
    )
    .bind(COOLDOWN_DAYS)
    .fetch_optional(pool)
    .await?;

    let Some((cancion_id, titulo, artista)) = row else {
        return Ok(false);
    };

    let query = format!("{} {}", artista.trim(), titulo.trim());
    let trimmed = query.trim();
    if trimmed.is_empty() {
        marcar_lookup(pool, cancion_id, None, "query_vacia").await?;
        return Ok(true);
    }

    match buscar_artwork_itunes(client, trimmed).await {
        Ok(Some(url)) => {
            sqlx::query(
                r"
                UPDATE canciones
                   SET imagen_url = $2,
                       metadata = COALESCE(metadata, '{}'::jsonb)
                                  || jsonb_build_object(
                                       'imagen_fuente', 'itunes',
                                       'imagen_lookup_at', NOW()::text
                                     )
                 WHERE id = $1
                ",
            )
            .bind(cancion_id)
            .bind(&url)
            .execute(pool)
            .await?;
            tracing::info!(cancion_id, url = %url, "portada enriquecida (itunes)");
        }
        Ok(None) => {
            marcar_lookup(pool, cancion_id, None, "sin_resultados").await?;
        }
        Err(e) => {
            tracing::warn!(cancion_id, %e, "error consultando iTunes");
            marcar_lookup(pool, cancion_id, None, "error_http").await?;
        }
    }

    Ok(true)
}

async fn marcar_lookup(
    pool: &PgPool,
    cancion_id: i32,
    _url: Option<&str>,
    motivo: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r"
        UPDATE canciones
           SET metadata = COALESCE(metadata, '{}'::jsonb)
                          || jsonb_build_object(
                               'imagen_lookup_at', NOW()::text,
                               'imagen_lookup_motivo', $2::text
                             )
         WHERE id = $1
        ",
    )
    .bind(cancion_id)
    .bind(motivo)
    .execute(pool)
    .await?;
    Ok(())
}

#[derive(Debug, Deserialize)]
struct ItunesResponse {
    results: Vec<ItunesResult>,
}

#[derive(Debug, Deserialize)]
struct ItunesResult {
    #[serde(rename = "artworkUrl100")]
    artwork_url_100: Option<String>,
}

async fn buscar_artwork_itunes(
    client: &reqwest::Client,
    query: &str,
) -> Result<Option<String>, reqwest::Error> {
    let url = "https://itunes.apple.com/search";
    let resp = client
        .get(url)
        .query(&[
            ("term", query),
            ("media", "music"),
            ("entity", "song"),
            ("limit", "1"),
        ])
        .send()
        .await?
        .error_for_status()?;
    let body: ItunesResponse = resp.json().await?;
    Ok(body
        .results
        .into_iter()
        .find_map(|r| r.artwork_url_100)
        .map(|u| u.replace("100x100bb.jpg", "600x600bb.jpg")))
}

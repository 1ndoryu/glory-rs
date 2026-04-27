use sqlx::PgPool;

/* [254A-8c-refactor] Repositorio de cola_extraccion_samples para extensión.
 *
 * Cambio vs. versión inicial: ya no recortamos en Rust. Solo encolamos para
 * que el scraper Python procese. `encolar_para_scraper` deja la fila en
 * estado='pendiente' con el nuevo timing y un flag `extension_modo` en
 * metadata_extraccion para que pipeline.py decida cómo tratarla
 * (extender, generar_siguiente, restaurar).
 *
 * Todas las queries usan API no-macro de sqlx (SQLX_OFFLINE-friendly). */

pub struct ColaExtraccionRepository;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ColaExtraccionRow {
    pub id: i32,
    pub relacion_id: i32,
    pub youtube_id: Option<String>,
    pub compas_inicio_seg: Option<f64>,
    pub compas_fin_seg: Option<f64>,
    pub ruta_audio_completo: Option<String>,
    pub sample_id: Option<i32>,
}

pub struct EncolarParams<'a> {
    pub cola_id: i32,
    pub nuevo_inicio: f64,
    pub nuevo_fin: f64,
    /// Una de: "extender", "generar_siguiente", "restaurar".
    pub modo: &'a str,
}

impl ColaExtraccionRepository {
    /// Busca la fila de cola_extraccion_samples vinculada a un sample.
    pub async fn find_by_sample_id(
        pool: &PgPool,
        sample_id: i32,
    ) -> Result<Option<ColaExtraccionRow>, sqlx::Error> {
        sqlx::query_as::<_, ColaExtraccionRow>(
            r#"SELECT
                id,
                relacion_id,
                youtube_id,
                compas_inicio_seg::DOUBLE PRECISION AS compas_inicio_seg,
                compas_fin_seg::DOUBLE PRECISION    AS compas_fin_seg,
                ruta_audio_completo,
                sample_id
             FROM cola_extraccion_samples
             WHERE sample_id = $1
             LIMIT 1"#,
        )
        .bind(sample_id)
        .fetch_optional(pool)
        .await
    }

    /// Marca la fila como pendiente con el nuevo rango y el modo de extensión.
    /// El scraper Python (pipeline.py) leerá `metadata_extraccion.extension_modo`
    /// para decidir cómo tratar el item.
    pub async fn encolar_para_scraper(
        pool: &PgPool,
        params: EncolarParams<'_>,
    ) -> Result<(), sqlx::Error> {
        let metadata_patch = serde_json::json!({
            "extension_modo": params.modo,
            "extension_solicitada_at": chrono::Utc::now().to_rfc3339(),
            "extension_nuevo_inicio": params.nuevo_inicio,
            "extension_nuevo_fin": params.nuevo_fin,
        });

        sqlx::query(
            "UPDATE cola_extraccion_samples
             SET compas_inicio_seg = $2,
                 compas_fin_seg    = $3,
                 estado            = 'pendiente',
                 intentos          = 0,
                 error_mensaje     = NULL,
                 proximo_intento_at = NULL,
                 metadata_extraccion = COALESCE(metadata_extraccion, '{}'::jsonb) || $4
             WHERE id = $1",
        )
        .bind(params.cola_id)
        .bind(params.nuevo_inicio)
        .bind(params.nuevo_fin)
        .bind(&metadata_patch)
        .execute(pool)
        .await?;
        Ok(())
    }
}

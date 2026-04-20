use sqlx::PgPool;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

/* [174A-95] Worker de mantenimiento de la cola de scraping.
 * El scraping real se hace desde el scraper Python externo (`kamples-scraper/`).
 * Este worker solo reagenda URLs con `re_scrapeable=TRUE` cuyo `proximo_rescrape`
 * ya pasó: las marca como `estado='pendiente'` para que el scraper Python las
 * recoja en su próximo poll. Reemplaza al cron PHP `process_scraping_queue`. */

const CHECK_INTERVAL: Duration = Duration::from_secs(900);
const ERROR_INTERVAL: Duration = Duration::from_secs(120);

pub fn spawn_scraping_queue_worker(pool: &PgPool) -> JoinHandle<()> {
    let pool = pool.clone();
    tokio::spawn(async move {
        run_forever(pool).await;
    })
}

async fn run_forever(pool: PgPool) {
    tracing::info!(
        "scraping queue worker iniciado (intervalo: {:?})",
        CHECK_INTERVAL
    );

    loop {
        match reschedule_due_rescrapes(&pool).await {
            Ok(0) => {}
            Ok(count) => {
                tracing::info!(rescheduled = count, "URLs reactivadas para rescrape");
            }
            Err(error) => {
                tracing::error!(%error, "error reagendando rescrapes");
                sleep(ERROR_INTERVAL).await;
                continue;
            }
        }

        sleep(CHECK_INTERVAL).await;
    }
}

async fn reschedule_due_rescrapes(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE scraping_log
        SET estado = 'pendiente',
            veces_rescrapeado = veces_rescrapeado + 1,
            proximo_rescrape = NULL
        WHERE re_scrapeable = TRUE
          AND proximo_rescrape IS NOT NULL
          AND proximo_rescrape < NOW()
          AND estado IN ('procesado', 'error')
        "#,
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

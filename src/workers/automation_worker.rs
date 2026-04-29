use sqlx::PgPool;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

use crate::services::AdminAutomationService;

/* [294A-1] Worker de automatizacion real para scraping/extraccion.
 * Por que: `*_enabled` y `*_intervalo_seg` vivian solo en app_config; no habia
 * ningun loop Rust consumiendo esa config para disparar lotes periodicos. */

const CHECK_INTERVAL: Duration = Duration::from_secs(15);
const ERROR_INTERVAL: Duration = Duration::from_secs(30);

pub fn spawn_automation_worker(pool: &PgPool) -> JoinHandle<()> {
    let pool = pool.clone();
    tokio::spawn(async move {
        run_forever(pool).await;
    })
}

async fn run_forever(pool: PgPool) {
    tracing::info!("automation worker iniciado (tick {:?})", CHECK_INTERVAL);

    loop {
        match AdminAutomationService::run_due(&pool).await {
            Ok(()) => sleep(CHECK_INTERVAL).await,
            Err(error) => {
                tracing::error!(%error, "error ejecutando automation worker");
                sleep(ERROR_INTERVAL).await;
            }
        }
    }
}

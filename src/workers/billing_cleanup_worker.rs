use sqlx::PgPool;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

/* [174A-92] Worker de cleanup de suscripciones expiradas.
 * Cada CHECK_INTERVAL recorre `suscripciones` con estado='activa' y `fin_at < NOW()`,
 * marcandolas como 'vencida'. Reemplaza al cron PHP `cleanupExpiredSubscriptions`
 * del legacy. No toca Stripe (esa cancelacion la dispara el webhook); este worker
 * solo refleja en BD las suscripciones cuyo periodo ya termino sin renovacion. */

const CHECK_INTERVAL: Duration = Duration::from_secs(3600);
const ERROR_INTERVAL: Duration = Duration::from_secs(120);

pub fn spawn_billing_cleanup_worker(pool: &PgPool) -> JoinHandle<()> {
    let pool = pool.clone();
    tokio::spawn(async move {
        run_forever(pool).await;
    })
}

async fn run_forever(pool: PgPool) {
    tracing::info!("billing cleanup worker iniciado (intervalo: {:?})", CHECK_INTERVAL);

    loop {
        match expire_due_subscriptions(&pool).await {
            Ok(0) => {}
            Ok(count) => {
                tracing::info!(expired = count, "suscripciones marcadas como vencidas");
            }
            Err(error) => {
                tracing::error!(%error, "error procesando expiracion de suscripciones");
                sleep(ERROR_INTERVAL).await;
                continue;
            }
        }

        sleep(CHECK_INTERVAL).await;
    }
}

async fn expire_due_subscriptions(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE suscripciones
        SET estado = 'vencida'
        WHERE estado = 'activa'
          AND fin_at IS NOT NULL
          AND fin_at < NOW()
        "#,
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

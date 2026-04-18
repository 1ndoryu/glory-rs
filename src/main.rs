use glory_backend::config::AppConfig;
use glory_backend::handlers;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::OpenApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    /* [174A-6] CLI mínima: --emit-openapi <ruta> escribe el schema a disco
     * y termina sin arrancar el servidor. Usado por el frontend (Orval)
     * para regenerar el cliente sin necesidad de un backend corriendo. */
    let args: Vec<String> = std::env::args().collect();
    if let Some(idx) = args.iter().position(|a| a == "--emit-openapi") {
        let path = args.get(idx + 1).cloned().unwrap_or_else(|| "openapi.json".to_string());
        let doc = handlers::ApiDoc::openapi();
        let json = serde_json::to_string_pretty(&doc)?;
        std::fs::write(&path, json)?;
        println!("OpenAPI schema escrito en {path}");
        return Ok(());
    }

    /* [174A-4] Tracing: EnvFilter (RUST_LOG) + formato. JSON si LOG_FORMAT=json,
     * si no formato compacto con spans para correlacionar request_id. */
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("glory_backend=debug,tower_http=info,sqlx=warn"));

    let registry = tracing_subscriber::registry().with(env_filter);

    if std::env::var("LOG_FORMAT").as_deref() == Ok("json") {
        registry
            .with(tracing_subscriber::fmt::layer().json().with_current_span(true).with_span_list(false))
            .init();
    } else {
        registry
            .with(tracing_subscriber::fmt::layer().compact().with_target(false))
            .init();
    }

    let config = AppConfig::from_env()?;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .min_connections(config.db_min_connections)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    /* [174A-5] Redis opcional. Si REDIS_URL está definido, creamos pool deadpool
     * y testeamos conexión con PING; si falla, abortamos al arrancar. */
    let redis = if let Some(url) = config.redis_url.clone() {
        let cfg = deadpool_redis::Config::from_url(url);
        let pool = cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1))?;
        let mut conn = pool.get().await?;
        let _: String = redis::cmd("PING").query_async(&mut conn).await?;
        tracing::info!("Redis conectado");
        Some(pool)
    } else {
        tracing::warn!("REDIS_URL no definido — operando sin cache distribuido");
        None
    };

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Servidor iniciando en {addr}");
    tracing::info!("Swagger UI disponible en http://{addr}/swagger-ui/");

    /* [174A-26+174A-27] Storage backend. STORAGE_BACKEND="local" usa LocalFs (default).
     * STORAGE_BACKEND="s3" requiere compilar con `--features s3` y env S3_BUCKET. */
    let storage: std::sync::Arc<dyn glory_backend::services::FileStorage> =
        if config.storage_backend == "s3" {
            #[cfg(feature = "s3")]
            {
                let bucket = config.s3_bucket.clone().ok_or("S3_BUCKET requerido cuando STORAGE_BACKEND=s3")?;
                let endpoint = config.s3_endpoint_url.clone();
                tracing::info!("Storage S3 bucket={bucket} endpoint={endpoint:?}");
                std::sync::Arc::new(glory_backend::services::S3Storage::new(bucket, endpoint).await?)
            }
            #[cfg(not(feature = "s3"))]
            {
                return Err("STORAGE_BACKEND=s3 pero binario compilado sin feature `s3`".into());
            }
        } else {
            tracing::info!("Storage LocalFs en {}", config.storage_root);
            std::sync::Arc::new(glory_backend::services::LocalFs::new(&config.storage_root).await?)
        };

    let _audio_pipeline_workers = glory_backend::workers::spawn_audio_pipeline_workers(&pool, &storage);
    let _ia_queue_workers = glory_backend::workers::spawn_ia_queue_workers(&pool);

    let push_runtime = glory_backend::services::PushDeliveryRuntime::from_config(&config)?;
    if let Some(runtime) = push_runtime.as_ref() {
        tracing::info!(
            vapid_subject = %runtime.subject(),
            "Web Push VAPID habilitado"
        );
    } else {
        tracing::warn!("VAPID no configurado — Web Push deshabilitado");
    }

    let app = handlers::create_router(pool, redis, config, storage, push_runtime);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

use glory_backend::config::AppConfig;
use glory_backend::handlers;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

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
        .max_connections(10)
        .min_connections(2)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Servidor iniciando en {addr}");
    tracing::info!("Swagger UI disponible en http://{addr}/swagger-ui/");

    let app = handlers::create_router(pool, config);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

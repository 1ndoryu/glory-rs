use std::net::SocketAddr;

use glory_backend::config::AppConfig;
use glory_backend::handlers;
use glory_backend::services::AssignmentService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                tracing_subscriber::EnvFilter::new("glory_backend=debug,tower_http=debug")
            }),
        )
        .init();

    let config = AppConfig::from_env()?;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    /* [044A-38 Fase 4] Background task: auto-asigna órdenes sin empleado tras 24h */
    let bg_pool = pool.clone();
    tokio::spawn(async move {
        AssignmentService::auto_assign_loop(bg_pool).await;
    });

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Servidor iniciando en {addr}");
    tracing::info!("Swagger UI disponible en http://{addr}/swagger-ui/");

    let app = handlers::create_app(pool, config);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    /* [074A-41] into_make_service_with_connect_info para que GovernorLayer
     * (PeerIpKeyExtractor) y ConnectInfo<SocketAddr> en ws_visitor funcionen.
     * Sin esto, tower_governor devuelve "Unable To Extract Key!" en todas las rutas /api/ routes. */
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}

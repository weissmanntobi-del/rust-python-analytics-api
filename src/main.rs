use anyhow::Context;
use redis::aio::ConnectionManager;
use rust_python_analytics_api::{
    app::build_router,
    config::AppConfig,
    queue::spawn_event_worker,
    state::AppState,
};
use sqlx::postgres::PgPoolOptions;
use tokio::sync::mpsc;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env()?;
    config.configure_tracing();

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .context("failed to connect to postgres")?;

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .context("failed to run migrations")?;

    let redis_client = redis::Client::open(config.redis_url.clone())
        .context("failed to create redis client")?;

    let redis = ConnectionManager::new(redis_client)
        .await
        .context("failed to connect to redis")?;

    let (event_tx, event_rx) = mpsc::channel(config.worker_buffer);

    let state = AppState::new(db.clone(), redis, config.clone(), event_tx);
    let _worker = spawn_event_worker(event_rx, db);

    let app = build_router(state);

    let listener = tokio::net::TcpListener::bind(&config.server_addr)
        .await
        .with_context(|| format!("failed to bind {}", config.server_addr))?;

    info!("analytics-api listening on {}", config.server_addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("server error")?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{signal, SignalKind};

        if let Ok(mut sigterm) = signal(SignalKind::terminate()) {
            sigterm.recv().await;
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }

    tracing::info!("shutdown signal received");
}

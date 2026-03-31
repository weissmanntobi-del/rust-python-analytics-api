use crate::config::AppConfig;
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct QueuedEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_name: String,
    pub page_url: Option<String>,
    pub session_id: Option<String>,
    pub properties: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: ConnectionManager,
    pub config: Arc<AppConfig>,
    pub event_tx: mpsc::Sender<QueuedEvent>,
}

impl AppState {
    pub fn new(
        db: PgPool,
        redis: ConnectionManager,
        config: AppConfig,
        event_tx: mpsc::Sender<QueuedEvent>,
    ) -> Self {
        Self {
            db,
            redis,
            config: Arc::new(config),
            event_tx,
        }
    }
}

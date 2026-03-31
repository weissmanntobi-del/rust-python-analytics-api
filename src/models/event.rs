use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_name: String,
    pub page_url: Option<String>,
    pub session_id: Option<String>,
    pub properties: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

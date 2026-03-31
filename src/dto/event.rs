use crate::models::event::EventRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TrackEventRequest {
    pub event_name: String,
    pub page_url: Option<String>,
    pub session_id: Option<String>,
    #[serde(default)]
    pub properties: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct TrackEventAcceptedResponse {
    pub status: String,
    pub event_id: uuid::Uuid,
    pub queued_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RecentEventsQuery {
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct RecentEventsResponse {
    pub items: Vec<EventRecord>,
}

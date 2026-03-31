use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SummaryQuery {
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TimeseriesQuery {
    pub from: Option<String>,
    pub to: Option<String>,
    pub bucket: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventCount {
    pub event_name: String,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SummaryResponse {
    pub total_events: i64,
    pub unique_sessions: i64,
    pub top_events: Vec<EventCount>,
    pub from: chrono::DateTime<chrono::Utc>,
    pub to: chrono::DateTime<chrono::Utc>,
    pub cached: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeseriesPoint {
    pub bucket_start: chrono::DateTime<chrono::Utc>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeseriesResponse {
    pub bucket: String,
    pub points: Vec<TimeseriesPoint>,
    pub from: chrono::DateTime<chrono::Utc>,
    pub to: chrono::DateTime<chrono::Utc>,
}

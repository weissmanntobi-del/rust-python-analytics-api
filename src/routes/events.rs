use crate::{
    dto::event::{RecentEventsQuery, RecentEventsResponse, TrackEventAcceptedResponse, TrackEventRequest},
    error::{AppError, AppResult},
    repository::event_repository,
    services::auth_service,
    state::{AppState, QueuedEvent},
};
use axum::{
    extract::{Query, State},
    http::HeaderMap,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use tokio::sync::mpsc::error::TrySendError;
use uuid::Uuid;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/events", post(track_event))
        .route("/events/recent", get(recent_events))
}

async fn track_event(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<TrackEventRequest>,
) -> AppResult<Json<TrackEventAcceptedResponse>> {
    validate_track_request(&payload)?;
    let user = auth_service::user_from_api_key(&state, &headers).await?;

    let event = QueuedEvent {
        id: Uuid::new_v4(),
        user_id: user.id,
        event_name: payload.event_name.trim().to_string(),
        page_url: payload.page_url.map(|value| value.trim().to_string()),
        session_id: payload.session_id.map(|value| value.trim().to_string()),
        properties: normalize_properties(payload.properties),
        created_at: Utc::now(),
    };

    match state.event_tx.try_send(event.clone()) {
        Ok(_) => Ok(Json(TrackEventAcceptedResponse {
            status: "queued".to_string(),
            event_id: event.id,
            queued_at: event.created_at,
        })),
        Err(TrySendError::Full(_)) => Err(AppError::QueueFull),
        Err(TrySendError::Closed(_)) => Err(AppError::Internal(
            "event worker is unavailable".to_string(),
        )),
    }
}

async fn recent_events(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<RecentEventsQuery>,
) -> AppResult<Json<RecentEventsResponse>> {
    let user = auth_service::user_from_bearer(&state, &headers).await?;
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let items = event_repository::list_recent_events(&state.db, user.id, limit).await?;
    Ok(Json(RecentEventsResponse { items }))
}

fn validate_track_request(payload: &TrackEventRequest) -> AppResult<()> {
    if payload.event_name.trim().is_empty() {
        return Err(AppError::Validation("event_name must not be empty".to_string()));
    }

    if let Some(session_id) = &payload.session_id {
        if session_id.trim().is_empty() {
            return Err(AppError::Validation("session_id must not be empty when provided".to_string()));
        }
    }

    Ok(())
}

fn normalize_properties(properties: serde_json::Value) -> serde_json::Value {
    if properties.is_null() {
        serde_json::json!({})
    } else {
        properties
    }
}

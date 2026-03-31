use crate::{
    dto::analytics::{
        SummaryQuery, SummaryResponse, TimeseriesQuery, TimeseriesResponse,
    },
    error::{AppError, AppResult},
    repository::event_repository,
    state::AppState,
};
use chrono::{DateTime, Duration, Utc};
use redis::AsyncCommands;
use uuid::Uuid;

pub async fn summary(
    state: &AppState,
    user_id: Uuid,
    query: SummaryQuery,
) -> AppResult<SummaryResponse> {
    let (from, to) = resolve_range(query.from.as_deref(), query.to.as_deref())?;
    let cache_key = format!("summary:{user_id}:{from}:{to}");

    let mut redis = state.redis.clone();
    if let Ok(Some(cached)) = redis.get::<_, Option<String>>(&cache_key).await {
        if let Ok(mut parsed) = serde_json::from_str::<SummaryResponse>(&cached) {
            parsed.cached = true;
            return Ok(parsed);
        }
    }

    let (total_events, unique_sessions) =
        event_repository::summary_totals(&state.db, user_id, from, to).await?;
    let top_events = event_repository::top_events(&state.db, user_id, from, to).await?;

    let response = SummaryResponse {
        total_events,
        unique_sessions,
        top_events,
        from,
        to,
        cached: false,
    };

    if let Ok(serialized) = serde_json::to_string(&response) {
        let _: Result<(), _> = redis.set_ex(cache_key, serialized, 60).await;
    }

    Ok(response)
}

pub async fn timeseries(
    state: &AppState,
    user_id: Uuid,
    query: TimeseriesQuery,
) -> AppResult<TimeseriesResponse> {
    let (from, to) = resolve_range(query.from.as_deref(), query.to.as_deref())?;
    let bucket = match query.bucket.as_deref().unwrap_or("day") {
        "hour" => "hour",
        _ => "day",
    }
    .to_string();

    let points = event_repository::timeseries(&state.db, user_id, from, to, &bucket).await?;

    Ok(TimeseriesResponse {
        bucket,
        points,
        from,
        to,
    })
}

fn resolve_range(
    from: Option<&str>,
    to: Option<&str>,
) -> AppResult<(DateTime<Utc>, DateTime<Utc>)> {
    let now = Utc::now();

    let from = match from {
        Some(value) => parse_datetime(value)?,
        None => now - Duration::days(7),
    };

    let to = match to {
        Some(value) => parse_datetime(value)?,
        None => now,
    };

    if from > to {
        return Err(AppError::BadRequest(
            "from must be earlier than or equal to to".to_string(),
        ));
    }

    Ok((from, to))
}

fn parse_datetime(input: &str) -> AppResult<DateTime<Utc>> {
    let parsed = chrono::DateTime::parse_from_rfc3339(input)
        .map_err(|_| AppError::BadRequest(format!("invalid RFC3339 datetime: {input}")))?;

    Ok(parsed.with_timezone(&Utc))
}

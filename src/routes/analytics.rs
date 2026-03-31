use crate::{
    dto::analytics::{SummaryQuery, SummaryResponse, TimeseriesQuery, TimeseriesResponse},
    error::AppResult,
    services::{analytics_service, auth_service},
    state::AppState,
};
use axum::{
    extract::{Query, State},
    http::HeaderMap,
    routing::get,
    Json, Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/analytics/summary", get(summary))
        .route("/analytics/timeseries", get(timeseries))
}

async fn summary(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<SummaryQuery>,
) -> AppResult<Json<SummaryResponse>> {
    let user = auth_service::user_from_bearer(&state, &headers).await?;
    let response = analytics_service::summary(&state, user.id, query).await?;
    Ok(Json(response))
}

async fn timeseries(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<TimeseriesQuery>,
) -> AppResult<Json<TimeseriesResponse>> {
    let user = auth_service::user_from_bearer(&state, &headers).await?;
    let response = analytics_service::timeseries(&state, user.id, query).await?;
    Ok(Json(response))
}

use crate::{error::AppResult, state::AppState};
use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use sqlx::Row;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health/live", get(live))
        .route("/health/ready", get(ready))
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn live() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn ready(State(state): State<AppState>) -> AppResult<Json<HealthResponse>> {
    let row = sqlx::query("SELECT 1 AS value")
        .fetch_one(&state.db)
        .await?;
    let _: i32 = row.try_get("value")?;

    let mut redis = state.redis.clone();
    let _: String = redis::cmd("PING").query_async(&mut redis).await?;

    Ok(Json(HealthResponse { status: "ok" }))
}

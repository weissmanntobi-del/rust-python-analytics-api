use crate::{
    dto::auth::{AuthResponse, LoginRequest, RegisterRequest},
    error::{ AppResult},
    services::auth_service,
    state::AppState,
};
use axum::{
    extract::State,
    http::HeaderMap,
    routing::{get, post},
    Json, Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/me", get(me))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    auth_service::validate_register_request(&payload)?;
    let auth = auth_service::register_user(&state, payload).await?;
    Ok(Json(auth))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    let auth = auth_service::login_user(&state, payload).await?;
    Ok(Json(auth))
}

async fn me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> AppResult<Json<serde_json::Value>> {
    let user = auth_service::user_from_bearer(&state, &headers).await?;
    Ok(Json(serde_json::json!({
        "id": user.id,
        "email": user.email,
        "full_name": user.full_name,
        "created_at": user.created_at,
    })))
}

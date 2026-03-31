use crate::{routes, state::AppState};
use axum::{routing::get, Router};
use tower_http::{
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};

pub fn build_router(state: AppState) -> Router {
    let api = Router::new()
        .merge(routes::health::routes())
        .merge(routes::auth::routes())
        .merge(routes::events::routes())
        .merge(routes::analytics::routes());

    Router::new()
        .route("/", get(root))
        .nest("/api/v1", api)
        .layer(TraceLayer::new_for_http())
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_headers(Any)
                .allow_origin(Any),
        )
        .with_state(state)
}

async fn root() -> &'static str {
    "Rust Python Analytics API is running"
}

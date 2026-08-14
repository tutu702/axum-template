use axum::{
    Router,
    middleware::from_extractor_with_state,
    routing::{get, post},
};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::{
    handler::{auth_handler::AuthUser, login},
    state::AppState,
};

pub fn init_routes(state: AppState) -> Router {
    // Routes that require a valid JWT.
    let protected =
        Router::new().route_layer(from_extractor_with_state::<AuthUser, _>(state.clone()));

    // Public routes (no auth).
    let public = Router::new().route("/login", post(login));

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(
            DefaultMakeSpan::new()
                .level(Level::INFO)
                .include_headers(false),
        )
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    Router::new()
        .nest("/api", protected.merge(public))
        .layer(trace_layer)
        .with_state(state)
}

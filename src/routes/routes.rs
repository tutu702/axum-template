use axum::{Router, routing::post};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::{errors::Result, handler::user_handler::login};

pub fn init_routes() -> Result<Router> {
    let app_router = Router::new().route("/login", post(login));

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(
            DefaultMakeSpan::new()
                .level(Level::INFO)
                .include_headers(false),
        )
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    Ok(Router::new().nest("/api", app_router).layer(trace_layer))
}

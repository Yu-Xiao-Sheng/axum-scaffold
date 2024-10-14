pub mod api;

use axum::Router;
use tower_http::trace::TraceLayer;

pub fn create_app() -> axum::Router {
    Router::new()
        .nest("/api", api::create_routers())
        .layer(TraceLayer::new_for_http())
}
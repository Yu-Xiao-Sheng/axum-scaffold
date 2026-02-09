use axum::http::Response;
use axum::response::IntoResponse;
use axum::Router;

pub fn create_routers() -> Router {
    Router::new()
        .route("/health_check", axum::routing::get(health_check))
}

async fn health_check() -> impl IntoResponse {
    Response::<String>::new("ok".into())
}
use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

pub fn create_app(serve_directory: &str) -> Router {
    let serve_dir = ServeDir::new(serve_directory);

    Router::new()
    .route("/world", get(|| async { "Hello, World!" }))
    .fallback_service(serve_dir)
}
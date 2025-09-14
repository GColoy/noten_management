use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

pub fn create_app(spa_directory: &Path) -> Router {
    let serve_file = ServeFile::new(spa_directory.join("index.html"));
    let serve_dir = ServeDir::new(spa_directory)
    .not_found_service(serve_file);

    Router::new()
    .route("/world", get(|| async { "Hello, World!" }))
    .fallback_service(serve_dir)
}
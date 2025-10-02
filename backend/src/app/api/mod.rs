mod sheet;

use axum::Router;

pub fn create_api_router() -> Router {
    Router::new()
    .route("sheet/{id}", method_router);
}
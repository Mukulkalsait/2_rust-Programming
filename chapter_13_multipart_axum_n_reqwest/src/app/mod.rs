use axum::{routing::get, routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

// EXT
use crate::handlers::*;

/// > This function is not await.
/// > unless we are actually awating someting inside it like db connection and all  
///
pub fn build_app() -> axum::Router {
    let cores = CorsLayer::new().allow_origin(Any).allow_headers(Any).allow_methods(Any);

    Router::new()
        .route("/", get(functions::workign))
        .route("/home", get(functions::home_page))
        .route("/test0", get(functions::test_dummy_file_save))
        .route("/upload", post(functions::get_upload_handler))
        .route("/file/{filename}", get(functions::check_if_post_file_present_or_not))
        .layer(cores)
}

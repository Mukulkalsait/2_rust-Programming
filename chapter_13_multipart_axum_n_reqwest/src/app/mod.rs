use axum::{Router, routing::delete, routing::get, routing::post};
use tower_http::cors::{Any, CorsLayer};

// EXT
use crate::handlers::{
    functions::{transfer_create_data, transfer_search_handler},
    *,
};

/// > This function is not await.
/// > unless we are actually awating someting inside it like db connection and all  
///
pub fn build_app() -> axum::Router {
    let cores = CorsLayer::new().allow_origin(Any).allow_headers(Any).allow_methods(Any);

    Router::new()
        //
        // Y: Test Functions
        .route("/", get(functions::workign))
        .route("/home", get(functions::home_page))
        //
        // Y: Data Transfering Functions
        .route("/transfer_search", get(transfer_search_handler))
        .route("/transfer_data", post(transfer_create_data))
        // .route("/transfer_upload", post(multipart_transfer_handler))
        //
        // Y: Processing Functions
        .route("/test0", get(functions::test_dummy_file_saveing))
        .route("/file/{filename}", get(functions::check_file_present_or_not))
        .route("/upload", post(functions::post_save_file)) // Y:  file should be come inside the request
        .route("/delete/{filename}", delete(functions::delete_file))
        .route("/delete/{filename}", delete(functions::delete_best_implimentation))
        .layer(cores)
}

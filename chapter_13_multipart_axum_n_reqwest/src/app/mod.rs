use axum::{Router, routing::delete, routing::get, routing::post};
use tower_http::cors::{Any, CorsLayer};

// EXT
use crate::handlers::functions::*;

/// > This function is not await.
/// > unless we are actually awating someting inside it like db connection and all  
///
pub fn build_app() -> axum::Router {
    let cores = CorsLayer::new().allow_origin(Any).allow_headers(Any).allow_methods(Any);

    Router::new()
        //
        // Y: Test Functions
        .route("/", get(workign))
        .route("/home", get(home_page))
        //
        // Y: Data Transfering Functions
        .route("/transfer_search", get(transfer_search_handler))
        .route("/transfer_data", post(transfer_create_data))
        // .route("/transfer_upload", post(multipart_transfer_handler))
        //
        // Y: Processing Functions
        .route("/test0", get(test_dummy_file_saveing)) // Dummy File Saver
        // G: Individual files
        .route("/upload", post(post_save_file)) // Y: Uploded File Saver =>  file should be come inside the request
        .route("/file/{filename}", get(check_file_present_or_not)) // Uploaded FIle checker
        .route("/delete/{filename}", delete(delete_file)) // delete uploaded file
        // .route("/delete/{filename}", delete(delete_best_implimentation)) // delete option 2
        // G: Folder Fn
        .route("/all_files", get(get_all_files_handler)) // All Files Metadata from upload folder
        //
        // .route("/file/{filename}", delete(functions::delete_best_implimentation))
        .layer(cores)
}

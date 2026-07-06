use std::collections::HashMap;

use crate::handlers::helper_functions::{get_upload_dir, save_file_on_default_upload_directory};
use axum::{
    self,
    Json,
    extract::{Multipart, Path, Query},
    http::StatusCode,
    response::{ErrorResponse, IntoResponse},
};

// Y: ============================================================================ Basic fn
//
/// just working or not.
pub async fn workign() -> String { "✅ WORKING".to_string() }
/// just working or not for homepage.
pub async fn home_page() -> String { "󰠧 HOME ".to_string() }

// Y: ============================================================================ INFO GETING FUNCTIONF FROM AXUM

/// GET query parameters
pub async fn transfer_search_handler(Query(params): Query<HashMap<String, String>>) {
    let query = params.get("q");
    println!("query = {:?}", query)
}

#[derive(Debug, serde::Deserialize)]
pub struct TempUserData {}

#[axum::debug_handler]
pub async fn transfer_create_data(Json(payload): Json<TempUserData>) {
    println!("{:?}", payload);
}

// pub async fn multipart_transfer_handler(axum::extract::Multipart(multipart) :  ){
//     println!("{}",multipart)
// }

// Y: ============================================================================ TEST functions

/// ### Create dummy file and test the function:
/// ***function::save_file_on_default_upload_directory()***
pub async fn test_dummy_file_saveing() {
    let test_bytes = b"This is test files".to_vec(); // test
    let test_filename = "test.md".to_string(); // taken as test because you did not told me where ot get this part? 

    match save_file_on_default_upload_directory(test_bytes, &test_filename).await {
        Ok(saved_name) => println!("✅ File Saved: {}", saved_name),
        Err(e) => println!("❌ Faild to save file: {}", e),
    }
}

// Y: ============================================================================ MAIN FUNCTIONS
#[axum::debug_handler]
/// ## to verify the file exists or not on server uploades folder.
/// ``` rust
///  // getting
///  Axum::extract::Path =  Path<String>
///  // create full path with upload dir + path with format!
/// std::fs::read(complete_file_path) // check if its available or not =>
/// ```
///  ---
pub async fn check_file_present_or_not(axum::extract::Path(filename): Path<String>) -> Result<Vec<u8>, axum::http::StatusCode> {
    let complete_file_path = format!("{}/{}", get_upload_dir(), filename);
    std::fs::read(complete_file_path).map_err(|_| StatusCode::NOT_FOUND)
}

///
/// use Multipart => while let some(field / mut field) = multipart.next_field().await.unwrap{...}
/// provides the filed.{headers(), name(), fille_name() bites() }
///
pub async fn post_save_file(mut multipart: axum::extract::Multipart) -> impl axum::response::IntoResponse {
    let mut res = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let filed_headers = field.headers();
        let field_content_types = field.content_type().unwrap_or("");
        let field_name = field.name().unwrap_or("").to_string();
        let file_name = field.file_name().unwrap_or("").to_string();

        // B: The vars above live only till this steatement lives, thats why if we put let bytes above this statement it will throgh error all because of  ".await"
        println!(" headers:{:?}\n content_type:{} \n name:{} \n filename:{}", filed_headers, field_content_types, field_name, file_name);

        let bytes = field.bytes().await.unwrap().to_vec();

        match save_file_on_default_upload_directory(bytes, &file_name).await {
            Ok(filename) => res.push(format!("✅ FILE Saved: {}", filename)),
            Err(e) => {
                res.push(format!("❌ Faild to save file: {}", e));
            }
        }
    }
    (StatusCode::OK, res.join("\n"))
}

pub async fn delete_file(axum::extract::Path(filename): Path<String>) -> Result<(), StatusCode> {
    // delete_file_on_default_upload_directory(&filename)
    let complete_file_path = format!("{}/{}", get_upload_dir(), filename);

    std::fs::remove_file(complete_file_path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => StatusCode::NOT_FOUND,
        std::io::ErrorKind::PermissionDenied => StatusCode::FORBIDDEN,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })
}

pub async fn delete_best_implimentation(axum::extract::Path(filename): Path<String>) -> Result<(), StatusCode> {
    let path: std::path::PathBuf = [get_upload_dir(), filename].iter().collect();

    tokio::fs::remove_file(&path).await.map_err(|e| {
        // Y: Tracing
        // tracing::error!("Faild to delete {}:{}",filename, e);
        match e.kind() {
            std::io::ErrorKind::NotFound => StatusCode::NOT_FOUND,
            std::io::ErrorKind::PermissionDenied => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    })
}

struct FileInfo {
    name: String,
    size: u64,
    modified: String,
    path: String,
}

struct FileListResponse {
    files: Vec<FileInfo>,
    count: usize,
    total_size: u64,
}

pub async fn get_all_files_handler() {
    // return impl IntoResponse
    todo!();
}

//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//

//
//
//
//
//
//
//
//
//
//
//
//

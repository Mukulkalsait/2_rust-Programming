
use axum::{self, extract::{Path}, http::StatusCode};
use crate::handlers::helper_functions::{ delete_file_on_default_upload_directory, get_upload_dir, save_file_on_default_upload_directory};



/// just working or not.
pub async fn workign()-> String{ "✅ WORKING".to_string() }

/// just working or not for homepage.
pub async fn home_page()-> String{ "󰠧 HOME ".to_string() }

/// ### Create dummy file and test the function:
/// ***function::save_file_on_default_upload_directory()***
pub async fn test_dummy_file_save(){
    let test_bytes = b"This is test files".to_vec(); // test
    let test_filename = "test.md".to_string(); // taken as test because you did not told me where ot get this part? 
    
    match save_file_on_default_upload_directory(test_bytes, &test_filename).await{
        Ok(saved_name) => println!("✅ File Saved: {}",saved_name),
        Err(e) => println!("❌ Faild to save file: {}",e),
    }
}

#[axum::debug_handler]
/// ## to verify the file exists or not on server uploades folder.
/// ``` rust
///  // getting 
///  Axum::extract::Path =  Path<String>
///  // create full path with upload dir + path with format!
/// std::fs::read(complete_file_path) // check if its available or not => 
/// ```
///  ---
pub async fn check_if_post_file_present_or_not( axum::extract::Path(filename): Path<String>)-> Result<Vec<u8>,axum::http::StatusCode>{
    let complete_file_path =  format!("{}/{}",get_upload_dir(),filename);
    std::fs::read(complete_file_path).map_err(|_| StatusCode::NOT_FOUND)
}


pub async fn post_upload_handler(mut multipart: axum::extract::Multipart)-> String{

    while let Some(field) =  multipart.next_field().await.unwrap(){
        println!("headers: {:?}",field.headers());
        println!("content_type: {}",field.content_type().unwrap_or(""));
        println!("name: {}",field.name().unwrap_or(""));
        println!("filename: {}",field.file_name().unwrap_or(""));

        let field_name = field.name().unwrap_or("").to_string();
        println!("field_name: {}",field_name);
        let file_name = field.file_name().unwrap_or("").to_string();
        let bytes = field.bytes().await.unwrap().to_vec();

        match save_file_on_default_upload_directory(bytes, &file_name).await{
            Ok(m) =>  {println!("✅ FILE Saved: {}",m)},
            Err(e) => {println!("❌ Faild to save file: {}", e)}
        }
    }   
    "upload endpoint hit".to_string()
}


// pub async fn get_file_handler(Path(filename):Path<String>)->String{
//     match read_file(filename).await{
//         Ok(bites)=>{
//             match String::from_utf8(bites){
//                 Ok(content) => content,
//                 Err(_) => "Binery file not found to display as text".to_string(),
//             }
//             Err(e) =>format!("Error reading file: {}",e),
//
//         },
//
//     }
//
// }
//

pub async fn post_delete_file(mut multipart: axum::extract::Multipart)-> StatusCode{
// delete_file_on_default_upload_directory(&filename)

    let mut filename :&str;
    while let Some(field) = multipart.next_field().await.unwrap() {
            let filedname= field.name().unwrap_or("");
            let filename = field.file_name().unwrap_or("");
            
    }

    if let Some(res) = delete_file_on_default_upload_directory(filename){
        StatusCode::OK
    } else { StatusCode::INTERNAL_SERVER_ERROR }
}

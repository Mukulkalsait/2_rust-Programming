use axum::{extract::Path, http::StatusCode};





/// providing default upload directory into root/upload 
///  => if present return , else create and return
pub fn get_upload_dir()->String {
    if !std::path::Path::new("./uploads").exists() {
        std::fs::create_dir_all("./uploads").unwrap()
    }
    "./uploads".to_string()
}


pub async fn save_file_on_default_upload_directory(bytes: Vec<u8>,orignal_filename: &str)->Result<String, std::io::Error>{

    // geting name of file with path
    let file_entenxion_extrated_from_path = std::path::Path::new(orignal_filename);
    // extension processing
    let extension_extration = file_entenxion_extrated_from_path.extension().and_then(|extension| extension.to_str()).unwrap_or("bin").to_string();

    println!( "Inside ***save_file_on_default_upload_directory*** \norignal_filename: {}\n file_entenxion_extrated_from_path: {:?}\n extension_extration: {} ",orignal_filename,file_entenxion_extrated_from_path, extension_extration);
    // R: debuging

    let uploading_filename = format!("file_{}.{}", chrono::Utc::now().timestamp(),extension_extration); // return this to user...
    let path = format!("{}/{}",get_upload_dir(),uploading_filename);

    std::fs::write(&path, bytes)?;

    Ok(uploading_filename)
}



pub async fn delete_file_on_default_upload_directory(orignal_file_name: &str)-> Result<&str,std::io::Error>{
    let full_path = format!("{}/{}",get_upload_dir(), orignal_file_name);
    println!("Deleting: {}",full_path);
    if std::fs::exists(&full_path).unwrap(){ std::fs::remove_dir(full_path).and_then(Ok(())) }else{ Ok(&full_path)}
}

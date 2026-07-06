/// providing default upload directory into root/upload
///  => if present return , else create and return
pub fn get_upload_dir() -> String {
    if !std::path::Path::new("./uploads").exists() {
        std::fs::create_dir_all("./uploads").unwrap()
    }
    "./uploads".to_string()
}

/// ### PATH => convert name into Path like Object where it konws what is "/" and "." and extensions, everting.
///
/// > it provides :
/// ```
/// exists() - Check if file/directory exists
/// is_file() / is_dir() - Check what it is
/// file_name() - Get filename from path
/// extension() - Get file extension
/// parent() - Get parent directory
/// join() - Join paths together (handles / vs \ automatically)
/// to_str() - Convert to string
///
/// ```
pub async fn save_file_on_default_upload_directory(bytes: Vec<u8>, orignal_filename: &str) -> Result<String, std::io::Error> {
    let path_obj = std::path::Path::new(orignal_filename);
    let extension_extration = path_obj.extension().and_then(|extension| extension.to_str()).unwrap_or("bin").to_string();

    println!(
        "Filename:{:?} \nIs empty ?:{} \nAbsolute path: {:?} \nIs Directory: {} \nIs File: {} \nInside ***save_file_on_default_upload_directory*** \norignal_filename: {}\n file_entenxion_extrated_from_path: {:?}\n extension_extration: {}",
        path_obj.file_name(),
        path_obj.is_empty(),
        path_obj.is_absolute(),
        path_obj.is_dir(),
        path_obj.is_file(),
        orignal_filename,
        path_obj,
        extension_extration
    );

    // R: debuging

    let uploading_filename = format!("file_{}.{}", chrono::Utc::now().timestamp(), extension_extration); // return this to user...
    let save_path = format!("{}/{}", get_upload_dir(), uploading_filename);

    std::fs::write(&save_path, bytes)?;
    Ok(uploading_filename)
}

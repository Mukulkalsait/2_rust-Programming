use std::ffi::OsStr;

use axum::Error;

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

// Struct:
pub struct FileEntry {
    name: String,
    size: u64,
}

// Struct:
pub struct FileResponse {
    all_files: Vec<FileEntry>,
    count: usize,
}

/// >>> Awesome Concept found here
///  1. when we do read_dir() it returns -> Result<ReadDir,Error>               G: 1
///  2. match with Ok/Err return the same things just to get inner value        G: 2
///  3. now we got inner value that is ReadDir which is an ITERATOR             G: 3
///  4. now loop over and we get each_entry with type Result<DirEntry, Error>   G: 4
///  5. Match and get inner value DirEntry                                      G: 5
///  6. NOW DirEntry has its Methods like .path() and return -> PathBuf         G: 6
///  7. NOW PathBof has iss methods like .file_name() -> OsString type name     G: 7
///
///
/// ```
/// // So basically we have:                                                    IMP:
///   Result<ReadDir:Result<DirEntry.Impls, Error>, Error>
/// ```
pub fn list_folder_x(folder_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut all_file_list = Vec::new();

    let directory_read = std::fs::read_dir(folder_path); // Y: 1

    // Y: 2 & 3
    //
    // can be replaced by just :
    // let directory_read_result = directory_read?;
    let directory_read_result = match directory_read {
        Ok(dir_res) => dir_res,
        Err(e) => return Err(e), // direct Error return
    };

    // Y: 4
    for each_entry in directory_read_result {
        // Y: 5
        let entry = match each_entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Faild to read file: {}", e);
                continue;
            }
        };
        // let file_metadata = entry.metadata(); //  Result<Metadata, Error>
        // let file_type = entry.file_type(); // Result<FileType,Error>
        let file_path = entry.path(); // Result<PathBuf, Err>  Y: 6 

        if file_path.is_file() {
            // Ref: 7
            if let Some(file_name) = file_path.file_name() {
                if let Some(file_name_string) = file_name.to_str() {
                    all_file_list.push(file_name_string.to_string());
                }
            }
        }
    } // Loop End:

    all_file_list.sort(); // for consistant resualt
    Ok(all_file_list)
}

/// try to understand the list_folder_x() function its the same but extreamly optimised edmotic version.
pub fn list_all_files_in_folder(folder_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut all_files_in_folder = Vec::new();

    match std::fs::read_dir(folder_path) {
        Ok(directory) => {
            for each_files in directory {
                match each_files {
                    Ok(inner_file) => {
                        let file_path = inner_file.path(); // need filename from PathBuf
                        if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
                            all_files_in_folder.push(file_name.to_string());
                        }
                    }
                    Err(e) => {
                        eprintln!("Faild to read file: {}", e);
                        continue;
                    }
                }
            }
            all_files_in_folder.sort();
            Ok(all_files_in_folder)
        }
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                // if directory not found empty vec return
                Ok(Vec::new())
            }
            _ => Err(e),
        },
    }
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
//
//
//
//
//
//
//
//
//

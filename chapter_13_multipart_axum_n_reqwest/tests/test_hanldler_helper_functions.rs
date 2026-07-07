use chapter_13_multipart_axum_n_reqwest::handlers::helper_functions::*;
use std::{fmt::format, fs};

#[test]
fn test_list_folder_with_non_exciting_path() {
    let res = list_folder_x("./this_folder_does_not_exists");
    assert!(res.is_ok());
}

#[test]
fn test_list_directory_with_empty_directory() {
    let foldername: &str = "./empty_folder";

    fs::create_dir_all(foldername).unwrap(); // cretate dir

    let res = list_folder_x(foldername).unwrap();
    assert!(res.is_empty());

    fs::remove_dir_all(foldername).unwrap(); // remove dir
}

#[test]
fn test_list_all_files_in_folder_universal() {
    let foldername: &str = "./test_folder";
    let filename: &str = "testfile.txt";
    let file_path = format!("{}/{}", foldername, filename);
    let file_content: &str = "hello";
    let subfolder = format!("{}/subfolder", foldername);
    let mut temp_vec = Vec::new();

    fs::create_dir_all(foldername).unwrap(); // create folder

    // SingleFile ====================================================================
    fs::write(&file_path, file_content).unwrap(); // create file

    let res = list_all_files_in_folder(foldername).unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0], filename);
    assert!(!res.contains(&"subfolder".to_string()));
    println!("subfolder check done...");

    println!("singlefile test completed...");

    fs::remove_file(&file_path).unwrap();

    // MultiFiles ====================================================================
    let set_file_no: u16 = 20;

    for files in 0..set_file_no {
        let file_path = format!("{}/a_{}.txt", foldername, files);
        fs::write(&file_path, file_content).unwrap();
        temp_vec.push(file_path);
    }
    let res = list_all_files_in_folder(foldername).unwrap();

    assert_eq!(res.len(), temp_vec.len());
    assert_eq!(res, temp_vec);
    assert!(!res.contains(&"subfolder".to_string()));
    println!("subfolder check done...");
    println!("Multiple files testing completed...");

    for files in 0..set_file_no {
        let file_path = format!("{}/a_{}.txt", foldername, files);
        temp_vec.remove(files.into());
        fs::remove_file(file_path).unwrap();
    }
    // what if i want to do the above loop but in temp_vec.iter() form
    // let Some(x) = temp_vec.iter() { ... } is it possible?

    // MultiEntensions ====================================================================
    let extensions = vec!["mp3", "flac", "alac", "wav", "m4a", "dsd"];

    for file_extension in extensions {
        let file_path = format!("{}/test.{}", foldername, file_extension);
        fs::write(&file_path, file_content).unwrap();
        temp_vec.push(file_path);
    }
    let res = list_all_files_in_folder(foldername).unwrap();
    assert_eq!(res.len(), temp_vec.len());

    for i in &temp_vec {
        assert!(res.contains(i));
        fs::remove_file(i).unwrap();
    }
    temp_vec.clear();

    // HiddenFiles ====================================================================
    let set_file_no: u16 = 4;

    for i in 0..set_file_no {
        let file_path = format!("{}/a_{}.txt", foldername, i);
        let hidden_file_path = format!("{}/.a_{}.txt", foldername, i);

        fs::write(file_path, file_content).unwrap();
        fs::write(hidden_file_path, file_content).unwrap();
    }

    fs::remove_dir_all(foldername).unwrap();
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

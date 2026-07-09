use chapter_13_multipart_axum_n_reqwest::handlers::helper_functions::*;
use pretty_assertions::assert_eq;
use std::fs;

const TEST_FOLDER: &str = "./test_folder";

/// ```
/// use once_cell::sync::Lazy;
/// static TEST_FOLDER: Lazy<String> = Lazy::new(|| { "./test_folder".to_string() });
// ```
fn static_variable() {
    println!(
        "
    STATIC VARIAES:
|===================================================================|
| What it does:                                                     |
|     Creates a lazy initialized static variable                    |
|     The value is only created once, the first time it's accessed  |
|     After that, it's cached and reused                            |
| When to use:                                                      |
|     When you need a static variable that requires computation     |
|     When you want to avoid const limitations                      |
|     When you need heap-allocated data (like String)               |
|===================================================================|

    Examples:
|--------------------------------------------------------------------------------------
| // ❌ const can only be compile-time constants
| const FOLDER: &str = \"./test_folder\";  // Works for simple strings
|
| // ❌ static requires compile-time evaluation
| static FOLDER: String = String::new();  // ❌ Can't create String at compile time
|
| // ✅ Lazy lets you create String at runtime, but only once
| static FOLDER: Lazy<String> = Lazy::new(|| \"./test_folder\".to_string());
|--------------------------------------------------------------------------------------
| "
    ); // IMP: Learn to use this instead of Const when needed.
}

// HELPER FUNCTIONS ========================================================================================================

fn setup_test_folder() -> String {
    fs::create_dir_all(TEST_FOLDER).unwrap();
    TEST_FOLDER.to_string()
}

fn clean_test_folder(foldername: &str) {
    std::fs::remove_dir_all(foldername).unwrap_or_else(|e| eprint!("Faild to temp+folder {}: {}", foldername, e))
}

fn create_file(foldername: &str, filename_a: &str, extension: &str, file_content: &str) -> Result<String, std::io::Error> {
    let filepath = format!("{}/{}.{}", foldername, filename_a, extension);
    fs::write(&filepath, file_content)?;
    Ok(filepath)
}

fn multi_file_creater(foldername: &str, count: u16, prefix: &str, extension: &str) -> Vec<String> {
    let mut all_files = Vec::new();
    for i in 0..count {
        let filename = format!("{}_{}", prefix, i);
        let demotext: &str = "Hello this is test text";
        if let Ok(file) = create_file(foldername, filename.as_str(), extension, demotext) {
            all_files.push(file);
        } else {
            eprintln!("Error creating file: {}/{}.{}", foldername, filename, extension);
        }
    }
    all_files
}

// Tests FUNCTIONS ========================================================================================================

#[test]
fn test_single_file() {
    let folderpath = setup_test_folder();
    let filepath = create_file(folderpath.as_str(), "testfile.md", "md", "# heading 1").unwrap();

    let res = list_all_files_in_folder(folderpath.as_str()).unwrap();
    assert!(res.contains(&String::from("testfile.md")));

    fs::remove_file(filepath).unwrap();
    clean_test_folder(&folderpath);
    static_variable();
}

#[test]
fn test_multiple_files() {
    let folderpath = setup_test_folder();
    let count: u16 = 20;
    let all_files = multi_file_creater(&folderpath, count, "a", ".md");

    let res = list_all_files_in_folder(&folderpath).unwrap();
    assert_eq!(res.len(), all_files.len());

    for i in res.iter() {
        assert!(all_files.contains(i));
        let _x = fs::remove_file(i).map_err(|e| eprintln!("Faild to delete file : {}", e));
    }
    clean_test_folder(folderpath.as_str());
}

#[test]
fn test_hidden_files() {
    let folderpath = setup_test_folder();
    let count: u16 = 5;
    let mut all_files = multi_file_creater(folderpath.as_str(), count, "visible_files", "txt");
    let mut hidden_files = multi_file_creater(folderpath.as_str(), count, ".hidden_files", "txt");

    all_files.append(&mut hidden_files); // move everyting from hidden files to all files

    let res = list_all_files_in_folder(folderpath.as_str()).unwrap();
    assert_eq!(res.len(), all_files.len());

    for i in res.iter() {
        assert!(all_files.contains(i));
        let _x = fs::remove_file(i).map_err(|e| eprintln!("Faild to remove file: {}", e));
    }
    clean_test_folder(folderpath.as_str());
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

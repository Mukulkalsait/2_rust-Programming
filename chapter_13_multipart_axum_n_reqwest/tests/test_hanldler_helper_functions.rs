use chapter_13_multipart_axum_n_reqwest::handlers::helper_functions::*;
use pretty_assertions::assert_eq;
use std::fs;
use tempfile::tempdir;
use test_case::test_case;

/// ```
/// use once_cell::sync::Lazy;
/// static TEST_FOLDER: Lazy<String> = Lazy::new(|| { "./test_folder".to_string() });
// ```
fn _static_variable() {
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

fn _clean_test_folder(foldername: &str) {
    std::fs::remove_dir_all(foldername).unwrap_or_else(|e| eprint!("Faild to temp+folder {}: {}", foldername, e))
}

fn create_file(foldername: &str, filename: &str, file_content: &str) -> Result<String, std::io::Error> {
    let filepath = format!("{}/{}", foldername, filename);
    fs::write(&filepath, file_content)?;
    Ok(filepath.to_string())
}

fn delete_file(filepath: &str) -> Result<(), std::io::Error> {
    fs::remove_file(filepath)?;
    Ok(())
}

enum MfcdTask {
    Add,
    Remove,
}

fn multi_file_creater_and_deleter(foldername: &str, count: u16, prefix: &str, extension: &str, task: MfcdTask) -> Vec<String> {
    let mut all_files = Vec::new();
    for i in 0..count {
        let filename = format!("{}_{}.{}", prefix, i, extension);
        let filepath = format!("{}/{}", foldername, filename);
        let demotext: &str = "Hello this is test text";
        let errormessage = "----------------------Error creating file: ";
        match task {
            MfcdTask::Add => {
                if let Err(e) = fs::write(&filepath, demotext) {
                    dbg!(&filepath);
                    eprintln!("{}:{}", errormessage, e);
                } else {
                    dbg!(&filename);
                    all_files.push(filename);
                }
                // if let Ok(file) = create_file(foldername, filename.as_str(), demotext) { all_files.push(file); } else { eprintln!("{}{}/{}.{}", errormessage, foldername, filename, extension); } R: Orignal i wrote
            }
            MfcdTask::Remove => {
                if let Err(e) = fs::remove_file(&filepath) {
                    dbg!(&filepath);
                    eprintln!("{}:{}", errormessage, e);
                } else {
                    dbg!(&filename);
                    all_files.push(filename);
                }
            }
        }
    }
    all_files
}

// Tests FUNCTIONS ========================================================================================================

#[test_case("testfile1.md")]
#[test_case("testfile2.txt")]
fn test_single_file(filename: &str) {
    let tempdir = tempdir().unwrap();
    let folderpath = tempdir.path().to_str().unwrap().to_string();
    let filepath = create_file(&folderpath, filename, "# heading 1").unwrap();
    dbg!(&filepath);

    let res = list_all_files_in_folder(&folderpath).unwrap();
    dbg!(&res);
    assert_eq!(res.len(), 1);
    assert!(res.contains(&filename.to_string()));

    delete_file(&filepath).unwrap();
}

#[test_case(5, "md")]
#[test_case(10, "txt")]
fn test_multiple_files(count: u16, extension: &str) {
    let tempdir = tempdir().unwrap();
    let folderpath = tempdir.path().to_str().unwrap().to_string();
    let expected_files = multi_file_creater_and_deleter(&folderpath, count, "a", extension, MfcdTask::Add);

    let res = list_all_files_in_folder(&folderpath).unwrap();
    dbg!(&res);

    assert_eq!(res.len(), expected_files.len());
    assert_eq!(res, expected_files); // check sorted order of files.

    multi_file_creater_and_deleter(&folderpath, count, "a", extension, MfcdTask::Remove);
}

#[test_case(4)]
#[test_case(10)]
fn test_hidden_files(count: u16) {
    let tempdir = tempdir().unwrap();
    let folderpath = tempdir.path().to_str().unwrap().to_string();
    let visible_files = multi_file_creater_and_deleter(&folderpath, count, "visible_files", "txt", MfcdTask::Add);
    let hidden_files = multi_file_creater_and_deleter(&folderpath, count, ".hidden_files", "txt", MfcdTask::Add);

    // cloning because we need both visible and hiddenfiles later.
    let mut expected_files = visible_files.clone();
    expected_files.extend(hidden_files.clone());
    expected_files.sort();

    let res = list_all_files_in_folder(&folderpath).unwrap();
    dbg!(&res);
    assert_eq!(res.len(), expected_files.len());

    multi_file_creater_and_deleter(&folderpath, count, "visible_files", "txt", MfcdTask::Remove);
    multi_file_creater_and_deleter(&folderpath, count, ".hidden_files", "txt", MfcdTask::Remove);
}

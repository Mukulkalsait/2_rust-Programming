use chapter_13_multipart_axum_n_reqwest::handlers::helper_functions::*;
use pretty_assertions::assert_eq;
use std::fs;
// This function takes a closure and runs it with a test folder
fn with_test_folder<F>(test_code: F)
where
    F: FnOnce(&str), // F is a function that takes &str and returns nothing
{
    let folder = "./test_folder";
    fs::create_dir_all(folder).unwrap();

    // Call your test code with the folder
    test_code(folder);

    // Clean up
    fs::remove_dir_all(folder).unwrap();
}

#[test]
/// > Using it:
/// ### See the flow?
///    * with_test_folder creates the folder
///    * Your closure runs with the folder path
///    * with_test_folder cleans up
///
/// ### Now you understand closures in tests! Later you can use this pattern in real code for things like:
///    * Database transactions (open → run code → commit/rollback)
///    * File operations (open → process → close)
///    * Resource management (acquire → use → release)
fn test_with_runner() {
    with_test_folder(|folder| {
        // This is your test code
        let filepath = format!("{}/{}", folder, "test.txt");
        fs::write(&filepath, "hello").unwrap();

        let result = list_all_files_in_folder(folder).unwrap();
        assert_eq!(result, vec!["test.txt".to_string()]);
    });
    // Cleanup happens automatically!
}

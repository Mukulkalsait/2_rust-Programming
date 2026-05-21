fn unrecoverable_error_triggring() {
    panic!("🧨🧨 crash and burn 🧨🧨");
}

fn unrecoverable_error_auto_treggring() -> Result<i32, String> {
    let v = vec![1, 2, 3];
    // Unrecoverable error cause by the billow line
    let x = Ok(v[99]);
    x
}

/// R: unrecoverable errors are not to be handled with anythin
pub fn run_unrecoverable_last_code() {
    unrecoverable_error_triggring();
    match unrecoverable_error_auto_treggring() {
        Ok(res) => {
            println!("Successed: {}", res)
        }
        Err(e) => {
            println!("Error: {:?}", e)
        }
    }
}

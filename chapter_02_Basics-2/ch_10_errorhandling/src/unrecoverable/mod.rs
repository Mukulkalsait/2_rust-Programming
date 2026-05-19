pub fn unrecoverable_error_triggring() {
    panic!("🧨🧨 crash and burn 🧨🧨");
}

pub fn unrecoverable_error_auto_treggring() -> Result<i32, String> {
    let v = vec![1, 2, 3];
    // Unrecoverable error cause by the billow line
    let x = Ok(v[99]);
    x
}

fn main() {
    linux_only_code();
    #[cfg(target_os = "linux")]
    fn linux_only_code() {
        println!("Running on Linux!");
    }
}

```bash

❯ cargo run
   Compiling chapter_5_reqwest v0.1.0 (/home/mukuldk/1_file/3_Repos_All/A_gitHub/002_Rust_Programing/chapter_5_reqwest)
warning: field `token` is never read
  --> src/models.rs:13:9
   |
12 | pub struct LoginResponse {
   |            ------------- field in this struct
13 |     pub token: Option<String>,
   |         ^^^^^
   |
   = note: `LoginResponse` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `results` is never read
  --> src/models.rs:18:9
   |
17 | pub struct SearchResponse {
   |            -------------- field in this struct
18 |     pub results: Vec<String>,
   |         ^^^^^^^
   |
   = note: `SearchResponse` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

warning: field `status` is never read
  --> src/models.rs:23:9
   |
22 | pub struct UploadResponse {
   |            -------------- field in this struct
23 |     pub status: String,
   |         ^^^^^^
   |
   = note: `UploadResponse` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

warning: `chapter_5_reqwest` (bin "chapter_5_reqwest") generated 3 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `/home/mukuldk/.cargo/target/debug/chapter_5_reqwest`
Login:
LoginResponse {
    token: Some(
        "jwt_12345",
    ),
}
Search:
SearchResponse {
    results: [
        "Rust",
        "Reqwest",
    ],
}
Upload:
UploadResponse {
    status: "uploaded",
}

  mukuldk …/002_Rust_Programing/chapter_5_reqwest   main ✘!?⇡   v1.94.1  ♥ 00:43  
❯                      

```

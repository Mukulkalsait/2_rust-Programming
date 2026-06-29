I love this approach! You're now in control, building systematically, and understanding everything. Let's build a complete file management system step by step.

## Complete File Management Functions - In Order

### Phase 1: Basic File Operations (Foundation)

1. **`save_file`** - Save bytes to disk with unique name
2. **`read_file`** - Read file contents from disk
3. **`delete_file`** - Delete file from disk
4. **`check_file_exists`** - Check if file exists at path

### Phase 2: File Metadata & Info

5. **`get_file_info`** - Get file size, modified date, type
6. **`list_files_in_directory`** - List all files in a directory
7. **`get_file_extension`** - Extract extension from filename
8. **`get_file_size`** - Get size of file in human-readable format

### Phase 3: File Validation

9. **`validate_file_type`** - Check if file type is allowed (jpg, png, etc.)
10. **`validate_file_size`** - Check if file size is within limits
11. **`sanitize_filename`** - Remove dangerous characters from filename

### Phase 4: Advanced Operations

12. **`move_file`** - Move file to different location
13. **`copy_file`** - Copy file to different location
14. **`rename_file`** - Rename file
15. **`get_file_hash`** - Calculate file hash (for deduplication)

### Phase 5: Directory Operations

16. **`create_directory`** - Create directory recursively
17. **`remove_directory`** - Remove directory and its contents
18. **`get_directory_size`** - Get total size of directory

---

## Let's Start With Function #1: `save_file`

### What It Should Do

- Accept: `bytes: Vec<u8>`, `base_filename: &str`
- Generate unique filename with timestamp + UUID
- Save to `./uploads/` directory
- Return: `Result<String, ApiError>` (the saved filename)

---

### Hints for `save_file`

**Step 1: Create uploads directory if it doesn't exist**

```rust
use std::fs;

// Check if directory exists, create if not
if !std::path::Path::new("./uploads").exists() {
    fs::create_dir_all("./uploads")?;
}
```

**Step 2: Get current timestamp**

```rust
use chrono::Utc;

let timestamp = Utc::now().timestamp();
```

**Step 3: Get file extension from base_filename**

```rust
use std::path::Path;

let extension = Path::new(base_filename)
    .extension()
    .and_then(|ext| ext.to_str())
    .unwrap_or("bin")
    .to_string();
```

**Step 4: Generate unique filename**

```rust
let filename = format!("file_{}_{}.{}", timestamp, uuid::Uuid::new_v4(), extension);
```

**Step 5: Save bytes to disk**

```rust
let full_path = format!("./uploads/{}", filename);
fs::write(&full_path, bytes)?;
```

**Step 6: Return the filename**

```rust
Ok(filename)
```

---

### Your Task

1. **Create a new file** `src/file_utils.rs` (or add to existing)
2. **Write the `save_file` function** with these steps
3. **Add the necessary imports** at top
4. **Add `uuid` dependency** to Cargo.toml:
   ```toml
   uuid = { version = "1.0", features = ["v4"] }
   ```

---

### What You Need to Add

**Dependencies:**
```toml
[dependencies]
axum = "0.8.9"
tokio = { version = "1.52.3", features = ["full"] }
tower = "0.5.3"
chrono = "0.4"
uuid = { version = "1.0", features = ["v4"] }
```

**Imports needed for the function:**
```rust
use std::fs;
use std::path::Path;
use chrono::Utc;
use uuid::Uuid;
```

---

### Testing Your Function

After you write it, modify your upload handler to use it:

```rust
// Replace the current save logic with:
let saved_filename = save_file(bytes, file_name).await?;
println!("File saved as: {}", saved_filename);
```

---

### What Success Looks Like

When you test, you should see:
- Filename like: `file_1749283745_550e8400-e29b-41d4-a716-446655440000.txt`
- Files in `./uploads/` with unique names
- Same content as the uploaded file

---

## Before You Write

**Tell me:**
1. Are you going to create a separate module/file for these functions?
2. Do you understand what each step of the `save_file` function does?
3. What part of the `save_file` function seems unclear?

**Write the function. If you get stuck, show me what you tried and where you're stuck.**

After we complete all functions one by one, you'll have a complete file management library that you built yourself!

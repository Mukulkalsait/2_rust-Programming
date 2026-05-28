Got it. You want a **clear component breakdown** – what each piece is called, where it lives, what logic goes inside, and why. No fluff.

Here’s your architecture glossary for a typical Rust backend (Axum/Actix + SQLx + JWT):

---

## 1. Handler (aka Controller)
**File:** `handlers/auth.rs`, `handlers/user.rs`  
**What it does:** Receives HTTP request, extracts data, calls services, returns HTTP response.  
**What goes in:**  
- Extracting JSON / path / query params  
- Calling service methods  
- Mapping service errors to HTTP status codes  
- Returning `impl IntoResponse`  

**What does NOT go in:**  
- Database queries  
- Business logic (e.g., “is this user allowed to post?”)  
- JWT generation (that's a service)

**Example logic:**  
```rust
pub async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> Response {
    match auth_service::login(&state.db, &payload).await {
        Ok(token) => (StatusCode::OK, Json(token)).into_response(),
        Err(AuthError::WrongPassword) => (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Oops").into_response(),
    }
}
```

---

## 2. Service (aka Business Logic)
**File:** `services/auth_service.rs`, `services/user_service.rs`  
**What it does:** Contains domain rules, orchestrates repositories, calls external APIs, generates JWTs, hashes passwords, etc.  
**What goes in:**  
- Password verification  
- JWT creation / validation  
- Checking if email is already taken  
- Transaction logic (multiple repo calls in one unit)  
- Calling third-party services (Google OAuth, email sender)

**What does NOT go in:**  
- HTTP status codes  
- Request extraction  

**Example:**  
```rust
pub async fn login(db: &PgPool, creds: &LoginRequest) -> Result<String, AuthError> {
    let user = user_repo::find_by_email(db, &creds.email).await?.ok_or(AuthError::UserNotFound)?;
    verify_password(&creds.password, &user.hash)?;
    let token = encode_jwt(user.id)?;
    Ok(token)
}
```

---

## 3. Repository (aka Data Access Layer)
**File:** `repositories/user_repo.rs`  
**What it does:** Raw database operations – select, insert, update, delete.  
**What goes in:**  
- SQLx queries (`sqlx::query!`)  
- Mapping database rows to structs  
- Returning `Option` or `Result` with DB errors  

**What does NOT go in:**  
- Business logic  
- HTTP concerns  

**Example:**  
```rust
pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!("SELECT id, email, password_hash FROM users WHERE email = $1", email)
        .fetch_optional(pool)
        .await
}
```

---

## 4. Middleware
**File:** `middleware/auth.rs` (or defined in `main.rs` with `tower`)  
**What it does:** Wraps handlers to run before/after request – logging, auth, CORS, compression.  
**What goes in:**  
- Extracting JWT from `Authorization` header  
- Validating token and inserting user ID into request extensions  
- Rate limiting  
- Request ID generation  

**Example (Auth middleware):**  
Checks token → if valid, adds `user_id` to request → passes to handler. If invalid, returns 401 directly.

---

## 5. AppState (Shared State)
**File:** Usually `main.rs` or `state.rs`  
**What it does:** Holds things handlers need – DB pool, Redis client, JWT secret, config.  
**Example:**  
```rust
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub redis_client: RedisClient,
}
```
Passed to handlers via `State<AppState>` in Axum.

---

## 6. Error Type (Custom)
**File:** `error.rs` or `errors/mod.rs`  
**What it does:** Defines all possible errors in your domain. Uses `thiserror`.  
**Why:** Allows `?` propagation across layers, then map to HTTP at handler.

**Example:**  
```rust
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("User not found")]
    NotFound,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("JWT error")]
    Jwt(#[from] jsonwebtoken::errors::Error),
}
```

---

## Where does Google OAuth go?
- **Handler** – receives callback from Google  
- **Service** – exchanges code for token, fetches user info from Google API  
- **Repository** – stores/retrieves user from DB by Google ID  
- **Middleware** – optionally protects routes after login  

---

## Summary table – where does X go?

| Logic | Where |
|-------|-------|
| Parse JSON from request | Handler |
| Validate email format | Service |
| Query user by ID | Repository |
| Generate JWT | Service |
| Check if user is admin | Middleware or Service (depends) |
| Log request URL | Middleware |
| Map `sqlx::Error` to 500 | Handler (via error type) |
| Create DB connection pool | `main.rs` → AppState |
| Send welcome email after signup | Service (call email module) |

---

## Your next step
Pick **one component** (say, handlers) and refactor your current project to separate handler vs service vs repo. You’ll see the pattern instantly.

// FILE: /src/db/db_macros.rs

/// #### user_fields is a macro which expnda as compiletime, and we used it to expands.
/// ```text
/// "id,name,email,password,verified,created_at,updated_at,verification_token,token_expire_at,role as \"role:UserRole\" "
///  👆🏻 this line
/// ```
#[macro_export]
macro_rules! user_fields {
    () => {
        "id,name,email,password,verified,created_at,updated_at,verification_token,token_expire_at,role as \"role:UserRole\" "
    };
}

/// #### This macro takes $column:expr expression, and return full
/// ```text
/// "SELECT {} FROM users WHERE {} = $1", user_fields!(), $column
/// ```
/// > the [user_fields!()] is another costume macro which is used witin this marco
#[macro_export]
macro_rules! select_user_by {
    ($column:expr) => {
        &format!("SELECT {} FROM users WHERE {} = $1", user_fields!(), $column)
    };
}

#[macro_export]
macro_rules! user_fields_no_role {
    () => {
        "id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at"
    };
}

// Pre-built queries
#[macro_export]
macro_rules! select_all_users {
    () => {
        &format!("SELECT {} FROM users", user_fields!())
    };
}

#[macro_export]
macro_rules! update_user_returning {
    ($set_clause:expr) => {
        &format!("UPDATE users SET {} WHERE id = $1 RETURNING {}", $set_clause, user_fields!())
    };
}

///====================================================|
/// THIS MACROS MIGHT BE HELPFULL TO US IF WE DO THIS  |  Y: =>
///====================================================|
/// ```rust
/// async fn get_user(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
///     sqlx::query_as!(
///         User,
///         select_user_by!("id"), // Clean! G:  the use of macro
///         user_id
///     )
///     .fetch_optional(&self.pool)
///     .await
/// }
/// ```
///## DX: but the query_as!() macro checks db data at typetime, while our macro return data while compiletime. mean they will not work together at;
///####  B: SOLUTIONS:
///     - use sqlx::query_as() function and not the macro.
pub fn use_of_macros() {
    println!("Check the documentations");
}

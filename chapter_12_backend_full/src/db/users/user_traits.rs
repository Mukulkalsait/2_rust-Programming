// FILE: /src/db/users/user_traits.rs

use crate::db::users::user_modle::{User, UserRole};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[async_trait]
pub trait UserExt {
    /// get user funcation
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
        token: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error>;

    /// get all user_id
    async fn get_all_users(&self, page: u32, limit: usize) -> Result<Vec<User>, sqlx::Error>;

    /// Save User
    async fn save_user(
        &self,
        name: Option<&str>,
        email: Option<&str>,
        password: Option<&str>,
        verification_token: Option<&str>,
        token_expire_at: Option<DateTime<Utc>>,
    ) -> Result<User, sqlx::Error>;

    /// Save Users Generic Function
    async fn save_generic_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
        verification_token: T,
        token_expire_at: Option<DateTime<Utc>>,
    ) -> Result<User, sqlx::Error>;

    /// Get total Users Count
    async fn get_user_count(&self) -> Result<u64, sqlx::Error>;

    /// Update User Info (Name only)
    async fn update_user_name<T: Into<String> + Send>(&self, user_id: Uuid, name: T) -> Result<User, sqlx::Error>;

    /// Update User Role
    async fn update_user_role(&self, user_id: Uuid, role: UserRole) -> Result<User, sqlx::Error>;

    /// User Password Update
    async fn update_user_pass<T: Into<String> + Send>(&self, user_id: Uuid, password: T) -> Result<User, sqlx::Error>;

    /// Verify TOken statu
    async fn verify_token(&self, user_id: Uuid, token: Option<&str>) -> Result<(), sqlx::Error>;

    /// Token Update
    async fn update_token_status(
        &self,
        user_id: Uuid,
        token: Option<&str>,
        token_expires_at: Option<DateTime<Utc>>,
    ) -> Result<(), sqlx::Error>;
}

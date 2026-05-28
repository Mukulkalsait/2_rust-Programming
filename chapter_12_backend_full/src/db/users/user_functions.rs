// FILE: /src/db/users/user_functions.rs

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::db::connection::DBClient;
use crate::db::users::user_modle::{User, UserRole};
use crate::db::users::user_traits::UserExt;

#[async_trait]
impl UserExt for DBClient {
    // Y: GET USER ehtier from(user_id, name, email, verification_token) any of them...
    //  created user with Option<user> =  None.
    //  and returned.
    //
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
        verification_token: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error> {
        let mut user: Option<User> = None;
        if let Some(user_id) = user_id {
            user = sqlx::query_as!(
                User,
                r#"
                SELECT id,name,email,password,verified,created_at,updated_at,verification_token,token_expire_at,role as "role:UserRole"
                FROM users WHERE id = $1
                "#,
                user_id
            )
            .fetch_optional(&self.pool)
            .await?;
        }
        if let Some(email) = email {
            user = sqlx::query_as!(
                User,
                r#"
                SELECT id,name,email,password,verified,created_at,updated_at,verification_token,token_expire_at,role as "role:UserRole"
                FROM users WHERE email = $1
                "#,
                email
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(name) = name {
            user = sqlx::query_as!(
                User,
                r#"
                SELECT id,name,email,password,verified,created_at,updated_at,verification_token,token_expire_at,role as "role:UserRole"
                FROM users WHERE name= $1"#,
                name
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(verification_token) = verification_token {
            user = sqlx::query_as!(
                User,
                r#"
                SELECT id,name,email,password,verified,created_at,updated_at,verification_token,token_expire_at,role as "role:UserRole"
                FROM users WHERE verification_token = $1"#,
                verification_token
            )
            .fetch_optional(&self.pool)
            .await?;
        }
        Ok(user)
    }

    /// get all Userr.
    /// created offset and all variable to use in query_as micro  
    /// used Ok to setup the proper type
    async fn get_all_users(&self, page: u32, limit: usize) -> Result<Vec<User>, sqlx::Error> {
        let offset_list = (page - 1) * limit as u32;
        let users = sqlx::query_as!(
            User,
            r#"
                SELECT id,name,email,password,verified,created_at,updated_at,verification_token,token_expire_at,role as "role:UserRole"
                FROM users
                ORDER BY created_at DESC LIMIT $1 OFFSET $2
            "#,
            limit as i64,
            offset_list as i64,
        )
        .fetch_all(&self.pool)
        .await?; // ?allow sqlx to handle error
        Ok(users)
    }

    async fn save_user(
        &self,
        name: Option<&str>,
        email: Option<&str>,
        password: Option<&str>,
        verification_token: Option<&str>,
        token_expire_at: Option<DateTime<Utc>>,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, email, password, verification_token, token_expire_at)
            VALUES ($1,$2,$3,$4,$5)
            RETURNING id, name,email, password, verified, created_at, updated_at, verification_token, token_expire_at, role as "role:UserRole"
            "#,
            name,
            email,
            password,
            verification_token,
            token_expire_at
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    /// Save Users Generic Function
    async fn save_generic_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
        verification_token: T,
        token_expire_at: Option<DateTime<Utc>>,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name,email,password,verification_token,token_expire_at)
            VALUES ($1,$2,$3,$4,$5)
            RETURNING id,name,email,password,verified,created_at,updated_at,verification_token,token_expire_at,role as "role:UserRole"
            "#,
            name.into(),
            email.into(),
            password.into(),
            verification_token.into(),
            token_expire_at
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    /// Get total Users Count
    async fn get_user_count(&self) -> Result<u64, sqlx::Error> {
        let count = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM users"#).fetch_one(&self.pool).await.unwrap();
        Ok(count.unwrap_or(0) as u64) // edge case not found hence unwrap_or(0)
    }

    async fn update_user_name<T: Into<String> + Send>(&self, user_id: Uuid, name: T) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET name= $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id,name,email,password,verified,created_at,updated_at,verification_token,token_expire_at,role as "role:UserRole"
            "#,
            name.into(),
            user_id,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    /// Update User Role
    async fn update_user_role(&self, user_id: Uuid, role: UserRole) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
        UPDATE users
        SET role= $1, updated_at = NOW()
        WHERE id = $2
        RETURNING id,name,email,password,verified,created_at,updated_at,verification_token,token_expire_at,role as "role:UserRole"
        "#,
            role as UserRole,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
    /// User Password Update
    async fn update_user_pass<T: Into<String> + Send>(&self, user_id: Uuid, password: T) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
        UPDATE users
        SET password= $1, updated_at = NOW()
        WHERE id = $2
        RETURNING id, name, email, password, verified, created_at, updated_at, verification_token, token_expire_at, role as "role:UserRole"
        "#,
            password.into(),
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }
    /// Verify TOken statu
    async fn verify_token(&self, user_id: Uuid, token: Option<&str>) -> Result<(), sqlx::Error> {
        let _ = sqlx::query!(
            r#"
            UPDATE users
            SET verified = true,
                updated_at = NOW(),
                verification_token = NULL,
                token_expire_at = NULL
            WHERE verification_token = $1
            "#,
            token,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_token_status(
        &self,
        user_id: Uuid,
        token: Option<&str>,
        token_expire_at: Option<DateTime<Utc>>,
    ) -> Result<(), sqlx::Error> {
        let _ = sqlx::query!(
            r#"
        UPDATE users
        SET verification_token = $1,
            token_expire_at = $2,
            updated_at = NOW()
        WHERE id = $3
        "#,
            token,
            token_expire_at,
            user_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

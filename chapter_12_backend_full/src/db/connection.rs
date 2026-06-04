// FILE: /src/db/connections.rs

use sqlx::{postgres, Pool, Postgres};

#[derive(Debug, Clone)]
pub struct DBClient {
    pub pool: Pool<sqlx::Postgres>,
}

impl DBClient {
    pub fn new(pool: Pool<sqlx::Postgres>) -> Self { DBClient { pool } }
}

// FILE: ./src/config/env.rs

use axum::response::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfigEnv {
    pub app_env: String,
    pub app_port: u16,
    pub debug_mode: bool,

    pub server_addr: String,
    pub database_url: String,
    pub max_connection: u32,

    pub jwt_secret: String,
    pub jwt_max_age: i64,

    pub smtp_server: String,
    pub smtp_port: String,
    pub smtp_username: String,
    pub smtp_pass: String,
    pub smtp_from_address: String,
}

impl AppConfigEnv {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder().add_source(config::Environment::default()).build()?.try_deserialize()
    }
}

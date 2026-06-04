// FILE: ./src/config/env.rs

use serde::Deserialize;
use std::env::var;

use crate::errors::SelfConfigError;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfigEnv {
    pub app_env: String,
    pub app_port: u16,
    pub debug_mode: bool,

    pub server_addr: String,
    pub database_url: String,
    pub max_connection: u32,

    pub jwt_secret: String,
    pub jwt_maxage: i64,

    pub smtp_server: String,
    pub smtp_port: String,
    pub smtp_username: String,
    pub smtp_pass: String,
    pub smtp_from_address: String,
}

impl AppConfigEnv {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let app_env = var("APP_ENV").map_err(|_| SelfConfigError::MissingEnvVar { var: "APP_ENV".to_string() })?;
        // let app_port = var("APP_PORT").map_err(|_| SelfConfigError::MissingEnvVar { var: "APP_PORT".to_string() })?;
        let app_port_raw = var("APP_PORT").map_err(|_| SelfConfigError::MissingEnvVar { var: "APP_PORT".to_string() })?;
        let app_port =
            app_port_raw.parse().map_err(|_| SelfConfigError::InvalidValue { key: "APP_PORT".to_string(), val: "must be a valid number".to_string() })?;

        let debug_mode_raw = var("DEBUG_MODE").map_err(|_| SelfConfigError::MissingEnvVar { var: "DEBUG_MODE".to_string() })?;
        let debug_mode = debug_mode_raw
            .parse()
            .map_err(|_| SelfConfigError::InvalidValue { key: "APP_PORT".to_string(), val: "Must be a value number".to_string() })?;

        let server_addr = var("SERVER_ADDR").map_err(|_| SelfConfigError::MissingEnvVar { var: "SERVER_ADDR".to_string() })?;
        let database_url = var("DATABASE_URL").map_err(|_| SelfConfigError::MissingEnvVar { var: "DATABASE_URL".to_string() })?;

        let max_connection_raw = var("MAX_CONNECTION").map_err(|_| SelfConfigError::MissingEnvVar { var: "MAX_CONNECTION".to_string() })?;
        let max_connection = max_connection_raw
            .parse()
            .map_err(|_| SelfConfigError::InvalidValue { key: "Max Connections".to_string(), val: "Must be a number".to_string() })?;

        let jwt_secret = var("JWT_SECRET").map_err(|_| SelfConfigError::MissingEnvVar { var: "JWT_SECRET".to_string() })?;
        let jwt_maxage_raw = var("JWT_MAXAGE").map_err(|_| SelfConfigError::MissingEnvVar { var: "JWT_MAXAGEY".to_string() })?;
        let jwt_maxage = jwt_maxage_raw
            .parse()
            .map_err(|_| SelfConfigError::InvalidValue { key: "Jwt Maxage Kai Kam Ahte Vichara Tari".to_string(), val: "badbad".to_string() })?;

        let smtp_server = var("SMTP_SERVER").map_err(|_| SelfConfigError::MissingEnvVar { var: "SMTP_SERVER".to_string() })?;
        let smtp_port = var("SMTP_PORT").map_err(|_| SelfConfigError::MissingEnvVar { var: "SMTP_PORT".to_string() })?;
        let smtp_username = var("SMTP_USERNAME").map_err(|_| SelfConfigError::MissingEnvVar { var: "SMTP_USERNAME".to_string() })?;
        let smtp_pass = var("SMTP_PASS").map_err(|_| SelfConfigError::MissingEnvVar { var: "SMTP_PASS".to_string() })?;
        let smtp_from_address = var("SMTP_FROM_ADDRESS").map_err(|_| SelfConfigError::MissingEnvVar { var: "SMTP_FROM_ADDRESS".to_string() })?;

        Ok(Self {
            app_env,
            app_port,
            debug_mode,
            server_addr,
            database_url,
            max_connection,
            jwt_secret,
            jwt_maxage,
            smtp_server,
            smtp_port,
            smtp_username,
            smtp_pass,
            smtp_from_address,
        })
    }
}

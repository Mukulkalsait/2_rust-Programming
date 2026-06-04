// FILE: src/config/conf.rs

use sqlx::postgres::PgPoolOptions;

use crate::{config::AppConfigEnv, db::DBClient};

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: AppConfigEnv,
    pub db: DBClient,
}

pub async fn build_app_state()-> Result<AppState, Box<dyn std::error::Error>>{
    let env = AppConfigEnv::from_env()?;
    tracing::info!("Connecting to Database at: {}",&env.database_url);
    
    let db_pool = PgPoolOptions::new().max_connections(env.max_connection).connect(&env.database_url).await?;
    tracing::info!("Database Connected Successfully. Max Connections: {}",&env.max_connection);

    Ok(AppState{env, db: DBClient::new(db_pool)})
}

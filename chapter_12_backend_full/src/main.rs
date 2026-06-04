// ─── Local Modules ────────────────────────────────────────────────────
mod config;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_subscriber::fmt::init;

use crate::config::{AppConfigEnv, conf::build_app_state};

mod db;
mod errors;
// ─── END: =============================================================


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // DX: init tracing scu  with Debug level messages
    tracing_subscriber::fmt().with_max_level(tracing_subscriber::filter::LevelFilter::DEBUG).with_env_filter("info").init(); 
     info!("Starging Appliations.....");


    match build_app_state().await {
        Ok(app_state) => {
            info!("🚀 App State Build Success.");
            info!("Env:{}",app_state.env.app_env);
            info!("Port:{}",app_state.env.app_port);
            info!("Database Connection ready to pool");
        }
        Err(e) =>{
            tracing::error!("Faild to build app state: {}",e);
            std::process::exit(1);
        }
        
    }
    info!("All Sysntem ready server not started.");


    todo!();
}

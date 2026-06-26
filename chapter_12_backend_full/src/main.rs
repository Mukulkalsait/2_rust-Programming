// ─── Local Modules ────────────────────────────────────────────────────
mod config;
use axum::{Router, http::StatusCode, response::IntoResponse, serve::Listener};
use sqlx::postgres::PgPoolOptions;
use tracing::{info, error};
use tracing_subscriber::fmt::init;

use crate::config::{AppConfigEnv, conf::build_app_state};

mod db;
mod errors;
// ─── END: =============================================================


#[tokio::main]
async fn main()  {
    // DX: init tracing scu  with Debug level messages
    tracing_subscriber::fmt().with_max_level(tracing_subscriber::filter::LevelFilter::DEBUG).with_env_filter("info").init(); 
    info!("Starging Appliations.....");
    info!("Sysntem ready server not started.");

    let app_state = match build_app_state().await{ Ok(state) => state, Err(err) => { error!("Faild to build app state!!!"); std::process::exit(1); } };
    let port = app_state.env.app_port;

    info!("🚀 App state built successfully");
    info!("|Environment: {}", app_state.env.app_env);
    info!("|Port: {}", app_state.env.app_port);

    let app: Router = axum::Router::new()
        .route("/health", axum::routing::get(check_health))
        .with_state(app_state);

    let addr = format!("0.0.0.0:{}",port);
    info!("Starting server on: {}",port);

    let app_listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(app_listener, app).await.unwrap();
}

pub async fn check_health()-> impl  IntoResponse{
    StatusCode::OK
}

use std::env;

use axum::{routing, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::{config, db};
use crate::config::AppState;

// eglash
// use crate::handlers::{create_user, google_auth, list_user, me, mobile_login, mobile_register, test};
//
//
//
pub fn build_app(&app_state: AppState) -> axum::Router {

    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    Router::new()
        .route("/home", routing::get(homepageurl))
        // important
        // .route("/auth/google", routing::post(google_auth::google_auth))
        // .route("/me", routing::get(me))
        // .route("/auth/register", routing::post(mobile_register))
        // .route("/auth/phone-login", routing::post(mobile_login))
        // .route("/users", routing::post(create_user))
        // .route("/users", routing::get(list_user))
        // .route("/appx", routing::get(appx))
        // // fail
        // .route("/fail", routing::get(test::fail))
        .layer(cors)
        .with_state(&app_state)
}

async fn homepageurl() -> &'static str {
    "<html><h1>Welcome to homepage</h1></html>"
}
async fn appx() -> &'static str {
    "appx"
}


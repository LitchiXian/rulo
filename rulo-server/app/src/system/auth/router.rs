use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use common::state::AppState;

use crate::system::auth::handler;

pub fn public_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(handler::login_handler))
        .route("/register", post(handler::register_handler))
}

pub fn protected_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/logout", post(handler::logout_handler))
        .route("/info", get(handler::info_handler))
}

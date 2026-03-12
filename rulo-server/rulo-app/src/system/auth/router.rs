use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use rulo_common::state::AppState;

use crate::system::auth::handler;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(handler::login_handler))
        .route("/register", post(handler::register_handler))
        .route("/logout", post(handler::hello_handler))
        .route("/info", get(handler::hello_handler))
}

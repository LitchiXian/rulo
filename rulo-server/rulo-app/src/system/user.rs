use std::sync::Arc;

use axum::{Router, routing::get};

use rulo_common::state::AppState;

pub mod handle;
pub mod model;
mod service;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        // .route("/save", post(handle::user_save_handler))
        // .route("/list", get(handle::user_list_handler))
        .route("/db_list", get(handle::db_user_list_handler))
        .route("/hello", get(handle::hello_handler))
        .route("/hello_error", get(handle::hello_error_handler))
        .route("/hello_redis", get(handle::hello_redis_handler))
}

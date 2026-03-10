use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use rulo_common::state::AppState;

pub mod handle;
pub mod model;
mod service;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/db_list", get(handle::db_user_list_handler))
        .route("/hello", get(handle::hello_handler))
        .route("/hello_error", get(handle::hello_error_handler))
        .route("/hello_redis", get(handle::hello_redis_handler))
        .route("/save", post(handle::save_handle))
        .route("/remove", post(handle::hello_handler))
        .route("/update", post(handle::hello_handler))
        .route("/get", get(handle::hello_handler))
        .route("/list", get(handle::hello_handler))
}

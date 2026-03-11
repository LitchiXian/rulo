use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use rulo_common::state::AppState;

use crate::system::user::handler;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/db_list", get(handler::db_user_list_handler))
        .route("/hello", get(handler::hello_handler))
        .route("/hello_error", get(handler::hello_error_handler))
        .route("/hello_redis", get(handler::hello_redis_handler))
        .route("/save", post(handler::save_handler))
        .route("/remove", post(handler::remove_handler))
        .route("/update", post(handler::update_handler))
        .route("/detail", get(handler::detail_handler))
        .route("/list", get(handler::list_handler))
}

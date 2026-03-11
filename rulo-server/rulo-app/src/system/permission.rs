use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use rulo_common::state::AppState;

pub mod handle;
pub mod model;
pub mod service;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/save", post(handle::save_handle))
        .route("/remove", post(handle::remove_handle))
        .route("/update", post(handle::update_handle))
        .route("/get_one", get(handle::get_one_handler))
        .route("/list", get(handle::list_handler))
}

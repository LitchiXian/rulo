use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use rulo_common::state::AppState;

use crate::system::menu::handler;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/save", post(handler::save_handler))
        .route("/remove", post(handler::remove_handler))
        .route("/update", post(handler::update_handler))
        .route("/detail", get(handler::detail_handler))
        .route("/list", get(handler::list_handler))
}

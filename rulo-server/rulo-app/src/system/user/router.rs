use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use rulo_common::state::AppState;

use crate::system::user::handler;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/save", post(handler::save_handler))
        .route("/remove", post(handler::remove_handler))
        .route("/update", post(handler::update_handler))
        .route("/update-bind-roles", post(handler::update_bind_roles_handler))
        .route("/detail", get(handler::detail_handler))
        .route("/list", get(handler::list_handler))
        .route("/list-bind-roles", get(handler::list_bind_roles_handler))
}

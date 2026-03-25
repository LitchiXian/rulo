use std::sync::Arc;

use axum::{Router, routing::get};
use rulo_common::state::AppState;

use super::handler;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/list", get(handler::list_handler))
}

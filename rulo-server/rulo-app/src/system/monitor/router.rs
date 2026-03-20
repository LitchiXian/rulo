use crate::system::monitor::handler;

use std::sync::Arc;

use axum::{Router, routing::get};
use rulo_common::state::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/server-info", get(handler::server_info_handler))
}

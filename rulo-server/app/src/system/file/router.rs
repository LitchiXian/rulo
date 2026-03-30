use std::sync::Arc;

use axum::{
    Router,
    routing::post,
};
use common::state::AppState;

use crate::system::file::handler;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/upload", post(handler::upload_handler))
}

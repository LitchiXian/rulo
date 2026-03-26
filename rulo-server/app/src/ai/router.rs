use std::sync::Arc;

use axum::Router;
use rulo_common::state::AppState;

use crate::ai::chat;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/chat", axum::routing::post(chat::handler::chat_handler))
        .route(
            "/chat/stream",
            axum::routing::post(chat::handler::chat_stream_handler),
        )
}

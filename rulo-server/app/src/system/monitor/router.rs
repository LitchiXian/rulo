use crate::system::monitor::handler;

use std::sync::Arc;

use axum::{Router, routing::get};
use rulo_common::state::AppState;

/// 需要鉴权的监控路由
pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/server-info", get(handler::server_info_handler))
}

/// 无需鉴权的公开路由（健康检查）
pub fn public_routes() -> Router<Arc<AppState>> {
    Router::new().route("/health", get(handler::health_handler))
}

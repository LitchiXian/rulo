use std::sync::Arc;

use axum::Router;
use rulo_common::state::AppState;

use crate::system;

pub fn routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new().nest("/system", system::router::routes(state))
}

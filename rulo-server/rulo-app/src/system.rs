use std::sync::Arc;

use axum::Router;

use rulo_common::state::AppState;

pub mod user;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().nest("/user", user::routes())
}

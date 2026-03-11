use std::sync::Arc;

use axum::Router;

use rulo_common::state::AppState;

pub mod menu;
pub mod permission;
pub mod role;
pub mod user;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/user", user::routes())
        .nest("/role", role::routes())
        .nest("/menu", menu::routes())
        .nest("/permission", permission::routes())
}

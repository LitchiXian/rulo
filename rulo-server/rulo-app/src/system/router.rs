use std::sync::Arc;

use axum::Router;
use rulo_common::state::AppState;

use crate::system::{auth, file, menu, monitor, permission, role, user};

pub fn public_routes() -> Router<Arc<AppState>> {
    Router::new().nest("/auth", auth::router::public_routes())
}

pub fn protected_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth::router::protected_routes())
        .nest("/user", user::router::routes())
        .nest("/role", role::router::routes())
        .nest("/menu", menu::router::routes())
        .nest("/permission", permission::router::routes())
        .nest("/monitor", monitor::router::routes())
        .nest("/file", file::router::routes())
}

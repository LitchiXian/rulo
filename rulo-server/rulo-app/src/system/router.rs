use std::sync::Arc;

use axum::Router;
use rulo_common::state::AppState;

use crate::system::{auth, menu, permission, role, user};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/user", user::router::routes())
        .nest("/role", role::router::routes())
        .nest("/menu", menu::router::routes())
        .nest("/permission", permission::router::routes())
        .nest("/auth", auth::router::routes())
}

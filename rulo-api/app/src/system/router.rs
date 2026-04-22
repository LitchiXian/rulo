use std::sync::Arc;

use axum::{Router, middleware as axum_mw};
use common::state::AppState;

use crate::middleware::rate_limit;
use crate::system::{audit, auth, dept, file, menu, monitor, permission, role, user};

pub fn public_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .nest(
            "/auth",
            auth::router::public_routes()
                .layer(axum_mw::from_fn_with_state(
                    state.clone(),
                    audit::middleware::audit_log,
                ))
                .layer(axum_mw::from_fn_with_state(
                    state,
                    rate_limit::login_rate_limit,
                )),
        )
        .nest("/monitor", monitor::router::public_routes())
}

pub fn protected_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth::router::protected_routes())
        .nest("/user", user::router::routes())
        .nest("/role", role::router::routes())
        .nest("/menu", menu::router::routes())
        .nest("/permission", permission::router::routes())
        .nest("/dept", dept::router::routes())
        .nest("/monitor", monitor::router::routes())
        .nest("/file", file::router::routes())
        .nest("/audit", audit::router::routes())
}

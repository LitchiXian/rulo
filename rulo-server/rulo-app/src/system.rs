use std::sync::{Arc, Mutex};

use axum::Router;

use rulo_common::state::AppState;

pub mod user;

pub fn routes() -> Router<Arc<Mutex<AppState>>> {
    Router::new().nest("/user", user::routes())
}

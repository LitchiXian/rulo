use std::sync::Arc;

use axum::{Json, extract::State};
use rulo_common::{result::R, state::AppState};

use crate::system::auth::{model::AuthUserDto, service};

pub async fn hello_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<AuthUserDto>,
) -> R<String> {
    service::login(&state.db_pool, &state.redis_pool, &dto).await
}

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<AuthUserDto>,
) -> R<String> {
    service::login(&state.db_pool, &state.redis_pool, &dto).await
}

pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<AuthUserDto>,
) -> R<()> {
    service::register(&state.db_pool, &dto).await
}

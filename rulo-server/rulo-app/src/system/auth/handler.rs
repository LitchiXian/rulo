use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Request, State},
    http::Extensions,
};
use rulo_common::{error::AppError, result::R, state::AppState};
use tracing_subscriber::registry::ExtensionsMut;

use crate::system::{
    auth::{model::AuthUserDto, service},
    user::model::{SysUser, UserId, UserToken},
};

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

pub async fn logout_handler(
    State(state): State<Arc<AppState>>,
    Extension(UserToken(token)): Extension<UserToken>,
) -> R<()> {
    service::logout(&state.redis_pool, &token).await
}

pub async fn info_handler(
    State(state): State<Arc<AppState>>,
    Extension(UserId(user_id)): Extension<UserId>,
) -> R<SysUser> {
    service::info(&state.redis_pool, user_id).await
}

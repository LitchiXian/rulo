use std::sync::Arc;

use axum::{Extension, extract::State};
use rulo_common::{extractor::ValidatedJson, result::R, state::AppState};

use crate::system::auth::{
    model::{AuthUserDto, LoginInfoVo, UserId, UserToken},
    service,
};

#[utoipa::path(
    post, path = "/system/auth/login",
    request_body = AuthUserDto,
    responses((status = 200, description = "login success, returns JWT token", body = String))
)]
pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    ValidatedJson(dto): ValidatedJson<AuthUserDto>,
) -> R<String> {
    service::login(&state.db_pool, &state.redis_pool, &state.jwt_config, &dto).await
}

#[utoipa::path(
    post, path = "/system/auth/register",
    request_body = AuthUserDto,
    responses((status = 200, description = "register success"))
)]
pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    ValidatedJson(dto): ValidatedJson<AuthUserDto>,
) -> R<()> {
    service::register(&state.db_pool, &dto).await
}

#[utoipa::path(
    post, path = "/system/auth/logout",
    responses((status = 200, description = "logout success")),
    security(("bearer_auth" = []))
)]
pub async fn logout_handler(
    State(state): State<Arc<AppState>>,
    Extension(UserToken(token)): Extension<UserToken>,
) -> R<()> {
    service::logout(&state.redis_pool, &token).await
}

#[utoipa::path(
    get, path = "/system/auth/info",
    responses((status = 200, description = "user info", body = LoginInfoVo)),
    security(("bearer_auth" = []))
)]
pub async fn info_handler(
    State(state): State<Arc<AppState>>,
    Extension(UserId(user_id)): Extension<UserId>,
) -> R<LoginInfoVo> {
    service::info(&state.db_pool, &state.redis_pool, user_id).await
}

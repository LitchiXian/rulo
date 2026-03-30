use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    extract::State,
    http::Request,
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
use common::{
    constant::redis_constant,
    error::AppError,
    model::PermCodes,
    state::AppState,
    util::{jwt_util, redis_util},
};

use crate::{
    ai,
    middleware::rate_limit,
    swagger,
    system::{
        self,
        auth::{
            model::{UserId, UserToken},
            service as auth_service,
        },
    },
};

// 顶层路由: 统一管理鉴权, 所有模块的公开/私密路由都在这里聚合
// 新增模块时, 只需在这里 merge, 不用每个模块重复写逻辑鉴权逻辑
pub fn routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    // public API（登录/注册加 IP 限流）
    let public_routes = Router::new()
        .merge(swagger::router::routes())
        .nest("/system", system::router::public_routes(state.clone()));

    // protected API: need authorization
    let protected_routes = Router::new()
        .nest("/system", system::router::protected_routes())
        .nest(
            "/ai",
            ai::router::routes().layer(middleware::from_fn_with_state(
                state.clone(),
                rate_limit::ai_rate_limit,
            )),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            system::audit::middleware::audit_log,
        ))
        .layer(middleware::from_fn_with_state(state.clone(), jwt_auth));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(middleware::from_fn_with_state(
            state,
            rate_limit::global_rate_limit,
        ))
}

async fn jwt_auth(
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let token = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok());

    let token = match token {
        Some(t) => t.to_string(),
        None => return AppError::Unauthorized("未登录, 请先登录".to_string()).into_response(),
    };

    if jwt_util::verify_token(&token, &state.jwt_config.secret).is_err() {
        return AppError::Unauthorized("认证失败, token无效或已过期".to_string()).into_response();
    }

    let redis_key = redis_constant::USER_TOKEN.to_owned() + &token;
    let user_id = match redis_util::get_obj::<i64>(&state.redis_pool, &redis_key).await {
        Ok(Some(user_id)) => user_id,
        Ok(None) => {
            return AppError::Unauthorized("登录已过期, 请重新登录".to_string()).into_response();
        }
        Err(e) => return e.into_response(),
    };

    req.extensions_mut().insert(UserId(user_id));
    req.extensions_mut().insert(UserToken(token));

    let perms =
        match auth_service::get_user_perms_from_cache(&state.db_pool, &state.redis_pool, user_id)
            .await
        {
            Ok(p) => p,
            Err(e) => return e.into_response(),
        };
    req.extensions_mut().insert(PermCodes(perms));

    next.run(req).await
}

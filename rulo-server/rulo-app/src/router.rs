use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    extract::State,
    http::Request,
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
use rulo_common::{
    constant::redis_constant,
    error::AppError,
    state::AppState,
    util::{jwt_util, redis_util},
};

use crate::system;

// 顶层路由: 统一管理鉴权, 所有模块的公开/私密路由都在这里聚合
// 新增模块时, 只需在这里 merge, 不用每个模块重复写逻辑鉴权逻辑
pub fn routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    // public API
    let public_routes = Router::new().nest("/system", system::router::public_routes());

    // protected API: need authorization
    let protected_routes = Router::new()
        .nest("/system", system::router::protected_routes())
        .layer(middleware::from_fn_with_state(state, jwt_auth));

    Router::new().merge(public_routes).merge(protected_routes)
}

async fn jwt_auth(State(state): State<Arc<AppState>>, req: Request<Body>, next: Next) -> Response {
    let token = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok());

    let token = match token {
        Some(t) => t,
        None => return AppError::Unauthorized("未登录, 请先登录".to_string()).into_response(),
    };

    if jwt_util::verify_token(token).is_err() {
        return AppError::Unauthorized("认证失败, token无效或已过期".to_string()).into_response();
    }

    let redis_key = redis_constant::USER_TOKEN.to_owned() + token;
    match redis_util::get_obj::<i64>(&state.redis_pool, &redis_key).await {
        Ok(Some(_)) => {}
        Ok(None) => {
            return AppError::Unauthorized("登录已过期, 请重新登录".to_string()).into_response();
        }
        Err(e) => return e.into_response(),
    }

    next.run(req).await
}

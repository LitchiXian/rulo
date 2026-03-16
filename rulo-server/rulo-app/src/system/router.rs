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

use crate::system::{auth, menu, permission, role, user};

pub fn routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let public_router = Router::new().nest("/auth", auth::router::routes());

    let protected_router = Router::new()
        .nest("/user", user::router::routes())
        .nest("/role", role::router::routes())
        .nest("/menu", menu::router::routes())
        .nest("/permission", permission::router::routes())
        .layer(middleware::from_fn_with_state(state, jwt_auth));

    Router::new().merge(public_router).merge(protected_router)
}

pub async fn jwt_auth(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let token = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok());

    let token = match token {
        Some(t) => t,
        None => return AppError::Unauthorized("未登录，请先登录".to_string()).into_response(),
    };

    if jwt_util::verify_token(token).is_err() {
        return AppError::Unauthorized("认证失败，token无效或已过期".to_string()).into_response();
    }

    let redis_key = redis_constant::USER_TOKEN.to_owned() + token;
    match redis_util::get_obj::<i64>(&state.redis_pool, &redis_key).await {
        Ok(Some(_)) => {}
        Ok(None) => {
            return AppError::Unauthorized("登录已过期,请重新登录".to_string()).into_response();
        }
        Err(e) => return e.into_response(),
    }

    next.run(req).await
}

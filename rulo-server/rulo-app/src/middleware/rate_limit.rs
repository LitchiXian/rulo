use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    body::Body,
    extract::{ConnectInfo, State},
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use deadpool_redis::Pool;
use redis::AsyncCommands;
use rulo_common::{constant::redis_constant, error::AppError, state::AppState};

use crate::system::auth::model::UserId;

/// 从请求中提取客户端 IP
fn extract_ip(req: &Request<Body>) -> String {
    req.headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("").trim().to_string())
        .or_else(|| {
            req.headers()
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
        })
        .or_else(|| {
            req.extensions()
                .get::<ConnectInfo<SocketAddr>>()
                .map(|ci| ci.0.ip().to_string())
        })
        .unwrap_or_else(|| "unknown".to_string())
}

/// Redis 固定窗口计数器：INCR key，首次设置 60s 过期
async fn check_rate_limit(pool: &Pool, key: &str, max_requests: u64) -> Result<(), AppError> {
    let mut conn = pool.get().await?;
    let count: u64 = conn.incr(key, 1u64).await?;
    if count == 1 {
        // 首次请求，设置 60 秒窗口
        let _: () = conn.expire(key, 60).await?;
    }
    if count > max_requests {
        return Err(AppError::TooManyRequests(
            "请求过于频繁，请稍后再试".to_string(),
        ));
    }
    Ok(())
}

/// 登录/注册限流中间件：按 IP 维度，防暴力破解
pub async fn login_rate_limit(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let ip = extract_ip(&req);
    let key = format!("{}login:{}", redis_constant::RATE_LIMIT, ip);
    let max = state.rate_limit_config.login_per_minute;

    if let Err(e) = check_rate_limit(&state.redis_pool, &key, max).await {
        return e.into_response();
    }

    next.run(req).await
}

/// AI 接口限流中间件：按用户 ID 维度，防 token 消耗
pub async fn ai_rate_limit(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Response {
    // 先按 IP 兜底（用户 ID 可能尚未注入）
    let user_key = req
        .extensions()
        .get::<UserId>()
        .map(|u| format!("user:{}", u.0));

    let key = match user_key {
        Some(uk) => format!("{}ai:{}", redis_constant::RATE_LIMIT, uk),
        None => {
            let ip = extract_ip(&req);
            format!("{}ai:ip:{}", redis_constant::RATE_LIMIT, ip)
        }
    };
    let max = state.rate_limit_config.ai_per_minute;

    if let Err(e) = check_rate_limit(&state.redis_pool, &key, max).await {
        return e.into_response();
    }

    next.run(req).await
}

/// 全局限流中间件：按 IP 维度，兜底保护
pub async fn global_rate_limit(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let ip = extract_ip(&req);
    let key = format!("{}global:{}", redis_constant::RATE_LIMIT, ip);
    let max = state.rate_limit_config.global_per_minute;

    if let Err(e) = check_rate_limit(&state.redis_pool, &key, max).await {
        return e.into_response();
    }

    next.run(req).await
}

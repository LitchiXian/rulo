use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use deadpool_redis::Pool;
use redis::AsyncCommands;
use common::{constant::redis_constant, error::AppError, state::AppState, util::ip_util};

use crate::system::auth::model::UserId;

fn now_millis() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as f64
}

// Redis 滑动窗口限流：基于 Sorted Set
// - member = 当前时间戳（毫秒），score = 同一时间戳
// - ZREMRANGEBYSCORE 清除窗口外记录，ZCARD 统计窗口内请求数
async fn check_rate_limit(pool: &Pool, key: &str, max_requests: u64) -> Result<(), AppError> {
    let now = now_millis();
    let window_start = now - 60_000.0; // 60 秒滑动窗口

    let mut conn = pool.get().await?;

    // 清除窗口外的旧记录
    let _: () = conn.zrembyscore(key, f64::NEG_INFINITY, window_start).await?;

    // 统计窗口内请求数
    let count: u64 = conn.zcard(key).await?;

    if count >= max_requests {
        return Err(AppError::TooManyRequests(
            "请求过于频繁，请稍后再试".to_string(),
        ));
    }

    // 添加当前请求记录（member 用纳秒避免同毫秒冲突）
    let member = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .to_string();
    let _: () = conn.zadd(key, &member, now).await?;

    // 设置 key 过期，防止冷 key 堆积
    let _: () = conn.expire(key, 65).await?;

    Ok(())
}

/// 登录/注册限流中间件：按 IP 维度，防暴力破解
pub async fn login_rate_limit(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let ip = ip_util::extract_client_ip(&req).unwrap_or_else(|| "unknown".into());
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
    let user_key = req
        .extensions()
        .get::<UserId>()
        .map(|u| format!("user:{}", u.0));

    let key = match user_key {
        Some(uk) => format!("{}ai:{}", redis_constant::RATE_LIMIT, uk),
        None => {
            let ip = ip_util::extract_client_ip(&req).unwrap_or_else(|| "unknown".into());
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
    let ip = ip_util::extract_client_ip(&req).unwrap_or_else(|| "unknown".into());
    let key = format!("{}global:{}", redis_constant::RATE_LIMIT, ip);
    let max = state.rate_limit_config.global_per_minute;

    if let Err(e) = check_rate_limit(&state.redis_pool, &key, max).await {
        return e.into_response();
    }

    next.run(req).await
}

use std::net::SocketAddr;

use axum::{body::Body, extract::ConnectInfo, http::Request};

/// 从请求中提取客户端 IP
/// 优先级：X-Forwarded-For → X-Real-IP → ConnectInfo 连接地址
pub fn extract_client_ip(req: &Request<Body>) -> Option<String> {
    req.headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("").trim().to_string())
        .filter(|s| !s.is_empty())
        .or_else(|| {
            req.headers()
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        })
        .or_else(|| {
            req.extensions()
                .get::<ConnectInfo<SocketAddr>>()
                .map(|ci| ci.0.ip().to_string())
        })
}

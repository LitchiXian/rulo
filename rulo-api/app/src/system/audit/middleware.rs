use std::sync::Arc;
use std::time::Instant;

use axum::{
    body::{self, Body},
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};
use common::state::AppState;
use common::util::{id_util, ip_util};

use crate::system::auth::model::UserId;

/// 判断是否为敏感操作路径（审计表标记用）
fn is_sensitive_path(path: &str) -> bool {
    let sensitive_patterns = [
        "/remove",
        "/update-bind-roles",
        "/update-bind-menus",
        "/update-bind-perms",
        "/password",
        "/logout",
    ];
    sensitive_patterns.iter().any(|p| path.contains(p))
}

/// 判断是否应隐藏请求参数（仅密码等高度敏感数据）
fn should_hide_params(path: &str) -> bool {
    let hide_patterns = ["/password", "/login", "/register"];
    hide_patterns.iter().any(|p| path.contains(p))
}

/// 判断路径是否应跳过审计（避免记录审计查询本身等）
fn should_skip(path: &str) -> bool {
    let skip_patterns = ["/system/audit/", "/system/monitor/health", "/scalar"];
    skip_patterns.iter().any(|p| path.contains(p))
}

/// 审计日志中间件：记录受保护路由的请求信息
pub async fn audit_log(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    // 跳过不需要审计的路径
    if should_skip(&path) {
        return next.run(req).await;
    }

    // 提取用户信息（由 jwt_auth 中间件注入）
    let user_id = req.extensions().get::<UserId>().map(|u| u.0);

    let is_sensitive = is_sensitive_path(&path);

    // 提取请求参数
    let hide_params = should_hide_params(&path);
    let (params, req) = if hide_params {
        // 密码等高敏感数据不记录参数
        (None, req)
    } else if matches!(method.as_str(), "POST" | "PUT") {
        // POST/PUT: 读取 body
        let (parts, body) = req.into_parts();
        match body::to_bytes(body, 64 * 1024).await {
            Ok(bytes) => {
                let params = String::from_utf8_lossy(&bytes).to_string();
                let params = if params.is_empty() { None } else { Some(params) };
                let req = Request::from_parts(parts, Body::from(bytes.to_vec()));
                (params, req)
            }
            Err(_) => {
                let req = Request::from_parts(parts, Body::empty());
                (None, req)
            }
        }
    } else {
        // GET/DELETE: 记录 query string
        let query = req.uri().query().map(|q| q.to_string()).filter(|q| !q.is_empty());
        (query, req)
    };

    // 获取客户端 IP（优先代理头，回退到连接地址）
    let ip = ip_util::extract_client_ip(&req);

    let start = Instant::now();
    let response = next.run(req).await;
    let duration_ms = start.elapsed().as_millis() as i64;

    let status = response.status().as_u16() as i16;

    // 异步写入审计日志，不阻塞响应
    let pool = state.db_pool.clone();
    tokio::spawn(async move {
        crate::system::audit::service::insert_log(
            &pool,
            id_util::next_id(),
            user_id,
            &method,
            &path,
            params,
            status,
            duration_ms,
            ip,
            is_sensitive,
        )
        .await;
    });

    response
}

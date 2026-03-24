use std::sync::Arc;

use axum::{Router, middleware::from_fn, routing::get};
use tower_http::trace::TraceLayer;
use tracing::info;

use rulo_common::{
    bootstrap,
    config::AppConfig,
    error,
    state::AppState,
};
mod ai;
mod router;
mod swagger;
mod system;

#[tokio::main]
async fn main() {
    // 初始化日志（必须持有 _guard 到 main 结束，否则文件日志丢失）
    let _guard = bootstrap::init_tracing("logs", "app.log");

    // 加载配置
    let cfg = AppConfig::load();
    info!("server config: {:?}", cfg.server);

    // 建立数据库连接池
    let db_pool = bootstrap::connect_db(&cfg.database).await;

    // 建立 Redis 连接池（含 PING 验证）
    let redis_pool = bootstrap::connect_redis(&cfg.redis).await;

    // 建立 S3 对象存储客户端
    let s3_bucket = bootstrap::connect_s3(&cfg.storage).await;

    // 共享状态
    let state = Arc::new(AppState {
        db_pool,
        redis_pool,
        ai_config: cfg.ai.clone(),
        jwt_config: cfg.jwt.clone(),
        storage_config: cfg.storage.clone(),
        s3_bucket,
    });

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .merge(router::routes(state.clone()))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(from_fn(error::log_app_errors));

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        cfg.server.ipaddr, cfg.server.port
    ))
    .await
    .unwrap();

    info!("listener on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

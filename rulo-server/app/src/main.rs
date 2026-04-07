use std::net::SocketAddr;
use std::sync::Arc;

use axum::{Router, middleware::from_fn, routing::get};
use tower_http::trace::TraceLayer;
use tracing::info;

use app::router;
use common::{bootstrap, config::AppConfig, error, state::AppState};

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
        rate_limit_config: cfg.rate_limit.clone(),
    });

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .merge(router::routes(state.clone()))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(from_fn(error::log_app_errors));

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", cfg.server.ipaddr, cfg.server.port))
            .await
            .expect("failed to bind TcpListener, port may already be in use");

    info!(
        "listener on {}",
        listener.local_addr().expect("failed to get local_addr")
    );

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("axum server exited with error");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("shutdown signal received, stopping server gracefully");
}

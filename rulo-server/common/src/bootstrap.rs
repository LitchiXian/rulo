use std::time::Duration;

use deadpool_redis::{Pool as RedisPool, Runtime};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::{DatabaseConfig, RedisConfig, StorageConfig};

/// 初始化日志：同时输出到终端和滚动文件
/// 返回的 WorkerGuard 必须在 main 中持有到程序退出，否则日志会丢失
pub fn init_tracing(log_dir: &str, log_file: &str) -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily(log_dir, log_file);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "rulo=debug,tower_http=debug,axum::rejection=trace"
                    .to_string()
                    .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .init();

    guard
}

/// 建立 PostgreSQL 连接池，并自动执行数据库迁移
pub async fn connect_db(cfg: &DatabaseConfig) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(cfg.max_connections)
        .acquire_timeout(Duration::from_secs(cfg.acquire_timeout_secs))
        .connect(&cfg.url)
        .await
        .expect("无法连接数据库，请检查 [database] 配置");

    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .expect("数据库迁移失败");

    pool
}

/// 建立 Redis 连接池，并立即 PING 验证连通性
pub async fn connect_redis(cfg: &RedisConfig) -> RedisPool {
    let pool = deadpool_redis::Config::from_url(&cfg.url)
        .create_pool(Some(Runtime::Tokio1))
        .expect("无法创建 Redis 连接池，请检查 [redis] 配置");

    // 连接池是延迟连接，立即获取连接并 PING，启动时即暴露问题
    let mut conn = pool.get().await.unwrap_or_else(|e| {
        panic!("Redis 连接失败: {e}");
    });
    let _pong: String = redis::cmd("PING")
        .query_async(&mut conn)
        .await
        .expect("Redis PING 失败");

    pool
}

/// 创建 S3 Bucket 客户端并验证连通性
pub async fn connect_s3(cfg: &StorageConfig) -> Box<s3::Bucket> {
    let bucket = crate::util::storage_util::create_s3_bucket(cfg);

    match crate::util::storage_util::ensure_bucket_exists(cfg).await {
        Ok(true) => {
            tracing::info!("S3 Bucket 不存在，已自动创建: {}", cfg.bucket);
        }
        Ok(false) => {
            tracing::info!("S3 Bucket 已存在: {}", cfg.bucket);
        }
        Err(e) => {
            tracing::warn!("S3 Bucket 自动创建检查失败: {e}");
        }
    }

    bucket
}

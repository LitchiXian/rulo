use std::{sync::Arc, time::Duration};

use axum::{Router, middleware::from_fn, routing::get};
use config::Config;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rulo_common::{error, state::AppState};
mod system;

#[derive(Debug, Deserialize)]
struct ServerConfig {
    ipaddr: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    server: ServerConfig,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // 记录日志到本地
    let file_appender = tracing_appender::rolling::daily("logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 日志输出配置
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                    "rulo=debug,tower_http=debug,axum::rejection=trace".to_string()
                .into()
            }),
        )
        // 输出到terminal
        .with(tracing_subscriber::fmt::layer())
        // 输出到文件
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .init();

    let app_config = Config::builder()
        .add_source(config::File::with_name("config/default"))
        .build()
        .unwrap()
        .try_deserialize::<AppConfig>()
        .unwrap();
    println!("{:?}", app_config);

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://l2:123456@127.0.0.1:5432/l2_db".to_string());

    info!("db_connection_str: {db_connection_str}");

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // 共享状态配置
    let state = Arc::new(AppState {
        db_pool: pool,
        // users: HashMap::new(),
        // next_id: 1,
    });

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/system", system::routes())
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(from_fn(error::log_app_errors));

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        app_config.server.ipaddr, app_config.server.port
    ))
    .await
    .unwrap();

    println!("listener on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

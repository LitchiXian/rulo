// 模块声明
mod handlers;
mod models;
mod services;
mod utils;

use std::net::SocketAddr;
use utils::db::init_db_pool;

#[tokio::main]
async fn main() {
    // 初始化日志
    env_logger::init();

    // 初始化数据库连接池
    log::info!("正在连接数据库...");
    let db_pool = match init_db_pool().await {
        Ok(pool) => {
            log::info!("数据库连接成功");
            pool
        }
        Err(e) => {
            log::error!("数据库连接失败: {}", e);
            std::process::exit(1);
        }
    };

    // 创建路由
    let app = utils::routes::create_routes(db_pool);

    // 设置监听地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    log::info!("服务启动，监听地址: {}", addr);

    // 启动服务器
    axum::serve(
        tokio::net::TcpListener::bind(addr)
            .await
            .expect("地址绑定失败"),
        app,
    )
    .await
    .expect("服务器启动失败");
}

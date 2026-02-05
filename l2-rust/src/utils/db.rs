use sqlx::PgPool;

// 初始化数据库连接池
pub async fn init_db_pool() -> Result<PgPool, sqlx::Error> {
    // 从环境变量获取数据库连接信息
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://l2:123456@localhost/l2_db".to_string()
    });

    log::info!("正在连接数据库: {}", database_url);
    let pool = PgPool::connect(&database_url).await?;

    // 验证连接
    sqlx::query!("SELECT 1 as one").fetch_one(&pool).await?;

    Ok(pool)
}

// 数据库操作的结果类型
#[allow(dead_code)]
pub type DbResult<T> = Result<T, sqlx::Error>;

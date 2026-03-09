use deadpool_redis::Pool as RedisPool;
use sqlx::{Pool, Postgres};

pub struct AppState {
    pub db_pool: Pool<Postgres>,
    pub redis_pool: RedisPool,
}

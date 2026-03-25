use deadpool_redis::Pool as RedisPool;
use s3::Bucket;
use sqlx::{Pool, Postgres};

use crate::config::{AiConfig, JwtConfig, RateLimitConfig, StorageConfig};

pub struct AppState {
    pub db_pool: Pool<Postgres>,
    pub redis_pool: RedisPool,
    pub ai_config: AiConfig,
    pub jwt_config: JwtConfig,
    pub storage_config: StorageConfig,
    pub s3_bucket: Box<Bucket>,
    pub rate_limit_config: RateLimitConfig,
}

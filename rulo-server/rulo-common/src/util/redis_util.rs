use std::fs::exists;

use deadpool_redis::Pool;
use redis::{AsyncCommands, ExistenceCheck};
use serde::{Serialize, de::DeserializeOwned};

use crate::error::AppError;

// 存储字符串, seconds 为 0 则永久有效
pub async fn set_str(pool: &Pool, key: &str, value: &str, seconds: u64) -> Result<(), AppError> {
    let mut conn = pool.get().await?;
    if seconds > 0 {
        let _: () = conn.set_ex(key, value, seconds).await?;
    } else {
        let _: () = conn.set(key, value).await?;
    }
    Ok(())
}

// 获取字符串,键不存在时返回 None
pub async fn get_str(pool: &Pool, key: &str) -> Result<Option<String>, AppError> {
    let mut conn = pool.get().await?;
    let val: Option<String> = conn.get(key).await?;
    Ok(val)
}

// 序列化结构体为 JSON 后存入 Redis, seconds 为 0 则永久有效
pub async fn set_obj<T: Serialize>(
    pool: &Pool,
    key: &str,
    value: &T,
    seconds: u64,
) -> Result<(), AppError> {
    let json = serde_json::to_string(value)
        .map_err(|e| AppError::Internal(format!("redis serialize error:{e}")))?;
    set_str(pool, key, &json, seconds).await
}

// 从 Redis 获取 JSON 并反序列化,键不存在时返回 None
pub async fn get_obj<T: DeserializeOwned>(pool: &Pool, key: &str) -> Result<Option<T>, AppError> {
    match get_str(pool, key).await? {
        Some(json) => {
            let val = serde_json::from_str::<T>(&json)
                .map_err(|e| AppError::Internal(format!("redis deserialize error:{e}")))?;
            Ok(Some(val))
        }
        None => Ok(None),
    }
}

// 删除一个或多个键
pub async fn del(pool: &Pool, key: &str) -> Result<bool, AppError> {
    let mut conn = pool.get().await?;
    let exists: bool = conn.exists(key).await?;
    Ok(exists)
}

// 设置键的过期时间(秒)
pub async fn expire(pool: &Pool, key: &str, seconds: u64) -> Result<(), AppError> {
    let mut conn = pool.get().await?;
    let _: bool = conn.expire(key, seconds as i64).await?;
    Ok(())
}

// 获取键的剩余过期时间(秒), -1 表示永久, -2 表示键不存在
pub async fn ttl(pool: &Pool, key: &str) -> Result<i64, AppError> {
    let mut conn = pool.get().await?;
    let ttl: i64 = conn.ttl(key).await?;
    Ok(ttl)
}

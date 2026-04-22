//! 鉴权相关 Redis 缓存的 key 构造与失效统一入口。
//!
//! 设计目标：
//! - 集中管理 USER_AUTH / USER_MENUS / USER_INFO 三类 key 的拼接，避免散落在各 handler 中。
//! - 提供常见的失效粒度（单用户、单角色下所有用户、全体已绑定用户），handler 只需声明意图。

use common::{constant::redis_constant, util::redis_util};
use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;

#[inline]
pub fn user_auth_key(user_id: i64) -> String {
    redis_constant::USER_AUTH.to_owned() + &user_id.to_string()
}

#[inline]
pub fn user_menus_key(user_id: i64) -> String {
    redis_constant::USER_MENUS.to_owned() + &user_id.to_string()
}

#[inline]
pub fn user_info_key(user_id: i64) -> String {
    redis_constant::USER_INFO.to_owned() + &user_id.to_string()
}

/// 仅清除单用户的基础信息缓存（修改用户资料时使用）
pub async fn invalidate_user_info(redis: &RedisPool, user_id: i64) {
    let _ = redis_util::del(redis, &user_info_key(user_id)).await;
}

/// 清除单用户的鉴权上下文 + 菜单缓存（角色绑定变化时使用）
pub async fn invalidate_user_authz(redis: &RedisPool, user_id: i64) {
    let _ = redis_util::del(redis, &user_auth_key(user_id)).await;
    let _ = redis_util::del(redis, &user_menus_key(user_id)).await;
}

/// 清除单用户的全部缓存（删除/冻结用户时使用）
pub async fn invalidate_user_full(redis: &RedisPool, user_id: i64) {
    let _ = redis_util::del(redis, &user_auth_key(user_id)).await;
    let _ = redis_util::del(redis, &user_menus_key(user_id)).await;
    let _ = redis_util::del(redis, &user_info_key(user_id)).await;
}

/// 清除指定角色下所有用户的鉴权上下文 + 菜单缓存
/// （角色 update / remove / 绑菜单 / 绑权限 时使用）
pub async fn invalidate_role_users_authz(redis: &RedisPool, db: &PgPool, role_id: i64) {
    if let Ok(ids) = sqlx::query_scalar!(
        "SELECT user_id FROM sys_user_role WHERE role_id = $1",
        role_id
    )
    .fetch_all(db)
    .await
    {
        for uid in ids {
            invalidate_user_authz(redis, uid).await;
        }
    }
}

/// 清除所有已绑定角色的用户的鉴权上下文 + 菜单缓存
/// （权限定义变更时使用，影响面广）
pub async fn invalidate_all_users_authz(redis: &RedisPool, db: &PgPool) {
    if let Ok(ids) = sqlx::query_scalar!("SELECT DISTINCT user_id FROM sys_user_role")
        .fetch_all(db)
        .await
    {
        for uid in ids {
            invalidate_user_authz(redis, uid).await;
        }
    }
}

/// 清除所有已绑定角色的用户的菜单缓存（菜单结构变更时使用）
pub async fn invalidate_all_users_menus(redis: &RedisPool, db: &PgPool) {
    if let Ok(ids) = sqlx::query_scalar!("SELECT DISTINCT user_id FROM sys_user_role")
        .fetch_all(db)
        .await
    {
        for uid in ids {
            let _ = redis_util::del(redis, &user_menus_key(uid)).await;
        }
    }
}
